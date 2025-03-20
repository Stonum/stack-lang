use crate::formatter::prelude::*;

use crate::formatter::utils::object::AnyMMemberName;
use crate::syntax::MMethodClassMember;
use crate::syntax::{
    AnyMClassMemberName, MConstructorClassMember, MConstructorParameters, MFunctionBody,
    MParameters,
};
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
        write!(f, [self.name()])?;

        write!(
            f,
            [group(&format_with(|f| {
                let parameters = self.parameters()?;

                write!(f, [parameters])
            }))]
        )?;

        if let Some(body) = self.body()? {
            write!(f, [space(), body.format()])?;
        }

        Ok(())
    }
}

impl FormatAnyMMethodMember {
    fn name(&self) -> SyntaxResult<AnyMMemberName> {
        Ok(match self {
            FormatAnyMMethodMember::MMethodClassMember(member) => member.name()?.into(),
            FormatAnyMMethodMember::MConstructorClassMember(member) => {
                AnyMMemberName::from(AnyMClassMemberName::from(member.name()?))
            }
        })
    }

    fn parameters(&self) -> SyntaxResult<MethodParameters> {
        Ok(match self {
            FormatAnyMMethodMember::MMethodClassMember(member) => member.parameters()?.into(),
            FormatAnyMMethodMember::MConstructorClassMember(member) => member.parameters()?.into(),
        })
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

impl MethodParameters {
    pub fn len(&self) -> usize {
        match self {
            MethodParameters::MParameters(parameters) => parameters.items().len(),
            MethodParameters::MConstructorParameters(parameters) => parameters.parameters().len(),
        }
    }
}

impl Format<MFormatContext> for MethodParameters {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        match self {
            MethodParameters::MParameters(parameters) => parameters.format().fmt(f),
            MethodParameters::MConstructorParameters(parameters) => parameters.format().fmt(f),
        }
    }
}
