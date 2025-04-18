use crate::formatter::prelude::*;

use crate::syntax::MSetterClassMember;
use crate::syntax::MSetterClassMemberFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMSetterClassMember;
impl_format_with_rule!(MSetterClassMember, FormatMSetterClassMember);

impl FormatNodeRule<MSetterClassMember> for FormatMSetterClassMember {
    fn fmt_fields(&self, node: &MSetterClassMember, f: &mut MFormatter) -> FormatResult<()> {
        let MSetterClassMemberFields {
            annotation,
            set_token,
            name,
            l_paren_token,
            parameter,
            comma_token,
            r_paren_token,
            doc_string,
            body,
        } = node.as_fields();

        write![
            f,
            [
                annotation.format(),
                set_token.format(),
                space(),
                name.format(),
                l_paren_token.format(),
                parameter.format(),
                comma_token.format(),
                r_paren_token.format(),
                doc_string.format(),
                body.format(),
            ]
        ]
    }
}
