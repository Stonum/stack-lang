//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::formatter::prelude::*;
use crate::syntax::AnyMStatement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyMStatement;
impl_format!(AnyMStatement, FormatAnyMStatement);
impl FormatRule<AnyMStatement> for FormatAnyMStatement {
    type Context = MFormatContext;
    fn fmt(&self, node: &AnyMStatement, f: &mut MFormatter) -> FormatResult<()> {
        match node {
            AnyMStatement::MBlockStatement(node) => node.format().fmt(f),
            AnyMStatement::MBogusStatement(node) => node.format().fmt(f),
            AnyMStatement::MBreakStatement(node) => node.format().fmt(f),
            AnyMStatement::MClassDeclaration(node) => node.format().fmt(f),
            AnyMStatement::MContinueStatement(node) => node.format().fmt(f),
            AnyMStatement::MDebugStatement(node) => node.format().fmt(f),
            AnyMStatement::MEmptyStatement(node) => node.format().fmt(f),
            AnyMStatement::MExpressionStatement(node) => node.format().fmt(f),
            AnyMStatement::MForAllInStatement(node) => node.format().fmt(f),
            AnyMStatement::MForAllStatement(node) => node.format().fmt(f),
            AnyMStatement::MForStatement(node) => node.format().fmt(f),
            AnyMStatement::MFunctionDeclaration(node) => node.format().fmt(f),
            AnyMStatement::MIfStatement(node) => node.format().fmt(f),
            AnyMStatement::MReturnStatement(node) => node.format().fmt(f),
            AnyMStatement::MSwitchStatement(node) => node.format().fmt(f),
            AnyMStatement::MThrowStatement(node) => node.format().fmt(f),
            AnyMStatement::MTryFinallyStatement(node) => node.format().fmt(f),
            AnyMStatement::MTryStatement(node) => node.format().fmt(f),
            AnyMStatement::MVariableStatement(node) => node.format().fmt(f),
            AnyMStatement::MWhileStatement(node) => node.format().fmt(f),
        }
    }
}
