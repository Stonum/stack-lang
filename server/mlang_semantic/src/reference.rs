use biome_rowan::{AstNode, AstSeparatedList};

use line_index::{LineColRange, LineIndex};

use mlang_lsp_definition::{LocationDefinition, SemanticInfo};
use mlang_syntax::{
    AnyMExpression, MCallExpression, MClassDeclaration, MNewExpression, MSyntaxKind, MSyntaxNode,
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
    if let Some(call_expression) = MCallExpression::cast(node.clone()) {
        let callee = call_expression.callee().ok()?;
        let args_count = {
            let args = call_expression.arguments().ok()?.args();
            Some(args.len())
        }
        .unwrap_or_default();

        if let AnyMExpression::MStaticMemberExpression(static_member) = callee {
            let object = static_member.object().ok()?;
            let member = static_member.member().ok()?;

            let name = member.text();
            let range = index.line_col_range(member.range())?;

            let class_id = {
                match object {
                    AnyMExpression::MThisExpression(_) => static_member
                        .syntax()
                        .ancestors()
                        .find(|p| p.kind() == MSyntaxKind::M_CLASS_DECLARATION)
                        .and_then(|class_node| {
                            let class = MClassDeclaration::cast(class_node)?;
                            let id = class.id().ok()?.text();
                            Some(id)
                        }),
                    _ => None,
                }
            };
            return Some((
                SemanticInfo::MethodCall(name, args_count, class_id),
                MReferenceLocation(range),
            ));
        }

        if let AnyMExpression::MIdentifierExpression(ident) = callee {
            let name = ident.name().ok()?.text();
            let range = index.line_col_range(ident.range())?;
            return Some((
                SemanticInfo::FunctionCall(name, args_count),
                MReferenceLocation(range),
            ));
        }
    }

    if let Some(new_expression) = MNewExpression::cast(node.clone()) {
        let class = new_expression.callee().ok()?;
        let args_count = {
            let args = new_expression.arguments()?.args();
            Some(args.len())
        }
        .unwrap_or_default();
        if let AnyMExpression::MIdentifierExpression(ident) = class {
            let name = ident.name().ok()?.text();
            let range = index.line_col_range(ident.range())?;
            return Some((
                SemanticInfo::NewExpression(Some(name), args_count),
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
            .get(&SemanticInfo::FunctionCall("fcall".to_string(), 1))
            .unwrap();
        assert_eq!(fcall[0].0, line_col_range(0, 8, 0, 13));
        assert_eq!(fcall[1].0, line_col_range(0, 14, 0, 19));
    }

    #[test]
    fn test_reference_function_call_with_different_args() {
        let model = parse_refs("var x = fcall(1); var z = fcall(1, 2);");
        let reference = model.references();
        assert_eq!(reference.len(), 2);

        let fcall = reference
            .get(&SemanticInfo::FunctionCall("fcall".to_string(), 1))
            .unwrap();
        assert_eq!(fcall[0].0, line_col_range(0, 8, 0, 13));

        let fcall = reference
            .get(&SemanticInfo::FunctionCall("fcall".to_string(), 2))
            .unwrap();
        assert_eq!(fcall[0].0, line_col_range(0, 26, 0, 31));
    }

    #[test]
    fn test_reference_method_call() {
        let model = parse_refs("var x = z.fcall(z.fcall(1))");
        let reference = model.references();
        assert_eq!(reference.len(), 1);

        let fcall = reference
            .get(&SemanticInfo::MethodCall("fcall".to_string(), 1, None))
            .unwrap();
        assert_eq!(fcall[0].0, line_col_range(0, 10, 0, 15));
        assert_eq!(fcall[1].0, line_col_range(0, 18, 0, 23));
    }

    #[test]
    fn test_reference_this_method_call() {
        let model = parse_refs("class x { calls() { var x = this.fcall(this.fcall(1)); } }");
        let reference = model.references();
        assert_eq!(reference.len(), 1);

        let fcall = reference
            .get(&SemanticInfo::MethodCall(
                "fcall".to_string(),
                1,
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
            .get(&SemanticInfo::NewExpression(Some("x".to_string()), 0))
            .unwrap();
        assert_eq!(x[0].0, line_col_range(0, 12, 0, 13));
    }
}
