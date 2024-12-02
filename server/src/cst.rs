pub use chumsky::span::SimpleSpan;

use crate::lexer::KwLang;
pub type Span = SimpleSpan<usize>;
pub type Spanned<T> = (T, Span);

// Expressions
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value<'source> {
    Null(&'source str),
    Bool(&'source str),
    Num(&'source str),
    Str(&'source str),
    LongStr(&'source str),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BinaryOp<'source> {
    Add,
    Sub,
    Mul,
    Div,
    AddEq,
    SubEq,
    MulEq,
    DivEq,
    Eq,
    NotEq,
    Gt,
    Lt,
    GtEq,
    LtEq,
    And(&'source str),
    Or(&'source str),
    BitAnd,
    BitOr,
    Mod,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnaryOp {
    Add,
    Sub,
    Not,
    Minus,
}

#[derive(PartialEq, Eq)]
pub enum Expr<'source> {
    Error,
    Value(Value<'source>),
    Ident(&'source str),
    Call(Box<Spanned<Self>>, Spanned<Vec<Spanned<Self>>>),
    Binary(Box<Spanned<Self>>, BinaryOp<'source>, Box<Spanned<Self>>),
    UnaryRight(Box<Spanned<Self>>, UnaryOp),
    UnaryLeft(UnaryOp, Box<Spanned<Self>>),
    Parentheses(Box<Spanned<Self>>),
    Arr(Vec<Spanned<Self>>),
    Set(Vec<Spanned<Self>>),
    Obj(Vec<(&'source str, Spanned<Self>)>),
    Then(Box<Spanned<Self>>, Box<Spanned<Self>>),
    ThenEquals(Box<Spanned<Self>>, Box<Spanned<Self>>),
    Ternary(
        Box<Spanned<Expr<'source>>>,
        Box<Spanned<Self>>,
        Box<Spanned<Self>>,
    ),
    IndexKey(Box<Spanned<Self>>, Vec<Spanned<Self>>),
}

impl<'source> std::fmt::Debug for Expr<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Expr::Error => f.write_str("Error"),
            Expr::Value(f0) => write!(f, "Value({f0:?})"),
            Expr::Ident(f0) => write!(f, "Ident({f0})"),
            Expr::Call(f0, f1) => write!(f, "Call({f0:?}, {f1:?})"),
            Expr::Binary(f0, f1, f2) => write!(f, "Binary({f0:?}, {f1:?}, {f2:?})"),
            Expr::UnaryRight(f0, f1) => write!(f, "UnaryRight({f0:?}, {f1:?})"),
            Expr::UnaryLeft(f0, f1) => write!(f, "UnaryLeft({f0:?}, {f1:?})"),
            Expr::Parentheses(f0) => write!(f, "Parentheses({f0:?})"),
            Expr::Arr(f0) => write!(f, "Arr({f0:?})"),
            Expr::Set(f0) => write!(f, "Set({f0:?})"),
            Expr::Obj(f0) => write!(f, "Obj({f0:?})"),
            Expr::Then(f0, f1) => write!(f, "Then({f0:?}, {f1:?})"),
            Expr::ThenEquals(f0, f1) => write!(f, "ThenEq({f0:?} = {f1:?})"),
            Expr::Ternary(f0, f1, f2) => write!(f, "Ternary({f0:?} ? {f1:?} : {f2:?})"),
            Expr::IndexKey(f0, f1) => write!(f, "KeyValue({f0:?} {f1:?})"),
        }
    }
}

// Statements
#[derive(PartialEq, Eq)]
pub enum Stmt<'source> {
    Error(Spanned<&'source str>),
    EmptyLine,
    Comment(Spanned<&'source str>),
    Expr(Spanned<Expr<'source>>),
    Var(Option<KwLang>, &'source str, Option<Spanned<Expr<'source>>>),
    Ret(KwLang, Option<Spanned<Expr<'source>>>),
    InlineComment(Box<Self>, Spanned<&'source str>),
    Throw(KwLang, Option<Spanned<Expr<'source>>>),
    Block(Vec<Self>),
    If(KwLang, Spanned<Expr<'source>>, Box<Self>, Option<Box<Self>>),
    While(KwLang, Spanned<Expr<'source>>, Box<Self>),
    ForAll(KwLang, &'source str, Spanned<Expr<'source>>, Box<Self>),
    ForAll2(
        KwLang,
        &'source str,
        Spanned<Expr<'source>>,
        &'source str,
        Box<Self>,
    ),
    For(
        KwLang,
        Box<Self>,
        Spanned<Expr<'source>>,
        Box<Self>,
        Box<Self>,
    ),
    Switch(
        KwLang,
        Spanned<Expr<'source>>,
        Vec<(Option<Vec<Spanned<Expr<'source>>>>, Box<Self>)>,
    ),
    TryCatch(
        KwLang,
        Box<Self>,
        Option<(Option<Spanned<Expr<'source>>>, Box<Self>)>,
        Option<Box<Self>>,
    ),
}

impl<'source> std::fmt::Debug for Stmt<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Stmt::Error(f0) => write!(f, "Error({f0:?})"),
            Stmt::EmptyLine => write!(f, "EmptyLine"),
            Stmt::Expr(f0) => write!(f, "Expr({f0:?})"),
            Stmt::Comment(f0) => write!(f, "Comment({f0:?})"),
            Stmt::Var(f0, f1, f2) => write!(f, "Var({f0:?}, {f1}, {f2:?})"),
            Stmt::Ret(f0, f1) => write!(f, "Ret({f0:?}, {f1:?})"),
            Stmt::InlineComment(f0, f1) => write!(f, "InlineComment({f0:?}, {f1:?})"),
            Stmt::Throw(f0, f1) => write!(f, "Throw({f0:?}, {f1:?})"),
            Stmt::Block(f0) => {
                f.write_str("Block(")?;
                f.debug_list().entries(f0).finish()?;
                f.write_str(")")
            }
            Stmt::If(f0, f1, f2, f3) => f
                .debug_tuple("If")
                .field(&f0)
                .field(&f1)
                .field(&f2)
                .field(&f3)
                .finish(),
            Stmt::While(f0, f1, f2) => f
                .debug_tuple("While")
                .field(&f0)
                .field(&f1)
                .field(&f2)
                .finish(),
            Stmt::ForAll(f0, f1, f2, f3) => f
                .debug_tuple("ForAll")
                .field(&f0)
                .field(&f1)
                .field(&f2)
                .field(&f3)
                .finish(),
            Stmt::ForAll2(f0, f1, f2, f3, f4) => f
                .debug_tuple("ForAll2")
                .field(&f0)
                .field(&f1)
                .field(&f2)
                .field(&f3)
                .field(&f4)
                .finish(),
            Stmt::For(f0, f1, f2, f3, f4) => f
                .debug_tuple("For")
                .field(&f0)
                .field(&f1)
                .field(&f2)
                .field(&f3)
                .field(&f4)
                .finish(),
            Stmt::Switch(f0, f1, f2) => f
                .debug_tuple("Switch")
                .field(&f0)
                .field(&f1)
                .field(&f2)
                .finish(),
            Stmt::TryCatch(f0, f1, f2, f3) => f
                .debug_tuple("TryCatch")
                .field(&f0)
                .field(&f1)
                .field(&f2)
                .field(&f3)
                .finish(),
        }
    }
}

// Declarations
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Parameter<'source> {
    pub identifier: &'source str,
    pub question_mark: bool,
    pub initializer: Option<Expr<'source>>,
}

impl<'source> std::fmt::Display for Parameter<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.identifier)?;
        if self.question_mark || self.initializer.is_some() {
            write!(f, "?")?;
        }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub enum MethodType {
    #[default]
    Func,
    Getter,
    Setter,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Method<'source> {
    pub m_type: MethodType,
    pub identifier: Spanned<Vec<&'source str>>,
    pub params: (Vec<Parameter<'source>>, Span, Option<&'source str>),
    pub body: Spanned<Vec<Stmt<'source>>>,
    pub descr: Option<Vec<&'source str>>,
    pub doc_string: Option<Vec<&'source str>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Decl<'source> {
    Error,
    Stmt(Spanned<Stmt<'source>>), // any stmt between declarations
    Func {
        lang: KwLang,
        identifier: Spanned<Vec<&'source str>>,
        params: (Vec<Parameter<'source>>, Span, Option<&'source str>),
        body: Spanned<Vec<Stmt<'source>>>,
        descr: Option<Vec<&'source str>>,
        doc_string: Option<Vec<&'source str>>,
    },
    Class {
        lang: KwLang,
        identifier: Spanned<Vec<&'source str>>,
        extends: Option<&'source str>,
        methods: Spanned<Vec<Method<'source>>>,
        descr: Option<Vec<&'source str>>,
        doc_string: Option<Vec<&'source str>>,
    },
}
