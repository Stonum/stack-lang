use crate::{
    factory::make::m_parameter_list,
    syntax::{
        AnyMClassMember, AnyMClassMemberName, AnyMFunction, AnyMFunctionBinding, AnyMFunctionBody,
        AnyMParameter, AnyMParameterList, AnyMStringLiteralExpression, MLanguage, MParameters,
        MSyntaxToken,
    },
};
use biome_rowan::{syntax::SyntaxTrivia, AstNode, SyntaxResult};

impl AnyMClassMember {
    /// Returns the `name` of the member if it has any.
    pub fn name(&self) -> SyntaxResult<Option<AnyMClassMemberName>> {
        match self {
            AnyMClassMember::MConstructorClassMember(constructor) => constructor
                .name()
                .map(|name| Some(AnyMClassMemberName::from(name))),
            AnyMClassMember::MGetterClassMember(getter) => getter.name().map(Some),
            AnyMClassMember::MMethodClassMember(method) => method.name().map(Some),
            AnyMClassMember::MSetterClassMember(setter) => setter.name().map(Some),
            AnyMClassMember::MBogusMember(_) => Ok(None),
        }
    }

    pub fn params(&self) -> SyntaxResult<Option<AnyMParameterList>> {
        match self {
            AnyMClassMember::MConstructorClassMember(constructor) => constructor
                .parameters()
                .map(|p| Some(p.parameters().into())),
            AnyMClassMember::MGetterClassMember(_) => Ok(None),
            AnyMClassMember::MMethodClassMember(method) => {
                method.parameters().map(|p| Some(p.items().into()))
            }
            AnyMClassMember::MSetterClassMember(setter) => setter.parameter().map(|p| {
                let parameters_list = m_parameter_list([AnyMParameter::AnyMFormalParameter(p)], []);
                Some(parameters_list.into())
            }),
            AnyMClassMember::MBogusMember(_) => Ok(None),
        }
    }

    pub fn leading_trivia(&self) -> Option<SyntaxTrivia<MLanguage>> {
        match self {
            AnyMClassMember::MConstructorClassMember(constructor) => {
                constructor.syntax().first_leading_trivia()
            }
            AnyMClassMember::MGetterClassMember(getter) => getter.syntax().first_leading_trivia(),
            AnyMClassMember::MMethodClassMember(method) => method.syntax().first_leading_trivia(),
            AnyMClassMember::MSetterClassMember(setter) => setter.syntax().first_leading_trivia(),
            AnyMClassMember::MBogusMember(_) => None,
        }
    }

    pub fn doc_string(&self) -> Option<AnyMStringLiteralExpression> {
        match self {
            AnyMClassMember::MConstructorClassMember(constructor) => constructor.doc_string(),
            AnyMClassMember::MGetterClassMember(getter) => getter.doc_string(),
            AnyMClassMember::MMethodClassMember(method) => method.doc_string(),
            AnyMClassMember::MSetterClassMember(setter) => setter.doc_string(),
            AnyMClassMember::MBogusMember(_) => None,
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
