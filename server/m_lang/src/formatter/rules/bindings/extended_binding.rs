use crate::formatter::prelude::*;
use crate::formatter::MLabels;

use crate::syntax::MExtendedBinding;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMExtendedBinding;
impl_format_with_rule!(MExtendedBinding, FormatMExtendedBinding);

impl FormatNodeRule<MExtendedBinding> for FormatMExtendedBinding {
    fn fmt_fields(&self, node: &MExtendedBinding, f: &mut MFormatter) -> FormatResult<()> {
        let is_member_chain = {
            let mut recording = f.start_recording();
            write!(recording, [node.object().format()])?;

            recording
                .stop()
                .has_label(LabelId::of(MLabels::MemberChain))
        };

        let format_no_break =
            format_with(|f| write!(f, [node.operator_token().format(), node.member().format()]));

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
}
