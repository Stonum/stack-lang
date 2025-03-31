use crate::syntax::{
    AnyMCallArgument, AnyMFunction, AnyMFunctionBody, MCallArguments, MMethodClassMember,
    MStatementList, MSyntaxToken,
};
use biome_rowan::{declare_node_union, AstNode, SyntaxResult, TextRange};

declare_node_union! {
    pub AnyFunctionLike = AnyMFunction | MMethodClassMember
}

impl AnyFunctionLike {
    pub fn body(&self) -> SyntaxResult<AnyMFunctionBody> {
        match self {
            AnyFunctionLike::AnyMFunction(m_function) => m_function.body(),
            AnyFunctionLike::MMethodClassMember(m_class_method) => {
                m_class_method.body().map(AnyMFunctionBody::MFunctionBody)
            }
        }
    }

    pub fn function_token(&self) -> Option<MSyntaxToken> {
        match self {
            AnyFunctionLike::AnyMFunction(any_m_function) => {
                any_m_function.function_token().ok().flatten()
            }
            AnyFunctionLike::MMethodClassMember(_) => None,
        }
    }

    pub fn name_range(&self) -> Option<TextRange> {
        match self {
            AnyFunctionLike::AnyMFunction(m_function) => {
                m_function.id().ok().flatten().map(|id| id.range())
            }
            AnyFunctionLike::MMethodClassMember(m_class_method) => {
                m_class_method.name().ok().map(|name| name.range())
            }
        }
    }

    pub fn statements(&self) -> Option<MStatementList> {
        Some(match self {
            AnyFunctionLike::AnyMFunction(any_m_function) => any_m_function
                .body()
                .ok()?
                .as_m_function_body()?
                .statements(),
            AnyFunctionLike::MMethodClassMember(method_class_member) => {
                method_class_member.body().ok()?.statements()
            }
        })
    }
}

impl MCallArguments {
    /// Get [AnyMCallArgument] by its index inside the [crate::MCallExpression] argument list.
    ///
    /// Each index inside `indices` should be unique qnd in-order.
    ///
    /// Supports a maximum of 16 indices to avoid stack overflow.
    pub fn get_arguments_by_index<const N: usize>(
        &self,
        indices: [usize; N],
    ) -> [Option<AnyMCallArgument>; N] {
        debug_assert!(N <= 16);
        // assert there are no duplicates and they are in-order
        debug_assert!(indices.windows(2).all(|vs| vs[0] < vs[1]));

        const INIT: Option<AnyMCallArgument> = None;
        let mut result = [INIT; N];
        let mut next = 0;
        for (i, arg) in self.args().into_iter().flatten().enumerate() {
            if i == indices[next] {
                result[next] = Some(arg);
                next += 1;
                if next == N {
                    break;
                }
            }
        }
        result
    }
}
