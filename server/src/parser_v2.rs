use crate::lexer_v2::Lexer;
use crate::MLangSyntaxKind::*;
use crate::{MLangSyntaxKind, Token, T};
use biome_parser::event::Event;
use biome_parser::prelude::*;
use biome_parser::token_source::Trivia;
use biome_parser::tree_sink::LosslessTreeSink;
use biome_parser::{token_set, Parser, ParserContext, TokenSet};
use biome_rowan::{Language, NodeCache, ParsedChildren, RawNodeSlots, RawSyntaxNode, TextRange, TriviaPieceKind};

pub struct MLangTokenSource<'source> {
    lexer: Lexer<'source>,
    trivia: Vec<Trivia>,
    current: MLangSyntaxKind,
    current_range: TextRange,
    preceding_line_break: bool,
    // options: JsonParserOptions,
}

impl<'source> MLangTokenSource<'source> {
    pub fn from_str(source: &'source str) -> Self {
        let lexer = Lexer::new(source);

        let mut source = Self {
            lexer,
            trivia: Vec::new(),
            current: T![TOMBSTONE],
            current_range: TextRange::default(),
            preceding_line_break: false,
        };

        source.next_non_trivia_token(true);
        source
    }

    fn next_non_trivia_token(&mut self, first_token: bool) {
        let mut trailing = !first_token;
        self.preceding_line_break = false;

        while let Some(token) = self.lexer.next() {
            let trivia_kind = TriviaPieceKind::try_from(token.kind);

            match trivia_kind {
                Err(_) => {
                    self.set_current_token(token);
                    break;
                }
                Ok(trivia_kind) => {
                    if trivia_kind.is_newline() {
                        trailing = false;
                        self.preceding_line_break = true;
                    }

                    self.trivia
                        .push(Trivia::new(trivia_kind, token.range, trailing));
                }
            }
        }
    }

    fn set_current_token(&mut self, token: Token) {
        self.current = token.kind;
        self.current_range = token.range;
    }
}

impl<'source> TokenSource for MLangTokenSource<'source> {
    type Kind = MLangSyntaxKind;

    fn current(&self) -> Self::Kind {
        self.current
    }

    fn current_range(&self) -> TextRange {
        self.current_range
    }

    fn text(&self) -> &str {
        self.lexer.source()
    }

    fn has_preceding_line_break(&self) -> bool {
        self.preceding_line_break
    }

    fn bump(&mut self) {
        if self.current != T![EOF] {
            self.next_non_trivia_token(false)
        }
    }

    fn skip_as_trivia(&mut self) {
        if self.current() != T![EOF] {
            self.trivia.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));

            self.next_non_trivia_token(false)
        }
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia, vec![])
    }
}

pub struct MLangParser<'source> {
    context: ParserContext<MLangSyntaxKind>,
    source: MLangTokenSource<'source>,
    // optional, only if the parser is meant to have some options
    // options: BetaParserOptions,
}

impl<'source> MLangParser<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            context: ParserContext::default(),
            source: MLangTokenSource::from_str(source),
        }
    }

    pub fn finish(
        self,
    ) -> (
        Vec<Event<MLangSyntaxKind>>,
        Vec<ParseDiagnostic>,
        Vec<Trivia>,
    ) {
        let (trivia, _) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();

        (events, parse_diagnostics, trivia)
    }
}

impl<'source> Parser for MLangParser<'source> {
    type Kind = MLangSyntaxKind;
    type Source = MLangTokenSource<'source>;

    fn context(&self) -> &ParserContext<Self::Kind> {
        &self.context
    }

    fn context_mut(&mut self) -> &mut ParserContext<Self::Kind> {
        &mut self.context
    }

    fn source(&self) -> &Self::Source {
        &self.source
    }

    fn source_mut(&mut self) -> &mut Self::Source {
        &mut self.source
    }
}

// pub fn parse(text: &str) -> Parse<AnyStackRoot> {
//     let mut cache = NodeCache::default();
//     parse_mlang_with_cache(text, &mut cache)
// }

// pub fn parse_mlang_with_cache(text: &str, cache: &mut NodeCache) -> Parse<AnyStackRoot> {
//     tracing::debug_span!("parse").in_scope(move || {
//         let (events, errors, tokens) = parse_common(text);
//         let mut tree_sink = JsLosslessTreeSink::with_cache(text, &tokens, cache);
//         biome_parser::event::process(&mut tree_sink, events, errors);
//         let (green, parse_errors) = tree_sink.finish();
//         Parse::new(green, parse_errors)
//     })
// }

fn parse_common(
    text: &str,
) -> (
    Vec<Event<MLangSyntaxKind>>,
    Vec<ParseDiagnostic>,
    Vec<Trivia>,
) {
    let mut p = MLangParser::new(text);
    // let m = p.start();

    parse_declarations(&mut p);

    let (events, errors, trivia) = p.finish();

    (events, errors, trivia)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct MLangLanguage;

impl Language for MLangLanguage {
    type Kind = MLangSyntaxKind;
    type Root = MLangRoot;
}

#[derive(Debug)]
pub struct MLangSyntaxFactory;

pub(crate) type MLangLosslessTreeSink<'source> =
    LosslessTreeSink<'source, MLangLanguage, MLangSyntaxFactory>;

impl SyntaxFactory for MLangSyntaxFactory {
    type Kind = MLangSyntaxKind;

    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            MLANG_NUMBER_LITERAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if element.kind() == Number {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MLANG_NUMBER_LITERAL_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MLANG_NUMBER_LITERAL_EXPRESSION, children)
            }
            MLANG_UNARY_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element {
                    if matches!(element.kind(), T ! [+] | T ! [-] | T![!]) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if let Some(element) = &current_element {
                    if AnyStackExpression::can_cast(element.kind()) {
                        slots.mark_present();
                        current_element = elements.next();
                    }
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        MLANG_UNARY_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(MLANG_UNARY_EXPRESSION, children)
            }
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}

pub enum AnyStackExpression {
    StackUnaryExpression(StackUnaryExpression),
}
pub type MlangSyntaxNode = biome_rowan::SyntaxNode<MLangLanguage>;
impl From<StackUnaryExpression> for AnyStackExpression {
    fn from(node: StackUnaryExpression) -> AnyStackExpression {
        AnyStackExpression::StackUnaryExpression(node)
    }
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct StackUnaryExpression {
    pub(crate) syntax: MlangSyntaxNode,
}

fn parse_declarations(p: &mut MLangParser) {
    let mut m = p.start();

    println!("{:?}", p.cur());

    let value = match parse_unary_expr(p) {
        ParsedSyntax::Present(value) => ParsedSyntax::Present(value),
        ParsedSyntax::Absent => {
            p.error(ParseDiagnostic::new("expected", p.cur_range()));
            ParsedSyntax::Absent
        }
    };

    println!("{:?}", p.cur());

    p.eat(T![EOF]);
    m.complete(p, MLANG_ROOT);
}

fn parse_unary_expr(p: &mut MLangParser) -> ParsedSyntax {
    // let mut m = p.start();

    const UNARY_SINGLE: TokenSet<MLangSyntaxKind> = token_set![T![+], T![-], T![!]];

    if p.at_ts(UNARY_SINGLE) {
        let m = p.start();
        let op = p.cur();

        p.bump_any();

        parse_postfix_expr(p).ok();

        return ParsedSyntax::Present(m.complete(p, MLANG_UNARY_EXPRESSION));
    }

    ParsedSyntax::Absent

    // parse_postfix_expr(p)
}

fn parse_postfix_expr(p: &mut MLangParser) -> ParsedSyntax {
    // let checkpoint = p.checkpoint();
    let lhs = parse_lhs_expr(p);
    lhs.map(|marker| {
        // if !p.has_preceding_line_break() {
        //     // test js post_update_expr
        //     // foo++
        //     // foo--
        //     match p.cur() {
        //         T![++] => {
        //             let assignment_target = expression_to_assignment(p, marker, checkpoint);
        //             let m = assignment_target.precede(p);
        //             p.bump(T![++]);
        //             m.complete(p, JS_POST_UPDATE_EXPRESSION)
        //         }
        //         T![--] => {
        //             let assignment_target = expression_to_assignment(p, marker, checkpoint);
        //             let m = assignment_target.precede(p);
        //             p.bump(T![--]);
        //             m.complete(p, JS_POST_UPDATE_EXPRESSION)
        //         }
        //         _ => marker,
        //     }
        // } else {
        marker
        // }
    })
}

pub fn parse_lhs_expr(p: &mut MLangParser) -> ParsedSyntax {
    // super.foo and super[bar]
    // test js super_property_access
    // super.foo
    // super[bar]
    // super[foo][bar]
    // let lhs = if p.at(T![super]) {
    //     parse_super_expression(p)
    // } else {
    let lhs = parse_primary_expression(p);
    // };

    lhs
}

fn parse_primary_expression(p: &mut MLangParser) -> ParsedSyntax {
    let parsed_literal_expression = parse_literal_expression(p);
    if parsed_literal_expression.is_present() {
        return parsed_literal_expression;
    }

    ParsedSyntax::Absent
}

pub(super) fn parse_literal_expression(p: &mut MLangParser) -> ParsedSyntax {
    let literal_kind = match p.cur() {
        MLangSyntaxKind::Number => {
            return parse_number_literal_expression(p);
        }
        // JsSyntaxKind::JS_STRING_LITERAL => JsSyntaxKind::JS_STRING_LITERAL_EXPRESSION,
        // JsSyntaxKind::NULL_KW => JsSyntaxKind::JS_NULL_LITERAL_EXPRESSION,
        // JsSyntaxKind::TRUE_KW | JsSyntaxKind::FALSE_KW => {
        //     JsSyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION
        // }
        _ => return ParsedSyntax::Absent,
    };

    // let m = p.start();
    // p.bump_any();
    // ParsedSyntax::Present(m.complete(p, literal_kind))
}

pub(crate) fn parse_number_literal_expression(p: &mut MLangParser) -> ParsedSyntax {
    if !p.at(Number) {
        return ParsedSyntax::Absent;
    }

    let m = p.start();
    p.bump_any();
    ParsedSyntax::Present(m.complete(p, MLANG_NUMBER_LITERAL_EXPRESSION))
}

#[cfg(test)]
mod tests {
    use crate::parser_v2::parse_common;

    #[test]
    fn test() {
        let (events, errors, trivia) = parse_common("+1 # a");
        dbg!(events);
        dbg!(errors);
        dbg!(trivia);
    }
}
