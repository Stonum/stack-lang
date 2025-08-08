use crate::prelude::*;

use mlang_syntax::MGetterClassMember;
use mlang_syntax::MGetterClassMemberFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMGetterClassMember;
impl_format_with_rule!(MGetterClassMember, FormatMGetterClassMember);

impl FormatNodeRule<MGetterClassMember> for FormatMGetterClassMember {
    fn fmt_fields(&self, node: &MGetterClassMember, f: &mut MFormatter) -> FormatResult<()> {
        let MGetterClassMemberFields {
            annotation,
            get_token,
            name,
            l_paren_token,
            r_paren_token,
            doc_string,
            body,
        } = node.as_fields();

        write![
            f,
            [
                annotation.format(),
                get_token.format(),
                space(),
                name.format(),
                l_paren_token.format(),
                r_paren_token.format(),
                hard_line_break(),
                doc_string.format(),
                hard_line_break(),
                body.format()
            ]
        ]
    }
}
