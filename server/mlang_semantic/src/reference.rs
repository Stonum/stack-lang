use biome_rowan::AstNode;

use line_index::{LineColRange, LineIndex};

use mlang_lsp_definition::{LocationDefinition, SemanticInfo};
use mlang_syntax::{
    AnyMExpression, MCallExpression, MClassDeclaration, MNewExpression, MStaticMemberExpression,
    MSyntaxKind, MSyntaxNode,
};

#[derive(Debug)]
pub struct MReferenceLocation(line_index::LineColRange);
impl LocationDefinition for MReferenceLocation {
    fn range(&self) -> LineColRange {
        self.0
    }
}

pub(crate) fn get_reference(
    index: &LineIndex,
    node: &MSyntaxNode,
) -> Option<(SemanticInfo, MReferenceLocation)> {
    if let Some(method) = MStaticMemberExpression::cast(node.clone()) {
        let object = method.object().ok()?;
        let member = method.member().ok()?;

        let name = member.text();
        let range = index.line_col_range(member.range())?;

        if let AnyMExpression::MThisExpression(_) = object {
            let class_id = method
                .syntax()
                .ancestors()
                .find(|p| p.kind() == MSyntaxKind::M_CLASS_DECLARATION)
                .and_then(|class_node| {
                    let class = MClassDeclaration::cast(class_node)?;
                    let id = class.id().ok()?.text();
                    Some(id)
                });
            return Some((
                SemanticInfo::MethodCall(name, class_id),
                MReferenceLocation(range),
            ));
        }

        return Some((
            SemanticInfo::MethodCall(name, None),
            MReferenceLocation(range),
        ));
    }

    if let Some(call_expression) = MCallExpression::cast(node.clone()) {
        let callee = call_expression.callee().ok()?;
        if let AnyMExpression::MIdentifierExpression(ident) = callee {
            let name = ident.name().ok()?.text();
            let range = index.line_col_range(ident.range())?;
            return Some((SemanticInfo::FunctionCall(name), MReferenceLocation(range)));
        }
    }

    if let Some(new_expression) = MNewExpression::cast(node.clone()) {
        let class = new_expression.callee().ok()?;
        if let AnyMExpression::MIdentifierExpression(ident) = class {
            let name = ident.name().ok()?.text();
            let range = index.line_col_range(ident.range())?;
            return Some((
                SemanticInfo::NewExpression(Some(name)),
                MReferenceLocation(range),
            ));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use line_index::LineCol;
    use mlang_parser::parse;
    use mlang_syntax::MFileSource;

    use super::*;
    use crate::{SemanticModel, semantics};

    fn line_col_range(
        start_line: u32,
        start_col: u32,
        end_line: u32,
        end_col: u32,
    ) -> LineColRange {
        LineColRange {
            start: LineCol {
                line: start_line,
                col: start_col,
            },
            end: LineCol {
                line: end_line,
                col: end_col,
            },
        }
    }

    fn parse_refs(text: &str) -> SemanticModel {
        let file_source = MFileSource::module();
        let parsed = parse(text, file_source);

        semantics(text, parsed.syntax(), file_source)
    }

    #[test]
    fn test_reference_function_call() {
        let model = parse_refs("var x = fcall(fcall(1))");
        let reference = model.references();
        assert_eq!(reference.len(), 1);

        let fcall = reference
            .get(&SemanticInfo::FunctionCall("fcall".to_string()))
            .unwrap();
        assert_eq!(fcall[0].0, line_col_range(0, 8, 0, 13));
        assert_eq!(fcall[1].0, line_col_range(0, 14, 0, 19));
    }

    #[test]
    fn test_reference_method_call() {
        let model = parse_refs("var x = z.fcall(z.fcall(1))");
        let reference = model.references();
        assert_eq!(reference.len(), 1);

        let fcall = reference
            .get(&SemanticInfo::MethodCall("fcall".to_string(), None))
            .unwrap();
        assert_eq!(fcall[0].0, line_col_range(0, 10, 0, 15));
        assert_eq!(fcall[1].0, line_col_range(0, 18, 0, 23));
    }

    #[test]
    fn test_reference_this_method_call() {
        let model = parse_refs("class x { calls() { var x = this.fcall(this.fcall(1)) } }");
        let reference = model.references();
        assert_eq!(reference.len(), 1);

        let fcall = reference
            .get(&SemanticInfo::MethodCall(
                "fcall".to_string(),
                Some("x".to_string()),
            ))
            .unwrap();
        assert_eq!(fcall[0].0, line_col_range(0, 33, 0, 38));
        assert_eq!(fcall[1].0, line_col_range(0, 44, 0, 49));
    }

    #[test]
    fn test_reference_new_call() {
        let model = parse_refs("var x = new x()");
        let reference = model.references();
        assert_eq!(reference.len(), 1);

        let x = reference
            .get(&SemanticInfo::NewExpression(Some("x".to_string())))
            .unwrap();
        assert_eq!(x[0].0, line_col_range(0, 12, 0, 13));
    }
}
