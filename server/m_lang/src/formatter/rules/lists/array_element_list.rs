use crate::formatter::prelude::*;
use biome_formatter::{write, CstFormatContext, FormatRuleWithOptions, GroupId};

use crate::formatter::utils::array::write_array_node;

use crate::formatter::context::trailing_commas::FormatTrailingCommas;
use crate::syntax::MArrayElementList;
use biome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMArrayElementList {
    group_id: Option<GroupId>,
}
impl_format!(MArrayElementList, FormatMArrayElementList);

impl FormatRuleWithOptions<MArrayElementList> for FormatMArrayElementList {
    type Options = Option<GroupId>;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.group_id = options;
        self
    }
}

impl FormatRule<MArrayElementList> for FormatMArrayElementList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MArrayElementList, f: &mut MFormatter) -> FormatResult<()> {
        let layout = if can_concisely_print_array_list(node, f.context().comments()) {
            ArrayLayout::Fill
        } else {
            ArrayLayout::OnePerLine
        };

        match layout {
            ArrayLayout::Fill => {
                let trailing_separator = FormatTrailingCommas::ES5.trailing_separator(f.options());

                let mut filler = f.fill();

                // Using format_separated is valid in this case as can_print_fill does not allow holes
                for (element, formatted) in node.iter().zip(
                    node.format_separated(",")
                        .with_trailing_separator(trailing_separator)
                        .with_group_id(self.group_id),
                ) {
                    filler.entry(
                        &format_once(|f| {
                            let element = element?;
                            if get_lines_before(element.syntax()) > 1 {
                                write!(f, [empty_line()])
                            } else if f.comments().has_leading_own_line_comment(element.syntax()) {
                                write!(f, [hard_line_break()])
                            } else {
                                write!(f, [soft_line_break_or_space()])
                            }
                        }),
                        &formatted,
                    );
                }

                filler.finish()
            }
            ArrayLayout::OnePerLine => write_array_node(node, f),
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum ArrayLayout {
    /// Tries to fit as many array elements on a single line as possible.
    ///
    /// ```javascript
    /// [
    ///     1, 2, 3,
    ///     5, 6,
    /// ]
    /// ```
    Fill,

    /// Prints every element on a single line if the whole array expression exceeds the line width, or any
    /// of its elements gets printed in *expanded* mode.
    /// ```javascript
    /// [
    ///     a.b(),
    ///     4,
    ///     3,
    /// ]
    /// ```
    OnePerLine,
}

/// Returns true if the provided MArrayElementList could
/// be "fill-printed" instead of breaking each element on
/// a different line.
///
/// The underlying logic only allows lists of literal expressions
/// with 10 or less characters, potentially wrapped in a "short"
/// unary expression (+, -, ~ or !)
pub(crate) fn can_concisely_print_array_list(
    list: &MArrayElementList,
    comments: &MComments,
) -> bool {
    use crate::syntax::AnyMArrayElement::*;
    use crate::syntax::AnyMExpression::*;
    use crate::syntax::MUnaryOperator::*;

    if list.is_empty() {
        return false;
    }

    list.elements().all(|item| {
        let syntax = match item.into_node() {
            Ok(AnyMExpression(AnyMLiteralExpression(
                crate::syntax::AnyMLiteralExpression::MNumberLiteralExpression(literal),
            ))) => literal.into_syntax(),

            Ok(AnyMExpression(MUnaryExpression(expr))) => {
                let signed = matches!(expr.operator(), Ok(Plus | Minus));
                let argument = expr.argument();

                match argument {
                    Ok(AnyMLiteralExpression(
                        crate::syntax::AnyMLiteralExpression::MNumberLiteralExpression(literal),
                    )) => {
                        if signed && !comments.has_comments(literal.syntax()) {
                            expr.into_syntax()
                        } else {
                            return false;
                        }
                    }
                    _ => {
                        return false;
                    }
                }
            }

            _ => {
                return false;
            }
        };

        // Does not have a line comment ending on the same line
        // ```javascript
        // [ a // not this
        //  b];
        //
        // [
        //   // This is fine
        //   thats
        // ]
        // ```
        !comments
            .trailing_comments(&syntax)
            .iter()
            .filter(|comment| comment.kind().is_line())
            .any(|comment| comment.lines_before() == 0)
    })
}
