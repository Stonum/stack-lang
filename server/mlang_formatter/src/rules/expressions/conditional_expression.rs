use crate::prelude::*;
use mlang_syntax::parentheses::NeedsParentheses;
use biome_formatter::{write, CstFormatContext, FormatOptions};
use biome_rowan::AstNode;

use mlang_syntax::{
    AnyMExpression, MAssignmentExpression, MConditionalExpression, MInitializerClause,
    MReturnStatement, MStaticMemberExpression, MSyntaxKind, MSyntaxNode, MThrowStatement,
    MUnaryExpression,
};

#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct FormatMConditionalExpression;
impl_format_with_rule!(MConditionalExpression, FormatMConditionalExpression);

impl FormatNodeRule<MConditionalExpression> for FormatMConditionalExpression {
    fn fmt_fields(
        &self,
        conditional: &MConditionalExpression,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        let syntax = conditional.syntax();
        let consequent = conditional.consequent()?;
        let alternate = conditional.alternate()?;
        let indent_style = f.options().indent_style();
        let layout = self.layout(conditional);

        let format_consequent_and_alternate = format_with(|f| {
            write!(
                f,
                [
                    soft_line_break_or_space(),
                    conditional.question_mark_token().format(),
                    space()
                ]
            )?;

            let is_consequent_nested = consequent.syntax().kind() == syntax.kind();
            let consequent = format_with(|f| {
                if indent_style.is_space() {
                    write!(f, [align(2, &consequent)])
                } else {
                    write!(f, [indent(&consequent)])
                }
            });
            if is_consequent_nested {
                // Add parentheses around the consequent if it is a conditional expression and fits on the same line
                // so that it's easier to identify the parts that belong to a conditional expression.
                // `a ? b ? c: d : e` -> `a ? (b ? c: d) : e
                write!(
                    f,
                    [
                        if_group_fits_on_line(&text("(")),
                        consequent,
                        if_group_fits_on_line(&text(")"))
                    ]
                )?;
            } else {
                write!(f, [consequent])?;
            }

            write!(
                f,
                [
                    soft_line_break_or_space(),
                    conditional.colon_token().format(),
                    space()
                ]
            )?;
            let alternate = format_with(|f| {
                if indent_style.is_space() {
                    write!(f, [align(2, &alternate)])
                } else {
                    write!(f, [indent(&alternate)])
                }
            });
            write!(f, [alternate])
        });

        let format_tail_with_indent = format_with(|f: &mut MFormatter| {
            // Add an extra level of indent to nested consequences.
            if layout.is_nested_consequent() {
                // This may look silly but the `dedent` is to remove the outer `align` added by the parent's formatting of the consequent.
                // The `indent` is necessary to indent the content by one level with a tab.
                // Adding an `indent` without the `dedent` would result in the `outer` align being converted
                // into a `indent` + the `indent` added here, ultimately resulting in a two-level indention.
                write!(f, [dedent(&indent(&format_consequent_and_alternate))])
            } else {
                format_consequent_and_alternate.fmt(f)
            }
        });

        let should_extra_indent = self.should_extra_indent(conditional, &layout);

        let format_inner = format_with(|f| {
            write!(
                f,
                [FormatConditionalTest {
                    conditional,
                    layout: &layout,
                }]
            )?;

            // Indent the `consequent` and `alternate` **only if** this is the root conditional expression
            // OR this is the `test` of a conditional expression.
            if layout.is_root() || layout.is_nested_test() {
                write!(f, [indent(&format_tail_with_indent)])?;
            } else {
                // Don't indent for nested `alternate`s or `consequence`s
                write!(f, [format_tail_with_indent])?;
            }

            let break_closing_parentheses = self.is_parent_static_member_expression(&layout);

            // Add a soft line break in front of the closing `)` in case the parent is a static member expression
            // ```
            // (veryLongCondition
            //      ? a
            //      : b // <- enforce line break here if the conditional breaks
            // ).more
            // ```
            if break_closing_parentheses && !should_extra_indent {
                write!(f, [soft_line_break()])?;
            }

            Ok(())
        });

        let grouped = format_with(|f| {
            if layout.is_root() {
                group(&format_inner).fmt(f)
            } else {
                format_inner.fmt(f)
            }
        });

        let has_multiline_comment = {
            let comments = f.context().comments();

            let has_block_comment = |syntax: &MSyntaxNode| {
                comments
                    .leading_trailing_comments(syntax)
                    .any(|comment| comment.kind().is_block())
            };

            let test_has_block_comments = has_block_comment(conditional.test()?.syntax());

            test_has_block_comments
                || has_block_comment(consequent.syntax())
                || has_block_comment(alternate.syntax())
        };

        if layout.is_nested_test() || should_extra_indent {
            group(&soft_block_indent(&grouped))
                .should_expand(has_multiline_comment)
                .fmt(f)
        } else {
            if has_multiline_comment {
                write!(f, [expand_parent()])?;
            }

            grouped.fmt(f)
        }
    }

    fn needs_parentheses(&self, item: &MConditionalExpression) -> bool {
        item.needs_parentheses()
    }
}

impl FormatMConditionalExpression {
    fn layout(&self, conditional: &MConditionalExpression) -> ConditionalLayout {
        match conditional.syntax().parent() {
            Some(parent) if parent.kind() == conditional.syntax().kind() => {
                let conditional_parent = MConditionalExpression::unwrap_cast(parent);

                // if conditional_parent.is_test(conditional.syntax()) {
                if conditional_parent
                    .test()
                    .ok()
                    .is_some_and(|resolved| resolved.syntax() == conditional.syntax())
                {
                    ConditionalLayout::NestedTest {
                        parent: conditional_parent,
                    }
                } else if conditional_parent
                    .alternate()
                    .map(AstNode::into_syntax)
                    .is_ok()
                {
                    ConditionalLayout::NestedAlternate {
                        parent: conditional_parent,
                    }
                } else {
                    ConditionalLayout::NestedConsequent {
                        parent: conditional_parent,
                    }
                }
            }
            parent => ConditionalLayout::Root { parent },
        }
    }

    /// It is desired to add an extra indent if this conditional is a [MConditionalExpression] and is directly inside
    /// of a member chain:
    ///
    /// ```javascript
    /// // Input
    /// return (a ? b : c).member
    ///
    /// // Default
    /// return (a
    ///     ? b
    ///     : c
    /// ).member
    ///
    /// // Preferred
    /// return (
    ///     a
    ///         ? b
    ///         : c
    /// ).member
    /// ```
    fn should_extra_indent(
        &self,
        conditional: &MConditionalExpression,
        layout: &ConditionalLayout,
    ) -> bool {
        enum Ancestor {
            MemberChain(AnyMExpression),
            Root(MSyntaxNode),
        }

        let ancestors = layout
            .parent()
            .into_iter()
            .flat_map(|parent| parent.ancestors());
        let mut parent = None;
        let mut expression = AnyMExpression::from(conditional.clone());

        // This tries to find the start of a member chain by iterating over all ancestors of the conditional.
        // The iteration "breaks" as soon as a non-member-chain node is found.
        for ancestor in ancestors {
            let ancestor = match AnyMExpression::try_cast(ancestor) {
                Ok(AnyMExpression::MCallExpression(call_expression)) => {
                    if call_expression.callee().as_ref() == Ok(&expression) {
                        Ancestor::MemberChain(call_expression.into())
                    } else {
                        Ancestor::Root(call_expression.into_syntax())
                    }
                }

                Ok(AnyMExpression::MStaticMemberExpression(member_expression)) => {
                    if member_expression.object().as_ref() == Ok(&expression) {
                        Ancestor::MemberChain(member_expression.into())
                    } else {
                        Ancestor::Root(member_expression.into_syntax())
                    }
                }
                Ok(AnyMExpression::MComputedMemberExpression(member_expression)) => {
                    if member_expression.object().as_ref() == Some(&expression) {
                        Ancestor::MemberChain(member_expression.into())
                    } else {
                        Ancestor::Root(member_expression.into_syntax())
                    }
                }
                Ok(AnyMExpression::MNewExpression(new_expression)) => {
                    // Skip over new expressions
                    if new_expression.callee().as_ref() == Ok(&expression) {
                        parent = new_expression.syntax().parent();
                        expression = new_expression.into();
                        break;
                    }

                    Ancestor::Root(new_expression.into_syntax())
                }
                Ok(ancestor) => Ancestor::Root(ancestor.into_syntax()),
                Err(ancestor) => Ancestor::Root(ancestor),
            };

            match ancestor {
                Ancestor::MemberChain(left) => {
                    // Store the node that is highest in the member chain
                    expression = left;
                }
                Ancestor::Root(root) => {
                    parent = Some(root);
                    break;
                }
            }
        }

        // Don't indent if this conditional isn't part of a member chain.
        // e.g. don't indent for `return a ? b : c`, only for `return (a ? b : c).member`
        if expression.syntax() == conditional.syntax() {
            return false;
        }

        match parent {
            None => false,
            Some(parent) => {
                let argument = match parent.kind() {
                    MSyntaxKind::M_INITIALIZER_CLAUSE => {
                        let initializer = MInitializerClause::unwrap_cast(parent);
                        initializer.expression().ok()
                    }
                    MSyntaxKind::M_RETURN_STATEMENT => {
                        let return_statement = MReturnStatement::unwrap_cast(parent);
                        return_statement.argument()
                    }
                    MSyntaxKind::M_THROW_STATEMENT => {
                        let throw_statement = MThrowStatement::unwrap_cast(parent);
                        throw_statement.argument().ok()
                    }
                    MSyntaxKind::M_UNARY_EXPRESSION => {
                        let unary_expression = MUnaryExpression::unwrap_cast(parent);
                        unary_expression.argument().ok()
                    }
                    MSyntaxKind::M_ASSIGNMENT_EXPRESSION => {
                        let assignment_expression = MAssignmentExpression::unwrap_cast(parent);
                        assignment_expression.right().ok()
                    }
                    _ => None,
                };

                argument == Some(expression)
            }
        }
    }

    /// Returns `true` if this is the root conditional expression and the parent is a [MStaticMemberExpression].
    fn is_parent_static_member_expression(&self, layout: &ConditionalLayout) -> bool {
        match layout {
            ConditionalLayout::Root {
                parent: Some(parent),
                ..
            } => MStaticMemberExpression::can_cast(parent.kind()),
            _ => false,
        }
    }
}

/// Formats the test conditional of a conditional expression.
struct FormatConditionalTest<'a> {
    conditional: &'a MConditionalExpression,
    layout: &'a ConditionalLayout,
}

impl Format<MFormatContext> for FormatConditionalTest<'_> {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        let indent_style = f.options().indent_style();
        let format_inner = format_with(|f| write!(f, [self.conditional.test().format()]));

        if self.layout.is_nested_alternate() {
            if indent_style.is_space() {
                write!(f, [align(2, &format_inner)])
            } else {
                write!(f, [indent(&format_inner)])
            }
        } else {
            format_inner.fmt(f)
        }
    }
}

impl Format<MFormatContext> for AnyMExpression {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        self.format().fmt(f)
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum ConditionalLayout {
    /// Conditional that is the `alternate` of another conditional.
    ///
    /// The `test` condition of a nested alternated is aligned with the parent's `:`.
    ///
    /// ```javascript
    /// outerCondition
    ///     ? consequent
    ///     : nestedAlternate +
    ///       binary + // <- notice how the content is aligned to the `: `
    ///     ? consequentOfnestedAlternate
    ///     : alternateOfNestedAlternate;
    /// ```
    NestedAlternate { parent: MConditionalExpression },

    /// Conditional that is the `test` of another conditional.
    ///
    /// ```javascript
    /// (
    ///     a              // <-- Note the extra indent here
    ///         ? b
    ///         : c
    ///  )
    ///     ? d
    ///     : e;
    /// ```
    ///
    /// Indents the
    NestedTest { parent: MConditionalExpression },

    /// Conditional that is the `consequent` of another conditional.
    ///
    /// ```javascript
    /// condition1
    ///     ? condition2
    ///         ? consequent2 // <-- consequent and alternate gets indented
    ///         : alternate2
    ///     : alternate1;
    /// ```
    NestedConsequent { parent: MConditionalExpression },

    /// This conditional isn't a child of another conditional.
    ///
    /// ```javascript
    /// return a ? b : c;
    /// ```
    Root {
        /// The closest ancestor that isn't a parenthesized node.
        parent: Option<MSyntaxNode>,
    },
}

impl ConditionalLayout {
    const fn is_root(&self) -> bool {
        matches!(self, ConditionalLayout::Root { .. })
    }

    /// Returns the parent node, if any
    fn parent(&self) -> Option<&MSyntaxNode> {
        match self {
            ConditionalLayout::NestedAlternate { parent, .. }
            | ConditionalLayout::NestedTest { parent, .. }
            | ConditionalLayout::NestedConsequent { parent, .. } => Some(parent.syntax()),
            ConditionalLayout::Root { parent, .. } => parent.as_ref(),
        }
    }

    const fn is_nested_test(&self) -> bool {
        matches!(self, ConditionalLayout::NestedTest { .. })
    }

    const fn is_nested_alternate(&self) -> bool {
        matches!(self, ConditionalLayout::NestedAlternate { .. })
    }

    const fn is_nested_consequent(&self) -> bool {
        matches!(self, ConditionalLayout::NestedConsequent { .. })
    }
}
