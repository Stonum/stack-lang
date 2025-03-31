use crate::syntax::{
    MBogusParameter, MCatchDeclaration, MClassDeclaration, MConstructorClassMember,
    MConstructorParameterList, MConstructorParameters, MFormalParameter, MFunctionDeclaration,
    MFunctionExpression, MIdentifierBinding, MMethodClassMember, MParameterList, MParameters,
    MRestParameter, MSetterClassMember, MSyntaxKind, MSyntaxNode, MSyntaxToken,
    MVariableDeclarator,
};
use biome_rowan::{declare_node_union, AstNode, SyntaxResult};

declare_node_union! {
    pub AnyMBindingDeclaration =
        // variable
            MVariableDeclarator
        // parameters
            | MFormalParameter | MRestParameter | MBogusParameter
        // functions
            | MFunctionDeclaration | MFunctionExpression
        // classes
            | MClassDeclaration
        // try/catch
            | MCatchDeclaration
}

impl AnyMBindingDeclaration {
    /// Returns `true` if `self` is a formal parameter, a rest parameter,
    /// a property parameter, or a bogus parameter.
    pub const fn is_parameter_like(&self) -> bool {
        matches!(
            self,
            AnyMBindingDeclaration::MFormalParameter(_)
                | AnyMBindingDeclaration::MRestParameter(_)
                | AnyMBindingDeclaration::MBogusParameter(_)
        )
    }
}

declare_node_union! {
    pub AnyMIdentifierBinding = MIdentifierBinding
}

fn declaration(node: MSyntaxNode) -> Option<AnyMBindingDeclaration> {
    match AnyMBindingDeclaration::cast(node)? {
        AnyMBindingDeclaration::MFormalParameter(parameter) => {
            Some(AnyMBindingDeclaration::MFormalParameter(parameter))
        }
        declaration => Some(declaration),
    }
}

impl AnyMIdentifierBinding {
    pub fn name_token(&self) -> SyntaxResult<MSyntaxToken> {
        match self {
            Self::MIdentifierBinding(binding) => binding.name_token(),
        }
    }

    pub fn declaration(&self) -> Option<AnyMBindingDeclaration> {
        declaration(self.syntax().parent()?)
    }
}

impl MIdentifierBinding {
    /// Navigate upward until the declaration of this binding bypassing all nodes
    /// related to pattern binding.
    pub fn declaration(&self) -> Option<AnyMBindingDeclaration> {
        declaration(self.syntax.parent()?)
    }
}

declare_node_union! {
    pub AnyMParameterParentFunction =
        MFunctionDeclaration
        | MFunctionExpression

        | MConstructorClassMember
        | MMethodClassMember
        | MSetterClassMember
}

fn parent_function(node: &MSyntaxNode) -> Option<AnyMParameterParentFunction> {
    let parent = node.parent()?;

    match parent.kind() {
        MSyntaxKind::M_PARAMETER_LIST => {
            // SAFETY: kind check above
            let parameters = MParameterList::unwrap_cast(parent).parent::<MParameters>()?;
            let parent = parameters.syntax.parent()?;
            AnyMParameterParentFunction::cast(parent)
        }
        MSyntaxKind::M_CONSTRUCTOR_PARAMETER_LIST => {
            // SAFETY: kind check above
            let parameters = MConstructorParameterList::unwrap_cast(parent)
                .parent::<MConstructorParameters>()?;
            let parent = parameters.syntax().parent()?;
            AnyMParameterParentFunction::cast(parent)
        }
        _ => AnyMParameterParentFunction::cast(parent),
    }
}

impl MFormalParameter {
    pub fn parent_function(&self) -> Option<AnyMParameterParentFunction> {
        parent_function(&self.syntax)
    }
}

impl MRestParameter {
    pub fn parent_function(&self) -> Option<AnyMParameterParentFunction> {
        parent_function(&self.syntax)
    }
}
