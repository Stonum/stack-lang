use crate::prelude::*;
use biome_formatter::write;

use crate::utils::FormatInitializerClause;

use super::parameters::{should_hug_function_parameters, FormatAnyMParameters};
use mlang_syntax::MFormalParameter;
use mlang_syntax::MFormalParameterFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMFormalParameter;
impl_format_with_rule!(MFormalParameter, FormatMFormalParameter);

impl FormatNodeRule<MFormalParameter> for FormatMFormalParameter {
    fn fmt_fields(&self, node: &MFormalParameter, f: &mut MFormatter) -> FormatResult<()> {
        let MFormalParameterFields {
            binding,
            initializer,
        } = node.as_fields();

        let content = format_with(|f| write![f, [binding.format(),]]);

        let is_hug_parameter = node
            .syntax()
            .grand_parent()
            .and_then(FormatAnyMParameters::cast).is_some_and(|parameters| {
                should_hug_function_parameters(&parameters, f.comments(), false).unwrap_or(false)
            });

        if is_hug_parameter {
            write![f, [content]]?;
        } else {
            write![f, [group(&content)]]?;
        }

        write![f, [FormatInitializerClause::new(initializer.as_ref())]]
    }
}
