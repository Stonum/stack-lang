use crate::prelude::*;
use crate::MLabels;

use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{
    AnyMAssignment, AnyMComputedMember, AnyMExpression, MAssignmentExpression, MInitializerClause,
    MName, MStaticMemberAssignment, MStaticMemberExpression, MSyntaxToken,
};
use biome_formatter::{format_args, write};
use biome_rowan::{declare_node_union, AstNode, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMStaticMemberExpression;
impl_format_with_rule!(MStaticMemberExpression, FormatMStaticMemberExpression);

impl FormatNodeRule<MStaticMemberExpression> for FormatMStaticMemberExpression {
    fn fmt_fields(&self, node: &MStaticMemberExpression, f: &mut MFormatter) -> FormatResult<()> {
        AnyMStaticMemberLike::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &MStaticMemberExpression) -> bool {
        item.needs_parentheses()
    }
}

#[derive(Debug, Copy, Clone)]
enum StaticMemberLikeLayout {
    /// Forces that there's no line break between the object, operator, and member
    NoBreak,

    /// Breaks the static member expression after the object if the whole expression doesn't fit on a single line
    BreakAfterObject,
}

declare_node_union! {
    pub(crate) AnyMStaticMemberLike = MStaticMemberExpression | MStaticMemberAssignment
}

impl Format<MFormatContext> for AnyMStaticMemberLike {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        let is_member_chain = {
            let mut recording = f.start_recording();
            write!(recording, [self.object().format()])?;

            recording
                .stop()
                .has_label(LabelId::of(MLabels::MemberChain))
        };

        let layout = self.layout(is_member_chain)?;

        match layout {
            StaticMemberLikeLayout::NoBreak => {
                let format_no_break = format_with(|f| {
                    write!(f, [self.operator_token().format(), self.member().format()])
                });

                if is_member_chain {
                    write!(
                        f,
                        [labelled(
                            LabelId::of(MLabels::MemberChain),
                            &format_no_break
                        )]
                    )
                } else {
                    write!(f, [format_no_break])
                }
            }
            StaticMemberLikeLayout::BreakAfterObject => {
                write!(
                    f,
                    [group(&indent(&format_args![
                        soft_line_break(),
                        self.operator_token().format(),
                        self.member().format(),
                    ]))]
                )
            }
        }
    }
}

impl AnyMStaticMemberLike {
    fn object(&self) -> SyntaxResult<AnyMExpression> {
        match self {
            AnyMStaticMemberLike::MStaticMemberExpression(expression) => expression.object(),
            AnyMStaticMemberLike::MStaticMemberAssignment(assignment) => assignment.object(),
        }
    }

    fn operator_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            AnyMStaticMemberLike::MStaticMemberExpression(expression) => {
                expression.operator_token()
            }
            AnyMStaticMemberLike::MStaticMemberAssignment(assignment) => assignment.dot_token(),
        }
    }

    fn member(&self) -> SyntaxResult<MName> {
        match self {
            AnyMStaticMemberLike::MStaticMemberExpression(expression) => expression.member(),
            AnyMStaticMemberLike::MStaticMemberAssignment(assignment) => assignment.member(),
        }
    }

    fn layout(&self, is_member_chain: bool) -> SyntaxResult<StaticMemberLikeLayout> {
        let parent = self.syntax().parent();
        let object = self.object()?;

        let is_nested = match &parent {
            Some(parent) => {
                if MAssignmentExpression::can_cast(parent.kind())
                    || MInitializerClause::can_cast(parent.kind())
                {
                    let no_break = match &object {
                        AnyMExpression::MCallExpression(call_expression) => {
                            !call_expression.arguments()?.args().is_empty()
                        }
                        _ => false,
                    };

                    if no_break || is_member_chain {
                        return Ok(StaticMemberLikeLayout::NoBreak);
                    }
                }

                AnyMStaticMemberLike::can_cast(parent.kind())
                    || AnyMComputedMember::can_cast(parent.kind())
            }
            None => false,
        };

        if !is_nested && matches!(&object, AnyMExpression::MIdentifierExpression(_)) {
            return Ok(StaticMemberLikeLayout::NoBreak);
        }

        let first_non_static_member_ancestor = self.syntax().ancestors().find(|parent| {
            !(AnyMStaticMemberLike::can_cast(parent.kind())
                || AnyMComputedMember::can_cast(parent.kind()))
        });

        let layout = match first_non_static_member_ancestor.and_then(AnyMExpression::cast) {
            Some(AnyMExpression::MNewExpression(_)) => StaticMemberLikeLayout::NoBreak,
            Some(AnyMExpression::MAssignmentExpression(assignment)) => {
                if matches!(assignment.left()?, AnyMAssignment::MIdentifierAssignment(_)) {
                    StaticMemberLikeLayout::BreakAfterObject
                } else {
                    StaticMemberLikeLayout::NoBreak
                }
            }
            _ => StaticMemberLikeLayout::BreakAfterObject,
        };

        Ok(layout)
    }
}
