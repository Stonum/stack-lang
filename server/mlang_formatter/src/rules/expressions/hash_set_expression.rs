use crate::prelude::*;

use biome_formatter::{FormatRuleWithOptions, write};
use biome_rowan::SyntaxResult;
use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{AnyMArrayElement, AnyMExpression, MHashSetExpressionFields};
use mlang_syntax::{MHashSetElementList, MHashSetExpression};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMHashSetExpression {
    options: FormatMHashSetExpressionOptions,
}
impl_format_with_rule!(MHashSetExpression, FormatMHashSetExpression);

impl FormatRuleWithOptions<MHashSetExpression> for FormatMHashSetExpression {
    type Options = FormatMHashSetExpressionOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.options = options;
        self
    }
}

impl FormatNodeRule<MHashSetExpression> for FormatMHashSetExpression {
    fn fmt_fields(&self, node: &MHashSetExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MHashSetExpressionFields {
            set_token,
            l_paren_token,
            members,
            r_paren_token,
        } = node.as_fields();

        let r_paren_token = r_paren_token?;

        if members.is_empty() {
            write!(
                f,
                [
                    set_token.format(),
                    l_paren_token.format(),
                    format_dangling_comments(node.syntax()).with_block_indent(),
                    r_paren_token.format(),
                ]
            )
        } else {
            let group_id = f.group_id("hashset");

            let should_expand = !self.options.is_force_flat_mode && should_break(&members)?;
            let members = members.format().with_options(Some(group_id));

            write!(
                f,
                [
                    set_token.format(),
                    l_paren_token.format(),
                    group(&soft_block_indent(&members))
                        .with_group_id(Some(group_id))
                        .should_expand(should_expand),
                    r_paren_token.format()
                ]
            )
        }
    }

    fn needs_parentheses(&self, item: &MHashSetExpression) -> bool {
        item.needs_parentheses()
    }

    fn fmt_dangling_comments(
        &self,
        _: &MHashSetExpression,
        _: &mut MFormatter,
    ) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatMHashSetExpressionOptions {
    pub(crate) is_force_flat_mode: bool,
}

/// Returns `true` for arrays containing at least two elements if:
/// * all elements are either object or array expressions
/// * each child array expression has at least two elements, or each child object expression has at least two members.
fn should_break(elements: &MHashSetElementList) -> SyntaxResult<bool> {
    if elements.len() < 2 {
        Ok(false)
    } else {
        let mut elements = elements.iter().peekable();

        while let Some(element) = elements.next() {
            match element? {
                AnyMArrayElement::AnyMExpression(AnyMExpression::MHashSetExpression(array)) => {
                    let next_is_array_or_end = matches!(
                        elements.peek(),
                        None | Some(Ok(AnyMArrayElement::AnyMExpression(
                            AnyMExpression::MHashSetExpression(_)
                        )))
                    );
                    if array.members().len() < 2 || !next_is_array_or_end {
                        return Ok(false);
                    }
                }
                AnyMArrayElement::AnyMExpression(AnyMExpression::MObjectExpression(object)) => {
                    let next_is_object_or_empty = matches!(
                        elements.peek(),
                        None | Some(Ok(AnyMArrayElement::AnyMExpression(
                            AnyMExpression::MObjectExpression(_)
                        )))
                    );

                    if object.members().len() < 2 || !next_is_object_or_empty {
                        return Ok(false);
                    }
                }
                _ => {
                    return Ok(false);
                }
            }
        }

        Ok(true)
    }
}
