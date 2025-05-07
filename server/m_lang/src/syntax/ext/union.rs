use crate::syntax::{
    AnyMClassMember, AnyMClassMemberName, AnyMFunction, AnyMFunctionBinding, AnyMFunctionBody,
    MParameters, MSyntaxToken,
};
use biome_rowan::SyntaxResult;

impl AnyMClassMember {
    /// Returns the `name` of the member if it has any.
    pub fn name(&self) -> SyntaxResult<Option<AnyMClassMemberName>> {
        match self {
            AnyMClassMember::MConstructorClassMember(constructor) => constructor
                .name()
                .map(|name| Some(AnyMClassMemberName::from(name))),
            AnyMClassMember::MEmptyClassMember(_) => Ok(None),
            AnyMClassMember::MGetterClassMember(getter) => getter.name().map(Some),
            AnyMClassMember::MMethodClassMember(method) => method.name().map(Some),
            AnyMClassMember::MSetterClassMember(setter) => setter.name().map(Some),
            AnyMClassMember::MBogusMember(_) => Ok(None),
        }
    }

    /// Tests if the member has a [`MLiteralMemberName`](crate::MLiteralMemberName) of `name`.
    pub fn has_name(&self, name: &str) -> SyntaxResult<bool> {
        match self.name()? {
            Some(AnyMClassMemberName::MLiteralMemberName(literal)) => {
                Ok(literal.value()?.text_trimmed() == name)
            }
            _ => Ok(false),
        }
    }
}

impl AnyMClassMemberName {
    pub const fn is_computed(&self) -> bool {
        matches!(self, AnyMClassMemberName::MComputedMemberName(_))
    }
}

impl AnyMFunction {
    /// Returns the binding by which the function can be accessed.
    ///
    /// This may be a binding for the function's identifier, or a binding for
    /// the variable to which the function is assigned.
    pub fn binding(&self) -> Option<AnyMFunctionBinding> {
        match self {
            AnyMFunction::MFunctionDeclaration(declaration) => declaration.id().ok(),
            AnyMFunction::MFunctionExpression(_) => None,
        }
    }

    pub fn function_token(&self) -> SyntaxResult<Option<MSyntaxToken>> {
        match self {
            AnyMFunction::MFunctionExpression(expr) => expr.function_token().map(Some),
            AnyMFunction::MFunctionDeclaration(declaration) => {
                declaration.function_token().map(Some)
            }
        }
    }

    pub fn id(&self) -> SyntaxResult<Option<AnyMFunctionBinding>> {
        match self {
            AnyMFunction::MFunctionExpression(_) => Ok(None),
            AnyMFunction::MFunctionDeclaration(declaration) => declaration.id().map(Some),
        }
    }

    pub fn parameters(&self) -> SyntaxResult<MParameters> {
        match self {
            AnyMFunction::MFunctionExpression(expr) => expr.parameters(),
            AnyMFunction::MFunctionDeclaration(declaration) => declaration.parameters(),
        }
    }

    pub fn body(&self) -> SyntaxResult<AnyMFunctionBody> {
        match self {
            AnyMFunction::MFunctionExpression(expr) => {
                expr.body().map(AnyMFunctionBody::MFunctionBody)
            }
            AnyMFunction::MFunctionDeclaration(declaration) => {
                declaration.body().map(AnyMFunctionBody::MFunctionBody)
            }
        }
    }
}
