use super::member_chain::is_member_call_chain;
use super::object::write_member_name;
use super::{FormatLiteralStringToken, StringLiteralParentKind};
use crate::prelude::*;
use mlang_syntax::binary_like_expression::AnyMBinaryLikeExpression;
use mlang_syntax::{
    AnyMAssignment, AnyMCallArgument, AnyMExpression, MAssignmentExpression, MInitializerClause,
    MPropertyObjectMember, MSyntaxKind, MVariableDeclarator,
};
use mlang_syntax::{AnyMLiteralExpression, MUnaryExpression};
use biome_formatter::{format_args, write, CstFormatContext, FormatOptions, VecBuffer};
use biome_rowan::{declare_node_union, AstNode, SyntaxNodeOptionExt, SyntaxResult};
use std::iter;

declare_node_union! {
    pub(crate) AnyMAssignmentLike =
        MPropertyObjectMember |
        MAssignmentExpression |
        MVariableDeclarator
}

declare_node_union! {
    pub(crate) RightAssignmentLike = AnyMExpression | AnyMAssignment | MInitializerClause
}

impl RightAssignmentLike {
    fn as_expression(&self) -> Option<AnyMExpression> {
        match self {
            RightAssignmentLike::AnyMExpression(expression) => Some(expression.clone()),
            RightAssignmentLike::MInitializerClause(initializer) => initializer.expression().ok(),
            RightAssignmentLike::AnyMAssignment(_) => None,
        }
    }
}

impl Format<MFormatContext> for RightAssignmentLike {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        match self {
            RightAssignmentLike::AnyMExpression(expression) => {
                write!(f, [expression.format()])
            }
            RightAssignmentLike::AnyMAssignment(assignment) => {
                write!(f, [assignment.format()])
            }
            RightAssignmentLike::MInitializerClause(initializer) => {
                write!(f, [space(), initializer.format()])
            }
        }
    }
}

/// Determines how a assignment like be formatted
///
/// Assignment like are:
/// - Assignment
/// - Object property member
/// - Variable declaration
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum AssignmentLikeLayout {
    /// This is a special layout usually used for variable declarations.
    /// This layout is hit, usually, when a [variable declarator](MVariableDeclarator) doesn't have initializer:
    /// ```JavaScript
    ///     let variable;
    /// ```
    /// ```ts
    ///     let variable: Map<string, number>;
    /// ```
    OnlyLeft,

    /// First break right-hand side, then after operator.
    /// ```JavaScript
    /// {
    ///   "array-key": [
    ///     {
    ///       "nested-key-1": 1,
    ///       "nested-key-2": 2,
    ///     },
    ///   ]
    /// }
    /// ```
    Fluid,

    /// First break after operator, then the sides are broken independently on their own lines.
    /// There is a soft line break after operator token.
    /// ```JavaScript
    /// {
    ///     "enough-long-key-to-break-line":
    ///         1 + 2,
    ///     "not-long-enough-key":
    ///         "but long enough string to break line",
    /// }
    /// ```
    BreakAfterOperator,

    /// First break right-hand side, then left-hand side. There are not any soft line breaks
    /// between left and right parts
    /// ```JavaScript
    /// {
    ///     key1: "123",
    ///     key2: 123,
    ///     key3: class MyClass {
    ///        constructor() {},
    ///     },
    /// }
    /// ```
    NeverBreakAfterOperator,

    /// This is a special layout usually used for long variable declarations or assignment expressions
    /// This layout is hit, usually, when we are in the "middle" of the chain:
    ///
    /// ```JavaScript
    /// var a =
    ///     loreum =
    ///     ipsum =
    ///         "foo";
    /// ```
    ///
    /// Given the previous snippet, then `loreum` and `ipsum` will be formatted using the [Chain] layout.
    Chain,

    /// This is a special layout usually used for long variable declarations or assignment expressions
    /// This layout is hit, usually, when we are in the end of a chain:
    /// ```JavaScript
    /// var a = loreum = ipsum = "foo";
    /// ```
    ///
    /// Given the previous snippet, then `"foo"` formatted  using the [ChainTail] layout.
    ChainTail,

    /// This layout is used in cases where we want to "break" the left hand side
    /// of assignment like expression, but only when the group decides to do it.
    ///
    /// ```JavaScript
    /// const a {
    ///     loreum: { ipsum },
    ///     something_else,
    ///     happy_days: { fonzy }
    /// } = obj;
    /// ```
    ///
    /// The snippet triggers the layout because the left hand side contains a "complex destructuring"
    /// which requires having the properties broke on different lines.
    BreakLeftHandSide,

    /// Layout used when the operator and right hand side are part of a `MInitializerClause<
    /// that has a suppression comment.
    SuppressedInitializer,
}

const MIN_OVERLAP_FOR_BREAK: u8 = 3;

impl AnyMAssignmentLike {
    fn right(&self) -> SyntaxResult<RightAssignmentLike> {
        let right = match self {
            AnyMAssignmentLike::MPropertyObjectMember(property) => property.value()?.into(),
            AnyMAssignmentLike::MAssignmentExpression(assignment) => assignment.right()?.into(),
            AnyMAssignmentLike::MVariableDeclarator(variable_declarator) => {
                // SAFETY: Calling `unwrap` here is safe because we check `has_only_left_hand_side` variant at the beginning of the `layout` function
                variable_declarator.initializer().unwrap().into()
            }
        };

        Ok(right)
    }

    fn write_left(&self, f: &mut MFormatter) -> FormatResult<bool> {
        match self {
            AnyMAssignmentLike::MPropertyObjectMember(property) => {
                let name = property.name()?;

                // It's safe to mark the name as checked here because it is at the beginning of the property
                // and any suppression comment that would apply to the name applies to the property too and is,
                // thus, handled on the property level.
                f.context()
                    .comments()
                    .mark_suppression_checked(name.syntax());

                let width = write_member_name(&name, f)?;
                let text_width_for_break =
                    (u8::from(f.options().tab_width()) + MIN_OVERLAP_FOR_BREAK) as usize;
                Ok(width < text_width_for_break)
            }
            AnyMAssignmentLike::MAssignmentExpression(assignment) => {
                let left = assignment.left()?;
                write!(f, [&left.format()])?;
                Ok(false)
            }
            AnyMAssignmentLike::MVariableDeclarator(variable_declarator) => {
                let id = variable_declarator.id()?;
                write!(f, [id.format()])?;
                Ok(false)
            }
        }
    }

    fn write_operator(&self, f: &mut MFormatter) -> FormatResult<()> {
        match self {
            AnyMAssignmentLike::MPropertyObjectMember(property) => {
                let colon_token = property.colon_token()?;
                write!(f, [colon_token.format()])
            }
            AnyMAssignmentLike::MAssignmentExpression(assignment) => {
                let operator_token = assignment.operator_token()?;
                write!(f, [space(), operator_token.format()])
            }
            AnyMAssignmentLike::MVariableDeclarator(variable_declarator) => {
                if let Some(initializer) = variable_declarator.initializer() {
                    let eq_token = initializer.eq_token()?;
                    write!(f, [space(), eq_token.format()])?
                }
                Ok(())
            }
        }
    }

    fn write_right(&self, f: &mut MFormatter) -> FormatResult<()> {
        match self {
            AnyMAssignmentLike::MPropertyObjectMember(property) => {
                let value = property.value()?;
                write!(f, [with_assignment_layout(&value)])
            }
            AnyMAssignmentLike::MAssignmentExpression(assignment) => {
                let right = assignment.right()?;
                write!(f, [space(), with_assignment_layout(&right)])
            }
            AnyMAssignmentLike::MVariableDeclarator(variable_declarator) => {
                if let Some(initializer) = variable_declarator.initializer() {
                    let expression = initializer.expression()?;
                    write!(
                        f,
                        [
                            space(),
                            format_leading_comments(initializer.syntax()),
                            with_assignment_layout(&expression),
                            format_trailing_comments(initializer.syntax())
                        ]
                    )?;
                }
                Ok(())
            }
        }
    }

    fn write_suppressed_initializer(&self, f: &mut MFormatter) -> FormatResult<()> {
        let initializer = match self {
            AnyMAssignmentLike::MVariableDeclarator(variable_declarator) => {
                variable_declarator.initializer()
            }

            AnyMAssignmentLike::MPropertyObjectMember(_)
            | AnyMAssignmentLike::MAssignmentExpression(_) => {
                unreachable!("These variants have no initializer")
            }
        };

        let initializer =
            initializer.expect("Expected an initializer because it has a suppression comment");

        write!(f, [soft_line_indent_or_space(&initializer.format())])
    }

    /// Returns the layout variant for an assignment like depending on right expression and left part length
    /// [Prettier applies]: https://github.com/prettier/prettier/blob/main/src/language-M/print/assignment.M
    fn layout(
        &self,
        is_left_short: bool,
        left_may_break: bool,
        f: &mut Formatter<MFormatContext>,
    ) -> FormatResult<AssignmentLikeLayout> {
        if self.has_only_left_hand_side() {
            return Ok(AssignmentLikeLayout::OnlyLeft);
        }

        let right = self.right()?;

        if let RightAssignmentLike::MInitializerClause(initializer) = &right
            && f.context().comments().is_suppressed(initializer.syntax()) {
                return Ok(AssignmentLikeLayout::SuppressedInitializer);
            }
        let right_expression = right.as_expression();

        if let Some(layout) = self.chain_formatting_layout(right_expression.as_ref())? {
            return Ok(layout);
        }

        if let Some(AnyMExpression::MCallExpression(call_expression)) = &right_expression
            && call_expression.callee()?.syntax().text() == "require" {
                return Ok(AssignmentLikeLayout::NeverBreakAfterOperator);
            }

        if self.should_break_left_hand_side()? {
            return Ok(AssignmentLikeLayout::BreakLeftHandSide);
        }

        if self.should_break_after_operator(&right, f)? {
            return Ok(AssignmentLikeLayout::BreakAfterOperator);
        }

        if is_left_short {
            return Ok(AssignmentLikeLayout::NeverBreakAfterOperator);
        }

        // Before checking `BreakAfterOperator` layout, we need to unwrap the right expression from `MUnaryExpression` or `TsNonNullAssertionExpression`
        // [Prettier applies]: https://github.com/prettier/prettier/blob/a043ac0d733c4d53f980aa73807a63fc914f23bd/src/language-M/print/assignment.M#L199-L211
        // Example:
        //  !"123" -> "123"
        //  void "123" -> "123"
        //  !!"string"! -> "string"
        let right_expression = iter::successors(right_expression, |expression| match expression {
            AnyMExpression::MUnaryExpression(unary) => unary.argument().ok(),

            _ => None,
        })
        .last();

        if matches!(
            right_expression,
            Some(AnyMExpression::AnyMLiteralExpression(
                AnyMLiteralExpression::MStringLiteralExpression(_)
            )),
        ) {
            return Ok(AssignmentLikeLayout::BreakAfterOperator);
        }

        let is_poorly_breakable = match &right_expression {
            Some(expression) => is_poorly_breakable_member_or_call_chain(expression, f)?,
            None => false,
        };

        if is_poorly_breakable {
            return Ok(AssignmentLikeLayout::BreakAfterOperator);
        }

        if !left_may_break
            && matches!(
                right_expression,
                Some(AnyMExpression::AnyMLiteralExpression(
                    AnyMLiteralExpression::MBooleanLiteralExpression(_)
                        | AnyMLiteralExpression::MNumberLiteralExpression(_)
                ))
            )
        {
            return Ok(AssignmentLikeLayout::NeverBreakAfterOperator);
        }

        Ok(AssignmentLikeLayout::Fluid)
    }

    /// Checks that a [MAnyAssignmentLike] consists only of the left part
    /// usually, when a [variable declarator](MVariableDeclarator) doesn't have initializer
    fn has_only_left_hand_side(&self) -> bool {
        if let AnyMAssignmentLike::MVariableDeclarator(declarator) = self {
            declarator.initializer().is_none()
        } else {
            false
        }
    }

    /// Checks if the right node is entitled of the chain formatting,
    /// and if so, it return the layout type
    fn chain_formatting_layout(
        &self,
        right_expression: Option<&AnyMExpression>,
    ) -> SyntaxResult<Option<AssignmentLikeLayout>> {
        let right_is_tail = !matches!(
            right_expression,
            Some(AnyMExpression::MAssignmentExpression(_))
        );

        // The chain goes up two levels, by checking up to the great parent if all the conditions
        // are correctly met.
        let upper_chain_is_eligible =
            // First, we check if the current node is an assignment expression
            if let AnyMAssignmentLike::MAssignmentExpression(assignment) = self {
                assignment.syntax().parent().is_some_and(|parent| {
                    // Then we check if the parent is assignment expression or variable declarator
                    if matches!(
                        parent.kind(),
                        MSyntaxKind::M_ASSIGNMENT_EXPRESSION
                            | MSyntaxKind::M_INITIALIZER_CLAUSE
                    ) {
                        let great_parent_kind = parent.parent().kind();
                        // Finally, we check the great parent.
                        // The great parent triggers the eligibility when
                        // - the current node that we were inspecting is not a "tail"
                        // - or the great parent is not an expression statement or a variable declarator
                        !right_is_tail
                            || !matches!(
                                great_parent_kind,
                                Some(
                                    MSyntaxKind::M_EXPRESSION_STATEMENT
                                        | MSyntaxKind::M_VARIABLE_DECLARATOR
                                )
                            )
                    } else {
                        false
                    }
                })
            } else {
                false
            };

        let result = if upper_chain_is_eligible {
            if !right_is_tail {
                Some(AssignmentLikeLayout::Chain)
            } else {
                Some(AssignmentLikeLayout::ChainTail)
            }
        } else {
            None
        };

        Ok(result)
    }

    fn is_complex_type_alias(&self) -> SyntaxResult<bool> {
        let result = false;

        Ok(result)
    }

    /// Particular function that checks if the left hand side of a [MAnyAssignmentLike] should
    /// be broken on multiple lines
    fn should_break_left_hand_side(&self) -> SyntaxResult<bool> {
        let is_complex_destructuring = false;

        let is_complex_type_alias = self.is_complex_type_alias()?;

        Ok(is_complex_destructuring || is_complex_type_alias)
    }

    /// Checks if the current assignment is eligible for [AssignmentLikeLayout::BreakAfterOperator]
    ///
    /// This function is small wrapper around [should_break_after_operator] because it has to work
    /// for nodes that belong to TypeScript too.
    fn should_break_after_operator(
        &self,
        right: &RightAssignmentLike,
        f: &Formatter<MFormatContext>,
    ) -> SyntaxResult<bool> {
        let comments = f.context().comments();
        let result = match right {
            RightAssignmentLike::AnyMExpression(expression) => {
                should_break_after_operator(expression, comments, f)?
            }
            RightAssignmentLike::MInitializerClause(initializer) => {
                comments.has_leading_own_line_comment(initializer.syntax())
                    || should_break_after_operator(&initializer.expression()?, comments, f)?
            }
            right => comments.has_leading_own_line_comment(right.syntax()),
        };

        Ok(result)
    }
}

/// Checks if the function is entitled to be printed with layout [AssignmentLikeLayout::BreakAfterOperator]
pub(crate) fn should_break_after_operator(
    right: &AnyMExpression,
    comments: &MComments,
    f: &Formatter<MFormatContext>,
) -> SyntaxResult<bool> {
    if comments.has_leading_own_line_comment(right.syntax()) {
        return Ok(true);
    }

    let result = match right {
        // head is a long chain, meaning that right -> right are both assignment expressions
        AnyMExpression::MAssignmentExpression(assignment) => {
            matches!(
                assignment.right()?,
                AnyMExpression::MAssignmentExpression(_)
            )
        }
        right if AnyMBinaryLikeExpression::can_cast(right.syntax().kind()) => {
            let binary_like = AnyMBinaryLikeExpression::unwrap_cast(right.syntax().clone());

            !binary_like.should_inline_logical_expression()
        }

        AnyMExpression::MSequenceExpression(_) => true,

        AnyMExpression::MConditionalExpression(conditional) => {
            AnyMBinaryLikeExpression::cast(conditional.test()?.into_syntax())
                .is_some_and(|expression| !expression.should_inline_logical_expression())
        }

        _ => {
            let argument = match right {
                AnyMExpression::MUnaryExpression(expression) => {
                    get_last_non_unary_argument(expression)
                }
                _ => None,
            };

            if let Some(argument) = argument {
                matches!(argument, AnyMExpression::AnyMLiteralExpression(_))
                    || is_poorly_breakable_member_or_call_chain(&argument, f)?
            } else {
                false
            }
        }
    };

    Ok(result)
}

/// Iterate over unary expression arguments to get last non-unary
/// Example: void !!(await test()) -> returns await as last argument
fn get_last_non_unary_argument(unary_expression: &MUnaryExpression) -> Option<AnyMExpression> {
    let mut argument = unary_expression.argument().ok()?;

    while let AnyMExpression::MUnaryExpression(ref unary) = argument {
        argument = match unary.argument() {
            Ok(arg) => arg,
            _ => break,
        };
    }

    Some(argument)
}

impl Format<MFormatContext> for AnyMAssignmentLike {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        let format_content = format_with(|f| {
            // We create a temporary buffer because the left hand side has to conditionally add
            // a group based on the layout, but the layout can only be computed by knowing the
            // width of the left hand side. The left hand side can be a member, and that has a width
            // can can be known only when it's formatted (it can incur in some transformation,
            // like removing some escapes, etc.).
            //
            // 1. we crate a temporary buffer
            // 2. we write the left hand side into the buffer and retrieve the `is_left_short` info
            // which is computed only when we format it
            // 3. we compute the layout
            // 4. we write the left node inside the main buffer based on the layout
            let mut buffer = VecBuffer::new(f.state_mut());
            let is_left_short = self.write_left(&mut Formatter::new(&mut buffer))?;
            let formatted_left = buffer.into_vec();
            let left_may_break = formatted_left.may_directly_break();

            // Compare name only if we are in a position of computing it.
            // If not (for example, left is not an identifier), then let's fallback to false,
            // so we can continue the chain of checks
            let layout = self.layout(is_left_short, left_may_break, f)?;

            let left = format_once(|f| f.write_elements(formatted_left));
            let right = format_with(|f| self.write_right(f));

            let inner_content = format_with(|f| {
                if matches!(
                    &layout,
                    AssignmentLikeLayout::BreakLeftHandSide | AssignmentLikeLayout::OnlyLeft
                ) {
                    write!(f, [left])?;
                } else {
                    write!(f, [group(&left)])?;
                }

                if layout != AssignmentLikeLayout::SuppressedInitializer {
                    self.write_operator(f)?;
                }

                match layout {
                    AssignmentLikeLayout::OnlyLeft => Ok(()),
                    AssignmentLikeLayout::Fluid => {
                        let group_id = f.group_id("assignment_like");

                        write![
                            f,
                            [
                                group(&indent(&soft_line_break_or_space()))
                                    .with_group_id(Some(group_id)),
                                line_suffix_boundary(),
                                indent_if_group_breaks(&right, group_id)
                            ]
                        ]
                    }
                    AssignmentLikeLayout::BreakAfterOperator => {
                        write![f, [group(&soft_line_indent_or_space(&right))]]
                    }
                    AssignmentLikeLayout::NeverBreakAfterOperator => {
                        write![f, [space(), right]]
                    }

                    AssignmentLikeLayout::BreakLeftHandSide => {
                        write![f, [space(), group(&right)]]
                    }

                    AssignmentLikeLayout::Chain => {
                        write!(f, [soft_line_break_or_space(), right])
                    }

                    AssignmentLikeLayout::ChainTail => {
                        write!(
                            f,
                            [&indent(&format_args![soft_line_break_or_space(), right])]
                        )
                    }

                    AssignmentLikeLayout::SuppressedInitializer => {
                        self.write_suppressed_initializer(f)
                    }
                }
            });

            match layout {
                // Layouts that don't need enclosing group
                AssignmentLikeLayout::Chain
                | AssignmentLikeLayout::ChainTail
                | AssignmentLikeLayout::SuppressedInitializer
                | AssignmentLikeLayout::OnlyLeft => {
                    write!(f, [&inner_content])
                }
                _ => {
                    write!(f, [group(&inner_content)])
                }
            }
        });

        write!(f, [format_content])
    }
}

/// A chain that has no calls at all or all of whose calls have no arguments
/// or have only one which [is_short_argument], except for member call chains
/// [Prettier applies]: https://github.com/prettier/prettier/blob/a043ac0d733c4d53f980aa73807a63fc914f23bd/src/language-M/print/assignment.M#L329
fn is_poorly_breakable_member_or_call_chain(
    expression: &AnyMExpression,
    f: &Formatter<MFormatContext>,
) -> SyntaxResult<bool> {
    let threshold = f.options().line_width().get() / 4;

    // Only call and member chains are poorly breakable
    // - `obj.member.prop`
    // - `obj.member()()`
    let mut is_chain = false;

    // Only chains with simple head are poorly breakable
    // Simple head is `MIdentifierExpression` or `MThisExpression`
    let mut is_chain_head_simple = false;

    // Keeping track of all call expressions in the chain to check them later
    let mut call_expressions = vec![];

    let mut expression = Some(expression.clone());

    while let Some(node) = expression.take() {
        expression = match node {
            AnyMExpression::MCallExpression(call_expression) => {
                is_chain = true;
                let callee = call_expression.callee()?;
                call_expressions.push(call_expression);
                Some(callee)
            }
            AnyMExpression::MStaticMemberExpression(node) => {
                is_chain = true;
                Some(node.object()?)
            }
            AnyMExpression::MComputedMemberExpression(node) => {
                is_chain = true;
                node.object()
            }
            AnyMExpression::MIdentifierExpression(_) | AnyMExpression::MThisExpression(_) => {
                is_chain_head_simple = true;
                break;
            }
            _ => {
                break;
            }
        }
    }

    if !is_chain || !is_chain_head_simple {
        return Ok(false);
    }

    for call_expression in call_expressions {
        if is_member_call_chain(
            call_expression.clone(),
            f.comments(),
            f.options().tab_width(),
        )? {
            return Ok(false);
        }

        let args = call_expression.arguments()?.args();

        let is_breakable_call = match args.len() {
            0 => false,
            1 => match args.iter().next() {
                Some(first_argument) => !is_short_argument(first_argument?, threshold, f)?,
                None => false,
            },
            _ => true,
        };

        if is_breakable_call {
            return Ok(false);
        }
    }

    Ok(true)
}

/// This function checks if `MAnyCallArgument` is short
/// We need it to decide if `MCallExpression` with the argument is breakable or not
/// If the argument is short the function call isn't breakable
/// [Prettier applies]: https://github.com/prettier/prettier/blob/a043ac0d733c4d53f980aa73807a63fc914f23bd/src/language-M/print/assignment.M#L374
fn is_short_argument(
    argument: AnyMCallArgument,
    threshold: u16,
    f: &Formatter<MFormatContext>,
) -> SyntaxResult<bool> {
    let comments = f.comments();

    if comments.has_comments(argument.syntax()) {
        return Ok(false);
    }

    if let AnyMCallArgument::AnyMExpression(expression) = argument {
        let is_short_argument = match expression {
            AnyMExpression::MThisExpression(_) => true,
            AnyMExpression::MIdentifierExpression(identifier) => {
                identifier.name()?.value_token()?.text_trimmed().len() <= threshold as usize
            }
            AnyMExpression::MUnaryExpression(unary_expression) => {
                let has_comments = comments.has_comments(unary_expression.argument()?.syntax());

                unary_expression.is_signed_numeric_literal()? && !has_comments
            }
            AnyMExpression::AnyMLiteralExpression(literal) => match literal {
                AnyMLiteralExpression::MStringLiteralExpression(string) => {
                    let token = string.value_token()?;
                    let formatter =
                        FormatLiteralStringToken::new(&token, StringLiteralParentKind::Expression);

                    formatter.clean_text().width() <= threshold as usize
                }
                _ => true,
            },
            _ => false,
        };
        Ok(is_short_argument)
    } else {
        Ok(false)
    }
}

/// Formats an expression and passes the assignment layout to its formatting function if the expressions
/// formatting rule takes the layout as an option.
pub(crate) struct WithAssignmentLayout<'a> {
    expression: &'a AnyMExpression,
}

pub(crate) fn with_assignment_layout(expression: &AnyMExpression) -> WithAssignmentLayout<'_> {
    WithAssignmentLayout { expression }
}

impl Format<MFormatContext> for WithAssignmentLayout<'_> {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        self.expression.format().fmt(f)
    }
}
