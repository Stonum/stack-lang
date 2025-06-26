use crate::formatter::prelude::*;

use crate::syntax::{AnyMStringLiteralExpression, MClassMemberName, MMethodClassMember};
use crate::syntax::{MConstructorClassMember, MConstructorParameters, MFunctionBody, MParameters};
use biome_formatter::write;
use biome_rowan::{declare_node_union, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMMethodClassMember;
impl_format_with_rule!(MMethodClassMember, FormatMMethodClassMember);

impl FormatNodeRule<MMethodClassMember> for FormatMMethodClassMember {
    fn fmt_fields(&self, node: &MMethodClassMember, f: &mut MFormatter) -> FormatResult<()> {
        write![f, [space(), FormatAnyMMethodMember::from(node.clone())]]
    }
}

declare_node_union! {
    /// Formats the type parameters, parameters, and return type annotation of a method
    pub(crate) FormatAnyMMethodMember =
        MMethodClassMember |
        MConstructorClassMember
}

impl Format<MFormatContext> for FormatAnyMMethodMember {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        write!(f, [self.name().format()])?;

        write!(
            f,
            [group(&format_with(|f| {
                let parameters = self.parameters()?;

                write!(f, [parameters])
            }))]
        )?;

        if let Some(doc_string) = self.doc_string() {
            write!(
                f,
                [hard_line_break(), doc_string.format(), hard_line_break()]
            )?;
        }

        if let Some(body) = self.body()? {
            write!(f, [body.format()])?;
        }

        Ok(())
    }
}

impl FormatAnyMMethodMember {
    fn name(&self) -> SyntaxResult<MClassMemberName> {
        Ok(match self {
            FormatAnyMMethodMember::MMethodClassMember(member) => member.name()?,
            FormatAnyMMethodMember::MConstructorClassMember(member) => member.name()?,
        })
    }

    fn parameters(&self) -> SyntaxResult<MethodParameters> {
        Ok(match self {
            FormatAnyMMethodMember::MMethodClassMember(member) => member.parameters()?.into(),
            FormatAnyMMethodMember::MConstructorClassMember(member) => member.parameters()?.into(),
        })
    }

    fn doc_string(&self) -> Option<AnyMStringLiteralExpression> {
        match self {
            FormatAnyMMethodMember::MMethodClassMember(member) => member.doc_string(),
            FormatAnyMMethodMember::MConstructorClassMember(member) => member.doc_string(),
        }
    }

    fn body(&self) -> SyntaxResult<Option<MFunctionBody>> {
        Ok(match self {
            FormatAnyMMethodMember::MMethodClassMember(member) => Some(member.body()?),
            FormatAnyMMethodMember::MConstructorClassMember(member) => Some(member.body()?),
        })
    }
}

declare_node_union! {
    MethodParameters = MParameters | MConstructorParameters
}

impl Format<MFormatContext> for MethodParameters {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        match self {
            MethodParameters::MParameters(parameters) => parameters.format().fmt(f),
            MethodParameters::MConstructorParameters(parameters) => parameters.format().fmt(f),
        }
    }
}
