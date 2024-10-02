use super::parser::{Decl, Method};
use super::position;
use ropey::Rope;
use tower_lsp::lsp_types::Range;

#[derive(Debug, PartialEq)]
pub enum Definition {
    Func {
        identifier: String,
        params: Vec<String>,
        descr: Option<Vec<String>>,
        doc_string: Option<String>,
        pos: Range,
    },
    Class {
        identifier: String,
        methods: Vec<Definition>,
        descr: Option<Vec<String>>,
        doc_string: Option<String>,
        pos: Range,
    },
}

impl Definition {
    pub fn try_from_declaration(decl: Decl, rope: &Rope) -> Result<Self, ()> {
        match decl {
            Decl::Error => Err(()),
            Decl::Stmt(_) => Err(()),
            Decl::Func {
                identifier: (identifier, span),
                params,
                descr,
                doc_string,
                ..
            } => Ok(Definition::Func {
                identifier,
                params: params.0.into_iter().map(|x| format!("{x}")).collect(),
                descr,
                doc_string,
                pos: position(rope, span.into()).unwrap_or_default(),
            }),
            Decl::Class {
                identifier: (identifier, span),
                methods: (methods, _),
                descr,
                doc_string,
                ..
            } => Ok(Definition::Class {
                identifier,
                methods: methods
                    .into_iter()
                    .map(|m| Definition::from_method(m, rope))
                    .collect(),
                descr,
                doc_string,
                pos: position(rope, span.into()).unwrap_or_default(),
            }),
        }
    }

    pub fn from_method(method: Method, rope: &Rope) -> Self {
        Definition::Func {
            identifier: method.identifier.0,
            params: method.params.0.into_iter().map(|x| x.identifier).collect(),
            descr: method.descr,
            doc_string: method.doc_string,
            pos: position(rope, method.identifier.1.into()).unwrap_or_default(),
        }
    }

    pub fn identifier(&self) -> &str {
        match self {
            Definition::Func { identifier, .. } => identifier,
            Definition::Class { identifier, .. } => identifier,
        }
    }

    pub fn description(&self) -> String {
        let map = |d: String| {
            if d.starts_with("#") {
                return format!("\\{}", d);
            }
            d
        };
        let descr = match self {
            Definition::Func { descr, .. } => descr,
            Definition::Class { descr, .. } => descr,
        };

        descr
            .clone()
            .unwrap_or_default()
            .into_iter()
            .map(map)
            .fold(String::new(), |a, b| a + "\n" + &b)
    }

    pub fn doc_string(&self) -> &str {
        match self {
            Definition::Func { doc_string, .. } => doc_string.as_deref().unwrap_or_default(),
            Definition::Class { doc_string, .. } => doc_string.as_deref().unwrap_or_default(),
        }
    }

    pub fn position(&self) -> Range {
        match self {
            Definition::Func { pos, .. } => *pos,
            Definition::Class { pos, .. } => *pos,
        }
    }

    pub fn parameters(&self) -> String {
        match self {
            Definition::Func { params, .. } => format!("( {} )", params.join(", ")),
            Definition::Class { .. } => String::new(),
        }
    }
}
