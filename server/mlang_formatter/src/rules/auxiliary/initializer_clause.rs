use crate::prelude::*;

use crate::utils::{AssignmentLikeLayout, with_assignment_layout};
use biome_formatter::{FormatRuleWithOptions, write};
use mlang_syntax::MInitializerClause;
use mlang_syntax::MInitializerClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMInitializerClause {
    assignment_layout: Option<AssignmentLikeLayout>,
}
impl_format_with_rule!(MInitializerClause, FormatMInitializerClause);

#[derive(Default, Debug)]
pub(crate) struct FormatMInitializerClauseOptions {
    pub(crate) assignment_layout: Option<AssignmentLikeLayout>,
}

impl FormatRuleWithOptions<MInitializerClause> for FormatMInitializerClause {
    type Options = FormatMInitializerClauseOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.assignment_layout = options.assignment_layout;
        self
    }
}

impl FormatNodeRule<MInitializerClause> for FormatMInitializerClause {
    fn fmt_fields(&self, node: &MInitializerClause, f: &mut MFormatter) -> FormatResult<()> {
        let MInitializerClauseFields {
            eq_token,
            expression,
        } = node.as_fields();

        write![
            f,
            [
                eq_token.format(),
                space(),
                with_assignment_layout(&expression?)
            ]
        ]
    }
}
