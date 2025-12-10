use crate::context::trailing_commas::FormatTrailingCommas;
use crate::prelude::*;

use crate::rules::bindings::parameters::has_only_simple_parameters;
use crate::rules::declarations::function_declaration::FormatFunctionOptions;
use crate::rules::lists::array_element_list::can_concisely_print_array_list;
use crate::utils::function_body::FunctionBodyCacheMode;
use crate::utils::member_chain::SimpleArgument;
use crate::utils::{is_long_curried_call, write_arguments_multi_line};
use biome_formatter::{VecBuffer, format_args, format_element, write};
use biome_rowan::{AstSeparatedElement, AstSeparatedList, SyntaxResult};
use mlang_syntax::{
    AnyMCallArgument, AnyMExpression, MBinaryExpressionFields, MCallArgumentList, MCallArguments,
    MCallArgumentsFields, MCallExpression, MFunctionExpression, MLanguage,
    MLogicalExpressionFields, MLongStringLiteralExpression, MStringLiteralExpression,
    MSyntaxKind::{M_LONG_STRING_LITERAL, M_STRING_LITERAL},
};

use super::string_expression::FormatStringLiteralOptions;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMCallArguments;
impl_format_with_rule!(MCallArguments, FormatMCallArguments);

impl FormatNodeRule<MCallArguments> for FormatMCallArguments {
    fn fmt_fields(&self, node: &MCallArguments, f: &mut MFormatter) -> FormatResult<()> {
        let MCallArgumentsFields {
            l_paren_token,
            args,
            r_paren_token,
        } = node.as_fields();

        if args.is_empty() {
            return write!(
                f,
                [
                    l_paren_token.format(),
                    format_dangling_comments(node.syntax()).with_soft_block_indent(),
                    r_paren_token.format()
                ]
            );
        }

        let call_expression = node.parent::<MCallExpression>();
        let query_like_call = is_query_like_call(call_expression.as_ref());

        let last_index = args.len().saturating_sub(1);
        let mut has_empty_line = false;
        let mut first_is_string = false;

        let arguments: Vec<_> = args
            .elements()
            .enumerate()
            .map(|(index, element)| {
                let leading_lines = element
                    .node()
                    .map_or(0, |node| get_lines_before(node.syntax()));
                has_empty_line = has_empty_line || leading_lines > 1;
                if index == 0
                    && let Ok(node) = element.node() {
                        first_is_string = MLongStringLiteralExpression::cast_ref(node.syntax())
                            .is_some()
                            | MStringLiteralExpression::cast_ref(node.syntax()).is_some()
                    }

                FormatCallArgument::Default {
                    element,
                    is_last: index == last_index,
                    leading_lines,
                    query_like_string: index == 0 && first_is_string && query_like_call,
                }
            })
            .collect();

        if first_is_string && is_query_like_call(call_expression.as_ref()) {
            return format_verbatim_node(node.syntax()).fmt(f);
            // return write!(
            //     f,
            //     [FormatQueryLikeArguments {
            //         l_paren: &l_paren_token.format(),
            //         args: &arguments,
            //         r_paren: &r_paren_token.format(),
            //     }]
            // );
        }

        if has_empty_line || is_function_composition_args(node) {
            return write!(
                f,
                [FormatAllArgsBrokenOut {
                    l_paren: &l_paren_token.format(),
                    args: &arguments,
                    r_paren: &r_paren_token.format(),
                    expand: true,
                }]
            );
        }

        if let Some(group_layout) = arguments_grouped_layout(&args, f.comments()) {
            write_grouped_arguments(node, arguments, group_layout, f)
        } else if is_long_curried_call(call_expression.as_ref()) {
            write!(
                f,
                [
                    l_paren_token.format(),
                    soft_block_indent(&format_once(|f| {
                        write_arguments_multi_line(arguments.iter(), f)
                    })),
                    r_paren_token.format(),
                ]
            )
        } else {
            write!(
                f,
                [FormatAllArgsBrokenOut {
                    l_paren: &l_paren_token.format(),
                    args: &arguments,
                    r_paren: &r_paren_token.format(),
                    expand: false,
                }]
            )
        }
    }

    fn fmt_dangling_comments(&self, _: &MCallArguments, _: &mut MFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}

/// Helper for formatting a call argument
enum FormatCallArgument {
    /// Argument that has not been inspected if its formatted content breaks.
    Default {
        element: AstSeparatedElement<MLanguage, AnyMCallArgument>,

        /// Whether this is the last element.
        is_last: bool,

        /// The number of lines before this node
        leading_lines: usize,

        /// query like
        query_like_string: bool,
    },

    /// The argument has been formatted because a caller inspected if it [Self::will_break].
    ///
    /// Allows to re-use the formatted output rather than having to call into the formatting again.
    Inspected {
        /// The formatted element
        content: FormatResult<Option<FormatElement>>,

        /// The separated element
        element: AstSeparatedElement<MLanguage, AnyMCallArgument>,

        /// The lines before this element
        leading_lines: usize,
    },
}

impl FormatCallArgument {
    /// Returns `true` if this argument contains any content that forces a group to [`break`](FormatElements::will_break).
    fn will_break(&mut self, f: &mut MFormatter) -> bool {
        match &self {
            FormatCallArgument::Default {
                element,
                leading_lines,
                ..
            } => {
                let interned = f.intern(&self);

                let breaks = match &interned {
                    Ok(Some(element)) => element.will_break(),
                    _ => false,
                };

                *self = FormatCallArgument::Inspected {
                    content: interned,
                    element: element.clone(),
                    leading_lines: *leading_lines,
                };
                breaks
            }
            FormatCallArgument::Inspected {
                content: Ok(Some(result)),
                ..
            } => result.will_break(),
            FormatCallArgument::Inspected { .. } => false,
        }
    }

    /// Formats the node of this argument and caches the function body.
    ///
    /// See [MFormatContext::cached_function_body]
    ///
    /// # Panics
    ///
    /// If [`cache_function_body`](Self::cache_function_body) or [`will_break`](Self::will_break) has been called on this argument before.
    fn cache_function_body(&mut self, f: &mut MFormatter) {
        match &self {
            FormatCallArgument::Default {
                element,
                leading_lines,
                ..
            } => {
                let interned = f.intern(&format_once(|f| {
                    self.fmt_with_cache_mode(FunctionBodyCacheMode::Cache, f)?;
                    Ok(())
                }));

                *self = FormatCallArgument::Inspected {
                    content: interned,
                    element: element.clone(),
                    leading_lines: *leading_lines,
                };
            }
            FormatCallArgument::Inspected { .. } => {
                panic!("`cache` must be called before inspecting or formatting the element.");
            }
        }
    }

    fn fmt_with_cache_mode(
        &self,
        cache_mode: FunctionBodyCacheMode,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        match self {
            // Re-use the cached formatted output if there is any.
            FormatCallArgument::Inspected { content, .. } => match content.clone()? {
                Some(element) => {
                    f.write_element(element)?;
                    Ok(())
                }
                None => Ok(()),
            },
            FormatCallArgument::Default {
                element,
                is_last,
                query_like_string,
                ..
            } => {
                match element.node()? {
                    AnyMCallArgument::AnyMExpression(AnyMExpression::MFunctionExpression(
                        function,
                    )) => {
                        write!(
                            f,
                            [function.format().with_options(FormatFunctionOptions {
                                body_cache_mode: cache_mode,
                                ..FormatFunctionOptions::default()
                            })]
                        )?;
                    }
                    AnyMCallArgument::AnyMExpression(AnyMExpression::AnyMLiteralExpression(
                        literal,
                    )) if matches!(
                        literal.value_token()?.kind(),
                        M_LONG_STRING_LITERAL | M_STRING_LITERAL
                    ) =>
                    {
                        write!(
                            f,
                            [literal.format().with_options(FormatStringLiteralOptions {
                                is_query_like_string: *query_like_string
                            })]
                        )?
                    }
                    node => write!(f, [node.format()])?,
                }

                if let Some(separator) = element.trailing_separator()? {
                    if *is_last {
                        write!(f, [format_removed(separator)])
                    } else {
                        write!(f, [separator.format()])
                    }
                } else if !is_last {
                    Err(FormatError::SyntaxError)
                } else {
                    Ok(())
                }
            }
        }
    }

    /// Returns the number of leading lines before the argument's node
    fn leading_lines(&self) -> usize {
        match self {
            FormatCallArgument::Default { leading_lines, .. } => *leading_lines,
            FormatCallArgument::Inspected { leading_lines, .. } => *leading_lines,
        }
    }

    /// Returns the [`separated element`](AstSeparatedElement) of this argument.
    fn element(&self) -> &AstSeparatedElement<MLanguage, AnyMCallArgument> {
        match self {
            FormatCallArgument::Default { element, .. } => element,
            FormatCallArgument::Inspected { element, .. } => element,
        }
    }
}

impl Format<MFormatContext> for FormatCallArgument {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        self.fmt_with_cache_mode(FunctionBodyCacheMode::default(), f)?;
        Ok(())
    }
}

/// Writes the function arguments, and groups the first or last argument depending on `group_layout`.
fn write_grouped_arguments(
    call_arguments: &MCallArguments,
    mut arguments: Vec<FormatCallArgument>,
    group_layout: GroupedCallArgumentLayout,
    f: &mut MFormatter,
) -> FormatResult<()> {
    let l_paren_token = call_arguments.l_paren_token();
    let r_paren_token = call_arguments.r_paren_token();

    let grouped_breaks = {
        let (grouped_arg, other_args) = match group_layout {
            GroupedCallArgumentLayout::GroupedFirstArgument => {
                let (first, tail) = arguments.split_at_mut(1);
                (&mut first[0], tail)
            }
            GroupedCallArgumentLayout::GroupedLastArgument => {
                let end_index = arguments.len().saturating_sub(1);
                let (head, last) = arguments.split_at_mut(end_index);
                (&mut last[0], head)
            }
        };

        let non_grouped_breaks = other_args.iter_mut().any(|arg| arg.will_break(f));

        // if any of the not grouped elements break, then fall back to the variant where
        // all arguments are printed in expanded mode.
        if non_grouped_breaks {
            return write!(
                f,
                [FormatAllArgsBrokenOut {
                    l_paren: &l_paren_token.format(),
                    args: &arguments,
                    r_paren: &r_paren_token.format(),
                    expand: true,
                }]
            );
        }

        match grouped_arg.element().node()? {
            AnyMCallArgument::AnyMExpression(AnyMExpression::MFunctionExpression(function))
                if !other_args.is_empty() || function_has_only_simple_parameters(function) =>
            {
                grouped_arg.cache_function_body(f);
            }
            _ => {
                // Node doesn't have a function body or its a function that doesn't get re-formatted.
            }
        }

        grouped_arg.will_break(f)
    };

    // We now cache them the delimiters tokens. This is needed because `[biome_formatter::best_fitting]` will try to
    // print each version first
    // tokens on the left
    let l_paren = l_paren_token.format().memoized();

    // tokens on the right
    let r_paren = r_paren_token.format().memoized();

    // First write the most expanded variant because it needs `arguments`.
    let most_expanded = {
        let mut buffer = VecBuffer::new(f.state_mut());
        buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;

        write!(
            buffer,
            [FormatAllArgsBrokenOut {
                l_paren: &l_paren,
                args: &arguments,
                r_paren: &r_paren,
                expand: true,
            }]
        )?;
        buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;

        buffer.into_vec()
    };

    // Now reformat the first or last argument if they happen to be a function or arrow function expression.
    // Function and arrow function expression apply a custom formatting that removes soft line breaks from the parameters,
    // type parameters, and return type annotation.
    //
    // This implementation caches the function body of the "normal" formatted function or arrow function expression
    // to avoid quadratic complexity if the functions' body contains another call expression with an arrow or function expression
    // as first or last argument.
    let last_index = arguments.len() - 1;
    let grouped = arguments
        .into_iter()
        .enumerate()
        .map(|(index, argument)| {
            let layout = match group_layout {
                GroupedCallArgumentLayout::GroupedFirstArgument if index == 0 => {
                    Some(GroupedCallArgumentLayout::GroupedFirstArgument)
                }
                GroupedCallArgumentLayout::GroupedLastArgument if index == last_index => {
                    Some(GroupedCallArgumentLayout::GroupedLastArgument)
                }
                _ => None,
            };

            FormatGroupedArgument {
                argument,
                single_argument_list: last_index == 0,
                layout,
            }
            .memoized()
        })
        .collect::<Vec<_>>();

    // Write the most flat variant with the first or last argument grouped.
    let most_flat = {
        let snapshot = f.state_snapshot();
        let mut buffer = VecBuffer::new(f.state_mut());
        buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;

        let result = write!(
            buffer,
            [
                l_paren,
                format_with(|f| {
                    f.join_with(soft_line_break_or_space())
                        .entries(grouped.iter())
                        .finish()
                }),
                r_paren
            ]
        );

        // Turns out, using the grouped layout isn't a good fit because some parameters of the
        // grouped function or arrow expression break. In that case, fall back to the all args expanded
        // formatting.
        // This back tracking is required because testing if the grouped argument breaks would also return `true`
        // if any content of the function body breaks. But, as far as this is concerned, it's only interested if
        // any content in the signature breaks.
        if matches!(result, Err(FormatError::PoorLayout)) {
            drop(buffer);
            f.restore_state_snapshot(snapshot);

            let mut most_expanded_iter = most_expanded.into_iter();
            // Skip over the Start/EndEntry items.
            most_expanded_iter.next();
            most_expanded_iter.next_back();

            return f.write_elements(most_expanded_iter);
        }

        buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;

        buffer.into_vec().into_boxed_slice()
    };

    // Write the second variant that forces the group of the first/last argument to expand.
    let middle_variant = {
        let mut buffer = VecBuffer::new(f.state_mut());

        buffer.write_element(FormatElement::Tag(Tag::StartEntry))?;

        write!(
            buffer,
            [
                l_paren,
                format_with(|f| {
                    let mut joiner = f.join_with(soft_line_break_or_space());

                    match group_layout {
                        GroupedCallArgumentLayout::GroupedFirstArgument => {
                            joiner.entry(&group(&grouped[0]).should_expand(true));
                            joiner.entries(&grouped[1..]).finish()
                        }
                        GroupedCallArgumentLayout::GroupedLastArgument => {
                            let last_index = grouped.len() - 1;
                            joiner.entries(&grouped[..last_index]);
                            joiner
                                .entry(&group(&grouped[last_index]).should_expand(true))
                                .finish()
                        }
                    }
                }),
                r_paren
            ]
        )?;

        buffer.write_element(FormatElement::Tag(Tag::EndEntry))?;

        buffer.into_vec().into_boxed_slice()
    };

    // If the grouped content breaks, then we can skip the most_flat variant,
    // since we already know that it won't be fitting on a single line.
    let variants = if grouped_breaks {
        write!(f, [expand_parent()])?;
        vec![middle_variant, most_expanded.into_boxed_slice()]
    } else {
        vec![most_flat, middle_variant, most_expanded.into_boxed_slice()]
    };

    // SAFETY: Safe because variants is guaranteed to contain exactly 3 entries:
    // * most flat
    // * middle
    // * most expanded
    // ... and best fitting only requires the most flat/and expanded.
    unsafe {
        f.write_element(FormatElement::BestFitting(
            format_element::BestFittingElement::from_vec_unchecked(variants),
        ))
    }
}

/// Helper for formatting the first grouped argument (see [should_group_first_argument]).
struct FormatGroupedFirstArgument<'a> {
    argument: &'a FormatCallArgument,
}

impl Format<MFormatContext> for FormatGroupedFirstArgument<'_> {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        let _element = self.argument.element().node()?;

        // For all other nodes, use the normal formatting (which already has been cached)
        self.argument.fmt(f)
    }
}

/// Helper for formatting the last grouped argument (see [should_group_last_argument]).
struct FormatGroupedLastArgument<'a> {
    argument: &'a FormatCallArgument,
    /// Is this the only argument in the arguments list
    is_only: bool,
}

impl Format<MFormatContext> for FormatGroupedLastArgument<'_> {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        use AnyMExpression::*;
        let element = self.argument.element();

        // For function and arrow expressions, re-format the node and pass the argument that it is the
        // last grouped argument. This changes the formatting of parameters, type parameters, and return types
        // to remove any soft line breaks.
        match element.node()? {
            AnyMCallArgument::AnyMExpression(MFunctionExpression(function))
                if !self.is_only || function_has_only_simple_parameters(function) =>
            {
                with_token_tracking_disabled(f, |f| {
                    write!(
                        f,
                        [function.format().with_options(FormatFunctionOptions {
                            body_cache_mode: FunctionBodyCacheMode::Cached,
                            call_argument_layout: Some(
                                GroupedCallArgumentLayout::GroupedLastArgument
                            ),
                        })]
                    )?;

                    if let Some(separator) = element.trailing_separator()? {
                        write!(f, [format_removed(separator)])?;
                    }

                    Ok(())
                })
            }

            _ => self.argument.fmt(f),
        }
    }
}

/// Disable the token tracking because it is necessary to format function/arrow expressions slightly different.
fn with_token_tracking_disabled<F: FnOnce(&mut MFormatter) -> R, R>(
    f: &mut MFormatter,
    callback: F,
) -> R {
    let was_disabled = f.state().is_token_tracking_disabled();
    f.state_mut().set_token_tracking_disabled(true);

    let result = callback(f);

    f.state_mut().set_token_tracking_disabled(was_disabled);

    result
}

fn function_has_only_simple_parameters(expression: &MFunctionExpression) -> bool {
    expression
        .parameters()
        .map_or(true, |parameters| has_only_simple_parameters(&parameters))
}

/// Helper for formatting a grouped call argument (see [should_group_first_argument] and [should_group_last_argument]).
struct FormatGroupedArgument {
    argument: FormatCallArgument,

    /// Whether this argument is the only argument in the argument list.
    single_argument_list: bool,

    /// The layout to use for this argument.
    layout: Option<GroupedCallArgumentLayout>,
}

impl Format<MFormatContext> for FormatGroupedArgument {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        match self.layout {
            Some(GroupedCallArgumentLayout::GroupedFirstArgument) => FormatGroupedFirstArgument {
                argument: &self.argument,
            }
            .fmt(f),
            Some(GroupedCallArgumentLayout::GroupedLastArgument) => FormatGroupedLastArgument {
                argument: &self.argument,
                is_only: self.single_argument_list,
            }
            .fmt(f),
            None => self.argument.fmt(f),
        }
    }
}

struct FormatAllArgsBrokenOut<'a> {
    l_paren: &'a dyn Format<MFormatContext>,
    args: &'a [FormatCallArgument],
    r_paren: &'a dyn Format<MFormatContext>,
    expand: bool,
}

impl Format<MFormatContext> for FormatAllArgsBrokenOut<'_> {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        write!(
            f,
            [group(&format_args![
                self.l_paren,
                soft_block_indent(&format_with(|f| {
                    for (index, entry) in self.args.iter().enumerate() {
                        if index > 0 {
                            match entry.leading_lines() {
                                0 | 1 => write!(f, [soft_line_break_or_space()])?,
                                _ => write!(f, [empty_line()])?,
                            }
                        }

                        write!(f, [entry])?;
                    }

                    write!(f, [FormatTrailingCommas::All])?;

                    Ok(())
                })),
                self.r_paren,
            ])
            .should_expand(self.expand)]
        )
    }
}

#[derive(Copy, Clone, Debug)]
pub enum GroupedCallArgumentLayout {
    /// Group the first call argument.
    GroupedFirstArgument,

    /// Group the last call argument.
    GroupedLastArgument,
}

fn arguments_grouped_layout(
    args: &MCallArgumentList,
    comments: &MComments,
) -> Option<GroupedCallArgumentLayout> {
    if should_group_first_argument(args, comments).unwrap_or(false) {
        Some(GroupedCallArgumentLayout::GroupedFirstArgument)
    } else if should_group_last_argument(args, comments).unwrap_or(false) {
        Some(GroupedCallArgumentLayout::GroupedLastArgument)
    } else {
        None
    }
}

/// Checks if the first argument requires grouping
fn should_group_first_argument(
    list: &MCallArgumentList,
    comments: &MComments,
) -> SyntaxResult<bool> {
    use AnyMExpression::*;

    let mut iter = list.iter();
    match (iter.next(), iter.next()) {
        (
            Some(Ok(AnyMCallArgument::AnyMExpression(first))),
            Some(Ok(AnyMCallArgument::AnyMExpression(second))),
        ) if iter.next().is_none() => {
            match &first {
                MFunctionExpression(_) => {}
                _ => return Ok(false),
            };

            if matches!(second, MFunctionExpression(_) | MConditionalExpression(_)) {
                return Ok(false);
            }

            Ok(!comments.has_comments(first.syntax())
                && !can_group_expression_argument(&second, comments)?
                && is_relatively_short_argument(second))
        }
        _ => Ok(false),
    }
}

/// Checks if the last argument should be grouped.
fn should_group_last_argument(
    list: &MCallArgumentList,
    comments: &MComments,
) -> SyntaxResult<bool> {
    use AnyMExpression::*;

    let mut iter = list.iter();
    let last = iter.next_back();

    match last {
        Some(Ok(AnyMCallArgument::AnyMExpression(last))) => {
            if comments.has_leading_comments(last.syntax())
                || comments.has_trailing_comments(last.syntax())
            {
                return Ok(false);
            }

            if !can_group_expression_argument(&last, comments)? {
                return Ok(false);
            }

            let penultimate = iter.next_back();

            if let Some(Ok(penultimate)) = &penultimate
                && penultimate.syntax().kind() == last.syntax().kind() {
                    return Ok(false);
                }

            match last {
                MArrayExpression(array) if list.len() > 1 => {
                    if can_concisely_print_array_list(&array.elements(), comments) {
                        return Ok(false);
                    }

                    Ok(true)
                }
                _ => Ok(true),
            }
        }
        _ => Ok(false),
    }
}

/// Checks if `argument` is "short" enough to be groupable. This aims to be
/// logically similar to Prettier's [`isHopefullyShortCallArgument`](https://github.com/prettier/prettier/blob/093745f0ec429d3db47c1edd823357e0ef24e226/src/language-M/print/call-arguments.M#L279),
fn is_relatively_short_argument(argument: AnyMExpression) -> bool {
    match argument {
        AnyMExpression::MBinaryExpression(binary_expression) => {
            if let MBinaryExpressionFields {
                left: Ok(left),
                operator_token: _,
                right: Ok(right),
            } = binary_expression.as_fields()
            {
                SimpleArgument::from(left).is_simple() && SimpleArgument::from(right).is_simple()
            } else {
                false
            }
        }
        AnyMExpression::MLogicalExpression(logical_expression) => {
            if let MLogicalExpressionFields {
                left: Ok(left),
                operator_token: _,
                right: Ok(right),
            } = logical_expression.as_fields()
            {
                SimpleArgument::from(left).is_simple() && SimpleArgument::from(right).is_simple()
            } else {
                false
            }
        }
        AnyMExpression::MCallExpression(call) => {
            if let Ok(arguments) = call.arguments() {
                match arguments.args().len() {
                    0 => true,
                    1 => SimpleArgument::from(AnyMExpression::from(call)).is_simple(),
                    _ => false,
                }
            } else {
                true
            }
        }
        _ => SimpleArgument::from(argument).is_simple(),
    }
}

/// Checks if `argument` benefits from grouping in call arguments.
fn can_group_expression_argument(
    argument: &AnyMExpression,
    comments: &MComments,
) -> SyntaxResult<bool> {
    use AnyMExpression::*;

    let result = match argument {
        MObjectExpression(object_expression) => {
            !object_expression.members().is_empty()
                || comments.has_comments(object_expression.syntax())
        }

        MArrayExpression(array_expression) => {
            !array_expression.elements().is_empty()
                || comments.has_comments(array_expression.syntax())
        }

        MFunctionExpression(_) => true,
        _ => false,
    };

    Ok(result)
}

/// Tests if a call has multiple anonymous function like (arrow or function expression) arguments.
///
/// ## Examples
///
/// ```javascript
/// compose(sortBy(x => x), flatten, map(x => [x, x*2]));
/// ```
fn is_function_composition_args(arguments: &MCallArguments) -> bool {
    let args = arguments.args();

    if args.len() <= 1 {
        return false;
    }

    let mut has_seen_function_like = false;

    for arg in args.iter().flatten() {
        use AnyMExpression::*;
        match arg {
            AnyMCallArgument::AnyMExpression(MFunctionExpression(_)) => {
                if has_seen_function_like {
                    return true;
                }
                has_seen_function_like = true;
            }
            AnyMCallArgument::AnyMExpression(MCallExpression(call)) => {
                if call.arguments().is_ok_and(|call_arguments| {
                    call_arguments.args().iter().flatten().any(|arg| {
                        matches!(
                            arg,
                            AnyMCallArgument::AnyMExpression(MFunctionExpression(_))
                        )
                    })
                }) {
                    return true;
                }
            }
            _ => {
                continue;
            }
        }
    }

    false
}

fn is_query_like_call(expression: Option<&MCallExpression>) -> bool {
    if let Some(expression) = expression
        && let Ok(callee) = expression.callee() {
            let callee_name = match callee {
                AnyMExpression::MIdentifierExpression(expression) => expression.text(),
                AnyMExpression::MStaticMemberExpression(expression) => {
                    let member = expression.member();
                    if member.is_err() {
                        return false;
                    }
                    member.unwrap().text()
                }
                _ => return false,
            };
            return matches!(
                callee_name.to_ascii_lowercase().as_ref(),
                "query" | "command" | "bufferedreader" | "execute_command"
            );
        }

    false
}

// struct FormatQueryLikeArguments<'a> {
//     l_paren: &'a dyn Format<MFormatContext>,
//     args: &'a [FormatCallArgument],
//     r_paren: &'a dyn Format<MFormatContext>,
// }

// impl Format<MFormatContext> for FormatQueryLikeArguments<'_> {
//     fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
//         let first = self.args.first().ok_or(FormatError::SyntaxError)?;
//         let text_query = first.element().node()?.text();
//         let multi_line_query = text_query.lines().count() > 1;

//         write!(
//             f,
//             [group(&format_args![
//                 self.l_paren,
//                 &format_with(|f| {
//                     for (index, entry) in self.args.iter().enumerate() {
//                         match index {
//                             0 => {
//                                 if multi_line_query {
//                                     write!(f, [entry])?
//                                 } else {
//                                     write!(
//                                         f,
//                                         [indent(&format_args![
//                                             soft_line_break(),
//                                             entry,
//                                             soft_line_break(),
//                                         ],)]
//                                     )?
//                                 }
//                             }
//                             _ => write!(f, [space(), entry])?,
//                         }
//                     }

//                     write!(f, [FormatTrailingCommas::All])?;

//                     Ok(())
//                 }),
//             ])]
//         )?;

//         if multi_line_query {
//             write!(f, [self.r_paren])
//         } else {
//             write!(f, [soft_line_break(), self.r_paren])
//         }
//     }
// }
