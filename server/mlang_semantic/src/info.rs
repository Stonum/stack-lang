use biome_rowan::{AstNode, SyntaxNode, SyntaxToken, TextRange, TextSize};
use mlang_lsp_definition::SemanticInfo;
use mlang_syntax::{MClassDeclaration, MLanguage, MSyntaxKind};

pub fn identifier_for_offset(
    root: SyntaxNode<MLanguage>,
    offset: TextSize,
) -> Option<SemanticInfo> {
    // checking the boundaries if cursor is at the start or end token
    let offsets = [
        offset,
        offset.checked_add(1.into()).unwrap_or_default(),
        offset.checked_sub(1.into()).unwrap_or_default(),
    ];

    for offset in offsets {
        let range = TextRange::new(offset, offset);
        if !root.text_range().contains_range(range) {
            continue;
        }
        let node  = root.covering_element(range);
        let token = node.as_token();
        if token.is_none() {
            return None;
        }
        let token = token.unwrap();
        match identifier_for_token(token) {
            Some(info) => {return Some(info);}
            None => {}
        }
    }
    None
}

fn identifier_for_token(token: &SyntaxToken<MLanguage> )
 -> Option<SemanticInfo>
{
    if token.kind() == MSyntaxKind::IDENT {
        let ident = token.text_trimmed().trim().to_string();
        if let Some(node) = token.parent() {
            if node.kind() == MSyntaxKind::M_REFERENCE_IDENTIFIER {
                match find_definition_by_referense(node) {
                    Some (assign_node) => {}
                    None => {}
                }
            }
            // take nearest parents
            for n in token.ancestors().take(3) {
                match n.kind() {
                    MSyntaxKind::M_FUNCTION_DECLARATION => {
                        return Some(SemanticInfo::FunctionDeclaration(ident));
                    }
                    MSyntaxKind::M_CLASS_DECLARATION => {
                        return Some(SemanticInfo::ClassDeclaration(ident));
                    }
                    MSyntaxKind::M_METHOD_CLASS_MEMBER => {
                        let class_member_list_node = n.parent()?;
                        let class_node = class_member_list_node.parent()?;

                        let class = MClassDeclaration::cast(class_node)?;
                        let class_id = class.id().ok()?.text();

                        return Some(SemanticInfo::MethodDeclaration(ident, class_id));
                    }
                    MSyntaxKind::M_STATIC_MEMBER_EXPRESSION => {
                        if let Some(child) = n.first_child() {
                            // try find class name
                            if child.kind() == MSyntaxKind::M_THIS_EXPRESSION {
                                let class_id = token
                                    .ancestors()
                                    .find(|p| p.kind() == MSyntaxKind::M_CLASS_DECLARATION)
                                    .and_then(|class_node| {
                                        let class = MClassDeclaration::cast(class_node)?;
                                        let id = class.id().ok()?.text();
                                        Some(id)
                                    });
                                return Some(SemanticInfo::MethodCall(ident, class_id));
                            }
                        }
                        return Some(SemanticInfo::MethodCall(ident, None));
                    }
                    MSyntaxKind::M_NEW_EXPRESSION => {
                        return Some(SemanticInfo::NewExpression(ident));
                    }
                    MSyntaxKind::M_CALL_EXPRESSION => {
                        return Some(SemanticInfo::FunctionCall(ident));
                    }
                    MSyntaxKind::M_EXTENDS_CLAUSE => {
                        return Some(SemanticInfo::ClassExtends(ident));
                    }
                    MSyntaxKind::M_FOR_ITERATOR_FACTORY => {
                        return Some(SemanticInfo::FunctionCall(ident));
                    }
                    _ => (),
                };
            }
        }
    }

    if token.kind() == MSyntaxKind::SUPER_KW {
        let super_class_id = token
            .ancestors()
            .find(|p| p.kind() == MSyntaxKind::M_CLASS_DECLARATION)
            .and_then(|class_node| {
                let class = MClassDeclaration::cast(class_node)?;
                let id = class.extends_clause()?.super_class().ok()?.text();
                Some(id)
            });
        if let Some(super_class) = super_class_id {
            let info =
                SemanticInfo::SuperCall(token.text_trimmed().trim().to_string(), super_class);
            return Some(info);
        }
    }

    if token.kind() == MSyntaxKind::THIS_KW {
        let class_id = token
            .ancestors()
            .find(|p| p.kind() == MSyntaxKind::M_CLASS_DECLARATION)
            .and_then(|class_node| {
                let class = MClassDeclaration::cast(class_node)?;
                let class_name = class.id().ok()?.text();
                Some(class_name)
            });
        if let Some(class_id) = class_id {
            let info =
                SemanticInfo::ThisCall(token.text_trimmed().trim().to_string(), class_id);
            return Some(info);
        }
    }
    None
}

fn find_definition_by_referense(node: SyntaxNode<MLanguage>) -> Option<SyntaxNode<MLanguage>> {
    if let Some(parent) = node.grand_parent() {
        if let Some(parent) = parent.grand_parent() {
            if let Some(parent) = parent.grand_parent() {
                if let Some(parent) = parent.grand_parent() {
        let parents = parent
            .siblings_with_tokens(biome_rowan::Direction::Prev)
            .filter_map(|e| {
                e.into_node()
            })
            .map(|_t| {
            let token_kind = _t.kind();
            let token_text = _t.text().to_string();
            (token_kind,token_text)
        }).collect::<Vec<_>>().clone();
        
        for p in parents {
            // let node_is = node;
            let c = p;
        }
    }
}
}}
    // если мы ничего не нашли и если исходный узел - это идентификатор
    // место определения не может содержаться в узле идентификатора
    // поэтому переходим выше к родительскому узлу и идем по элементам дерева в обратном направлении
    if let Some(node) = node.parent() {
        let mut parent: SyntaxNode<MLanguage> = node;
        while true {
            match parent.parent() {
                Some(n) => {parent = n;}
                None => {break;}
            }
            let parents = parent
                .siblings_with_tokens(biome_rowan::Direction::Prev)
                .skip(1)
                .filter_map(|e| {e.into_node()})
                .filter(|n| n.kind() == MSyntaxKind::M_EXPRESSION_STATEMENT)
                .collect::<Vec<_>>().clone();
        }
    }
    // дойдя до конца перейдем к уже его родителю и повторим процедуру поиска вверх по дереву.
    // M_STATEMENT_LIST "a = new cl(); while(true){a . t}"
    // M_EXPRESSION_STATEMENT "a = new cl();"
    // M_ASSIGNMENT_EXPRESSION "a = new cl()"
    // M_IDENTIFIER_ASSIGNMENT "a "
    None
}

#[cfg(test)]
mod tests {
    use mlang_parser::parse;
    use mlang_syntax::MFileSource;

    use super::*;

    #[test]
    fn test_identifier_for_offset() {
        #[rustfmt::skip]
        let inputs = [
            ("var x = callFunction()", 15, SemanticInfo::FunctionCall("callFunction".to_owned())),
            ("var x = z.callMethod()", 15, SemanticInfo::MethodCall("callMethod".to_owned(), None)),
            ("var x = new TodoClass()",15, SemanticInfo::NewExpression("TodoClass".to_owned())),
            ("var x = callFunction( z.callMethod() )", 30, SemanticInfo::MethodCall("callMethod".to_owned(), None)),
            ("var x = z.callMethod( callFunction() )", 30, SemanticInfo::FunctionCall("callFunction".to_owned())),
            ("var x = z.callMethod( new TodoClass() )",30, SemanticInfo::NewExpression("TodoClass".to_owned())),
            ("#comment line
              callaFterComment()",30, SemanticInfo::FunctionCall("callaFterComment".to_owned())),
            ("class B extends A {}", 17, SemanticInfo::ClassExtends("A".to_owned())),
            ("class B extends A { constructor() { super() } }", 40, SemanticInfo::SuperCall("super".to_owned(), "A".to_owned())),
            ("forall( iterator(arr, ind)) {}", 15, SemanticInfo::FunctionCall("iterator".to_owned()))
        ];

        for (input, offset, info) in inputs {
            let parsed = parse(input, MFileSource::script());
            let semantic_info =
                identifier_for_offset(parsed.syntax(), TextSize::from(offset)).unwrap();
            assert_eq!(info, semantic_info, "{input}");
        }
    }

    #[test]
    fn test_identifier_for_offset2() {
        let input = r#"
            class Test {
                constructor() { this.m2(); }
                m1() {}
                m2() { this.m1(); }
            }
        "#;
        let parsed = parse(input, MFileSource::script());

        let offsets = [
            (
                65,
                SemanticInfo::MethodCall("m2".to_owned(), Some("Test".into())),
            ),
            (
                125,
                SemanticInfo::MethodCall("m1".to_owned(), Some("Test".into())),
            ),
        ];

        for (offset, info) in offsets {
            let semantic_info =
                identifier_for_offset(parsed.syntax(), TextSize::from(offset)).unwrap();
            assert_eq!(info, semantic_info);
        }
    }

    #[test]
    fn test_identifier_for_offset3() {
        let input = r#"func f(){a = new cl(); while(true){a . t}}"#;
        let parsed = parse(input, MFileSource::script());
        let semantic_info =
            identifier_for_offset(parsed.syntax(), TextSize::from(36)).unwrap();
    }
}
