//! Extended AST node definitions for statements which are unique and special enough to generate code for manually

use crate::syntax::{
    AnyMSwitchClause, MBlockStatement, MCatchClause, MFinallyClause, MStatementList,
    MSyntaxToken as SyntaxToken, MTryFinallyStatement, MTryStatement,
};
use biome_rowan::{declare_node_union, SyntaxResult};

impl AnyMSwitchClause {
    pub fn clause_token(&self) -> SyntaxResult<SyntaxToken> {
        match &self {
            AnyMSwitchClause::MCaseClause(item) => item.case_token(),
            AnyMSwitchClause::MDefaultClause(item) => item.else_token(),
        }
    }

    pub fn colon_token(&self) -> SyntaxResult<Option<SyntaxToken>> {
        match &self {
            AnyMSwitchClause::MCaseClause(item) => item.colon_token().map(Some),
            AnyMSwitchClause::MDefaultClause(_) => Ok(None),
        }
    }

    pub fn consequent(&self) -> MStatementList {
        match &self {
            AnyMSwitchClause::MCaseClause(item) => item.consequent(),
            AnyMSwitchClause::MDefaultClause(item) => item.consequent(),
        }
    }
}

declare_node_union! {
    pub AnyMTryStatement = MTryStatement | MTryFinallyStatement
}

impl AnyMTryStatement {
    pub fn try_token(&self) -> SyntaxResult<SyntaxToken> {
        match self {
            Self::MTryStatement(node) => node.try_token(),
            Self::MTryFinallyStatement(node) => node.try_token(),
        }
    }

    pub fn body(&self) -> SyntaxResult<MBlockStatement> {
        match self {
            Self::MTryStatement(node) => node.body(),
            Self::MTryFinallyStatement(node) => node.body(),
        }
    }

    pub fn catch_clause(&self) -> Option<MCatchClause> {
        match self {
            Self::MTryStatement(node) => node.catch_clause().ok(),
            Self::MTryFinallyStatement(node) => node.catch_clause(),
        }
    }

    pub fn finally_clause(&self) -> Option<MFinallyClause> {
        match self {
            Self::MTryStatement(_) => None,
            Self::MTryFinallyStatement(node) => node.finally_clause().ok(),
        }
    }
}
