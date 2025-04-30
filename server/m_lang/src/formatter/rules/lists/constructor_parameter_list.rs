use crate::formatter::prelude::*;
use crate::formatter::rules::bindings::parameters::ParameterLayout;
use crate::formatter::rules::lists::parameter_list::FormatMAnyParameterList;
use crate::syntax::AnyMParameterList;
use crate::syntax::MConstructorParameterList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMConstructorParameterList;
impl_format!(MConstructorParameterList, FormatMConstructorParameterList);

impl FormatRule<MConstructorParameterList> for FormatMConstructorParameterList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MConstructorParameterList, f: &mut MFormatter) -> FormatResult<()> {
        FormatMAnyParameterList::with_layout(
            &AnyMParameterList::from(node.clone()),
            ParameterLayout::Default,
        )
        .fmt(f)
    }
}
