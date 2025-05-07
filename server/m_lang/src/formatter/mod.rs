mod cst;
mod macros;
mod prelude;
mod rules;
pub mod utils;

pub(crate) mod comments;
pub(crate) mod context;
mod parentheses;
pub(crate) mod separated;
mod syntax_rewriter;

use super::syntax::{
    AnyMDeclaration, AnyMStatement, MLanguage, MSyntaxKind, MSyntaxNode, MSyntaxToken,
};
use biome_formatter::format_element::tag::Label;
use biome_formatter::prelude::*;
use biome_formatter::{
    comments::Comments, write, CstFormatContext, Format, FormatLanguage, FormatToken,
    TransformSourceMap,
};

use biome_formatter::{Buffer, FormatOwnedWithRule, FormatRefWithRule, Printed};
pub use biome_formatter::{IndentStyle, IndentWidth, LineWidth};
use biome_rowan::TextRange;
use biome_rowan::{AstNode, SyntaxNode};

use comments::MCommentStyle;
pub(crate) use context::MFormatContext;
pub use context::MFormatOptions;
use cst::FormatMSyntaxNode;
use syntax_rewriter::transform;

#[cfg(test)]
use biome_formatter::Formatted;

/// Used to get an object that knows how to format this object.
pub(crate) trait AsFormat<Context> {
    type Format<'a>: biome_formatter::Format<Context>
    where
        Self: 'a;

    /// Returns an object that is able to format this object.
    fn format(&self) -> Self::Format<'_>;
}

/// Implement [AsFormat] for references to types that implement [AsFormat].
impl<T, C> AsFormat<C> for &T
where
    T: AsFormat<C>,
{
    type Format<'a>
        = T::Format<'a>
    where
        Self: 'a;

    fn format(&self) -> Self::Format<'_> {
        AsFormat::format(&**self)
    }
}

/// Implement [AsFormat] for [SyntaxResult] where `T` implements [AsFormat].
///
/// Useful to format mandatory AST fields without having to unwrap the value first.
impl<T, C> AsFormat<C> for biome_rowan::SyntaxResult<T>
where
    T: AsFormat<C>,
{
    type Format<'a>
        = biome_rowan::SyntaxResult<T::Format<'a>>
    where
        Self: 'a;

    fn format(&self) -> Self::Format<'_> {
        match self {
            Ok(value) => Ok(value.format()),
            Err(err) => Err(*err),
        }
    }
}

/// Implement [AsFormat] for [Option] when `T` implements [AsFormat]
///
/// Allows to call format on optional AST fields without having to unwrap the field first.
impl<T, C> AsFormat<C> for Option<T>
where
    T: AsFormat<C>,
{
    type Format<'a>
        = Option<T::Format<'a>>
    where
        Self: 'a;

    fn format(&self) -> Self::Format<'_> {
        self.as_ref().map(|value| value.format())
    }
}

/// Used to convert this object into an object that can be formatted.
///
/// The difference to [AsFormat] is that this trait takes ownership of `self`.
pub(crate) trait IntoFormat<Context> {
    type Format: biome_formatter::Format<Context>;

    fn into_format(self) -> Self::Format;
}

impl<T, Context> IntoFormat<Context> for biome_rowan::SyntaxResult<T>
where
    T: IntoFormat<Context>,
{
    type Format = biome_rowan::SyntaxResult<T::Format>;

    fn into_format(self) -> Self::Format {
        self.map(IntoFormat::into_format)
    }
}

/// Implement [IntoFormat] for [Option] when `T` implements [IntoFormat]
///
/// Allows to call format on optional AST fields without having to unwrap the field first.
impl<T, Context> IntoFormat<Context> for Option<T>
where
    T: IntoFormat<Context>,
{
    type Format = Option<T::Format>;

    fn into_format(self) -> Self::Format {
        self.map(IntoFormat::into_format)
    }
}

/// Formatting specific [Iterator] extensions
pub(crate) trait FormattedIterExt {
    /// Converts every item to an object that knows how to format it.
    fn formatted<Context>(self) -> FormattedIter<Self, Self::Item, Context>
    where
        Self: Iterator + Sized,
        Self::Item: IntoFormat<Context>,
    {
        FormattedIter {
            inner: self,
            options: std::marker::PhantomData,
        }
    }
}

impl<I> FormattedIterExt for I where I: std::iter::Iterator {}

pub(crate) struct FormattedIter<Iter, Item, Context>
where
    Iter: Iterator<Item = Item>,
{
    inner: Iter,
    options: std::marker::PhantomData<Context>,
}

impl<Iter, Item, Context> std::iter::Iterator for FormattedIter<Iter, Item, Context>
where
    Iter: Iterator<Item = Item>,
    Item: IntoFormat<Context>,
{
    type Item = Item::Format;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next()?.into_format())
    }
}

impl<Iter, Item, Context> std::iter::FusedIterator for FormattedIter<Iter, Item, Context>
where
    Iter: std::iter::FusedIterator<Item = Item>,
    Item: IntoFormat<Context>,
{
}

impl<Iter, Item, Context> std::iter::ExactSizeIterator for FormattedIter<Iter, Item, Context>
where
    Iter: Iterator<Item = Item> + std::iter::ExactSizeIterator,
    Item: IntoFormat<Context>,
{
}

pub(crate) type MFormatter<'buf> = Formatter<'buf, MFormatContext>;

/// Rule for formatting a JavaScript [AstNode].
pub(crate) trait FormatNodeRule<N>
where
    N: AstNode<Language = MLanguage>,
{
    fn fmt(&self, node: &N, f: &mut MFormatter) -> FormatResult<()> {
        if self.is_suppressed(node, f) {
            return write!(f, [format_suppressed_node(node.syntax())]);
        }

        self.fmt_leading_comments(node, f)?;
        self.fmt_node(node, f)?;
        self.fmt_dangling_comments(node, f)?;
        self.fmt_trailing_comments(node, f)
    }

    /// Formats the node without comments. Ignores any suppression comments.
    fn fmt_node(&self, node: &N, f: &mut MFormatter) -> FormatResult<()> {
        let needs_parentheses = self.needs_parentheses(node);

        if needs_parentheses {
            write!(f, [text("(")])?;
        }

        self.fmt_fields(node, f)?;

        if needs_parentheses {
            write!(f, [text(")")])?;
        }

        Ok(())
    }

    /// Formats the node's fields.
    fn fmt_fields(&self, item: &N, f: &mut MFormatter) -> FormatResult<()>;

    /// Returns whether the node requires parens.
    fn needs_parentheses(&self, item: &N) -> bool {
        let _ = item;
        false
    }

    /// Returns `true` if the node has a suppression comment and should use the same formatting as in the source document.
    fn is_suppressed(&self, node: &N, f: &MFormatter) -> bool {
        f.context().comments().is_suppressed(node.syntax())
    }

    /// Formats the [leading comments](biome_formatter::comments#leading-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the leading comments.
    fn fmt_leading_comments(&self, node: &N, f: &mut MFormatter) -> FormatResult<()> {
        format_leading_comments(node.syntax()).fmt(f)
    }

    /// Formats the [dangling comments](biome_formatter::comments#dangling-comments) of the node.
    ///
    /// You should override this method if the node handled by this rule can have dangling comments because the
    /// default implementation formats the dangling comments at the end of the node, which isn't ideal but ensures that
    /// no comments are dropped.
    ///
    /// A node can have dangling comments if all its children are tokens or if all node childrens are optional.
    fn fmt_dangling_comments(&self, node: &N, f: &mut MFormatter) -> FormatResult<()> {
        format_dangling_comments(node.syntax())
            .with_soft_block_indent()
            .fmt(f)
    }

    /// Formats the [trailing comments](biome_formatter::comments#trailing-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the trailing comments.
    fn fmt_trailing_comments(&self, node: &N, f: &mut MFormatter) -> FormatResult<()> {
        format_trailing_comments(node.syntax()).fmt(f)
    }
}

/// Rule for formatting an bogus node.
pub(crate) trait FormatBogusNodeRule<N>
where
    N: AstNode<Language = MLanguage>,
{
    fn fmt(&self, node: &N, f: &mut MFormatter) -> FormatResult<()> {
        format_bogus_node(node.syntax()).fmt(f)
    }
}

/// Format implementation specific to JavaScript tokens.
pub(crate) type FormatMSyntaxToken = FormatToken<MFormatContext>;

impl AsFormat<MFormatContext> for MSyntaxToken {
    type Format<'a> = FormatRefWithRule<'a, MSyntaxToken, FormatMSyntaxToken>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatMSyntaxToken::default())
    }
}

impl IntoFormat<MFormatContext> for MSyntaxToken {
    type Format = FormatOwnedWithRule<MSyntaxToken, FormatMSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatMSyntaxToken::default())
    }
}

#[derive(Debug, Clone)]
pub(crate) struct MFormatLanguage {
    options: MFormatOptions,
}
impl MFormatLanguage {
    pub fn new(options: MFormatOptions) -> Self {
        Self { options }
    }
}

impl FormatLanguage for MFormatLanguage {
    type SyntaxLanguage = MLanguage;
    type Context = MFormatContext;
    type FormatRule = FormatMSyntaxNode;

    fn transform(
        &self,
        root: &SyntaxNode<Self::SyntaxLanguage>,
    ) -> Option<(SyntaxNode<Self::SyntaxLanguage>, TransformSourceMap)> {
        Some(transform(root.clone()))
    }

    fn is_range_formatting_node(&self, node: &MSyntaxNode) -> bool {
        let kind = node.kind();

        // Do not format variable declaration nodes, format the whole statement instead
        if matches!(kind, MSyntaxKind::M_VARIABLE_DECLARATION) {
            return false;
        }

        AnyMStatement::can_cast(kind)
            || AnyMDeclaration::can_cast(kind)
            || matches!(kind, MSyntaxKind::M_DIRECTIVE)
    }

    fn options(&self) -> &MFormatOptions {
        &self.options
    }

    fn create_context(
        self,
        root: &MSyntaxNode,
        source_map: Option<TransformSourceMap>,
    ) -> Self::Context {
        let comments = Comments::from_node(root, &MCommentStyle, source_map.as_ref());
        MFormatContext::new(self.options, comments).with_source_map(source_map)
    }
}

/// Formats a range within a file, supported by Biome
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [MFormatContext], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Formatted] result with a range corresponding to the
/// range of the input that was effectively overwritten by the formatter
pub fn format_range(
    options: MFormatOptions,
    root: &MSyntaxNode,
    range: TextRange,
) -> FormatResult<Printed> {
    biome_formatter::format_range(root, range, MFormatLanguage::new(options))
}

/// Formats a JavaScript (and its super languages) file based on its features.
///
/// It returns a [Formatted] result, which the user can use to override a file.
/// Used only in tests
#[cfg(test)]
pub(crate) fn format_node(
    options: MFormatOptions,
    root: &MSyntaxNode,
) -> FormatResult<Formatted<MFormatContext>> {
    biome_formatter::format_node(root, MFormatLanguage::new(options))
}

/// Formats a single node within a file, supported by Biome.
///
/// This runs a simple heuristic to determine the initial indentation
/// level of the node based on the provided [MFormatContext], which
/// must match currently the current initial of the file. Additionally,
/// because the reformatting happens only locally the resulting code
/// will be indented with the same level as the original selection,
/// even if it's a mismatch from the rest of the block the selection is in
///
/// It returns a [Formatted] result
pub fn format_sub_tree(options: MFormatOptions, root: &MSyntaxNode) -> FormatResult<Printed> {
    biome_formatter::format_sub_tree(root, MFormatLanguage::new(options))
}

#[derive(Copy, Clone, Debug)]
pub(crate) enum MLabels {
    MemberChain,
}

impl Label for MLabels {
    fn id(&self) -> u64 {
        *self as u64
    }

    fn debug_name(&self) -> &'static str {
        match self {
            MLabels::MemberChain => "MemberChain",
        }
    }
}

#[cfg(test)]
mod tests {

    use super::format_node;
    use super::format_range;

    use super::context::MFormatOptions;
    use crate::formatter::context::TrailingCommas;
    use crate::parser::parse;
    use crate::syntax::MFileSource;
    use biome_formatter::{IndentStyle, IndentWidth, LineWidth};
    use biome_rowan::{TextRange, TextSize};

    #[test]
    fn test_range_formatting() {
        let input = "
Функция a() {
   перем массив1 = @[1
   ,2,3,
   ];
}

Функция b() {
   перем массив2 = @[4
   ,5,6,
   ];
}

Функция c() {
   перем массив3 = @[7
   ,8,9,
   ];
}
";

        let range_start = TextSize::try_from(input.find("@[").unwrap()).unwrap();
        let range_end = TextSize::try_from(input.find("];").unwrap()).unwrap();

        let tree = parse(input, MFileSource::script());

        let result = format_range(
            MFormatOptions::new(MFileSource::script())
                .with_indent_style(IndentStyle::Space)
                .with_indent_width(4.into()),
            &tree.syntax(),
            TextRange::new(range_start, range_end),
        );
        let result = result.expect("range formatting failed");
        assert_eq!(result.as_code(), "перем массив1 = @[1, 2, 3];");
        assert_eq!(
            result.range(),
            Some(TextRange::new(TextSize::from(25), TextSize::from(70)))
        );
    }

    #[test]
    fn test_range_formatting_indentation() {
        let input = "
func f() {
         var veryLongIdentifierToCauseALineBreak = @{ veryLongKeyToCauseALineBreak: \"veryLongValueToCauseALineBreak\" }
}
";

        let range_start = TextSize::try_from(input.find("var").unwrap()).unwrap();
        let range_end = TextSize::try_from(input.find('}').unwrap()).unwrap();

        let tree = parse(input, MFileSource::script());
        let result = format_range(
            MFormatOptions::new(MFileSource::script())
                .with_indent_style(IndentStyle::Space)
                .with_indent_width(4.into())
                .with_trailing_commas(TrailingCommas::All),
            &tree.syntax(),
            TextRange::new(range_start, range_end),
        );

        let result = result.expect("range formatting failed");
        // As a result of the indentation normalization, the number of spaces within
        // the object expression is currently rounded down from an odd indentation level
        assert_eq!(
            result.as_code(),
            "var veryLongIdentifierToCauseALineBreak = @{\n            veryLongKeyToCauseALineBreak: \"veryLongValueToCauseALineBreak\",\n        };"
        );
        assert_eq!(
            result.range(),
            Some(TextRange::new(range_start, range_end + TextSize::from(1)))
        );
    }

    #[test]
    fn test_range_formatting_whitespace() {
        let input = "               ";

        let range_start = TextSize::from(5);
        let range_end = TextSize::from(5);

        let tree = parse(input, MFileSource::script());
        let result = format_range(
            MFormatOptions::new(MFileSource::script())
                .with_indent_style(IndentStyle::Space)
                .with_indent_width(4.into()),
            &tree.syntax(),
            TextRange::new(range_start, range_end),
        );

        let result = result.expect("range formatting failed");
        assert_eq!(result.as_code(), "");
        assert_eq!(result.range(), Some(TextRange::new(range_start, range_end)));
    }

    #[test]
    fn test_range_formatting_middle_of_token() {
        let input = r#"func Foo(){
#
}
"#;

        let range = TextRange::new(TextSize::from(6), TextSize::from(15));

        debug_assert_eq!(
            &input[range],
            r#"oo(){
#
}"#
        );

        let tree = parse(input, MFileSource::script());
        let result = format_range(
            MFormatOptions::new(MFileSource::script())
                .with_indent_style(IndentStyle::Space)
                .with_indent_width(4.into()),
            &tree.syntax(),
            range,
        )
        .expect("Range formatting failed");

        assert_eq!(
            result.as_code(),
            r#"func Foo()
{
    #
}"#
        );
        assert_eq!(
            result.range(),
            Some(TextRange::new(TextSize::from(0), TextSize::from(15)))
        )
    }

    #[test]
    fn format_range_out_of_bounds() {
        let src = "statement();";

        let syntax = MFileSource::script();
        let tree = parse(src, syntax);

        let result = format_range(
            MFormatOptions::new(syntax),
            &tree.syntax(),
            TextRange::new(TextSize::from(0), TextSize::of(src) + TextSize::from(5)),
        );

        assert!(result.is_err());
    }

    #[test]
    fn format_query_like_expressions() {
        let src = r#"
var qq = Query(`
    select row_id from ~Лицевые договора~ where Договор = :1
`,1,"p1,S");

var qq = Command(`update stack."Лицевые договора" set Договор = :1 where row_id = :2`,1,"p1,S,p2,S");

var qq = сессия.Query(`select row_id from ~Лицевые договора~ where Договор = :1`,1,"p1,S");

var qq = Query(`select row_id from ~Лицевые договора~ `,1,"p1,S");
        "#;

        let syntax = MFileSource::module();
        let tree = parse(src, syntax);

        let doc = format_node(
            MFormatOptions::new(syntax)
                .with_indent_style(IndentStyle::Space)
                .with_line_width(LineWidth::try_from(80).unwrap())
                .with_indent_width(IndentWidth::from(3)),
            &tree.syntax(),
        );

        let result = doc.unwrap().print().unwrap();
        assert_eq!(
            result.as_code(),
            r#"var qq = Query(`
    select row_id from ~Лицевые договора~ where Договор = :1
`, 1, "p1,S");

var qq = Command(
   `update stack."Лицевые договора" set Договор = :1 where row_id = :2`,
   1, "p1,S,p2,S"
);

var qq = сессия.Query(
   `select row_id from ~Лицевые договора~ where Договор = :1`,
   1, "p1,S"
);

var qq = Query(`select row_id from ~Лицевые договора~ `, 1, "p1,S");
"#
        );
    }

    #[test]
    fn format() {
        let src = r#"

        "#;

        let syntax = MFileSource::module();
        let tree = dbg!(parse(src, syntax));

        let doc = format_node(
            MFormatOptions::new(syntax)
                .with_indent_style(IndentStyle::Space)
                .with_line_width(LineWidth::try_from(120).unwrap())
                .with_indent_width(IndentWidth::from(3)),
            &tree.syntax(),
        );

        let result = doc.unwrap().print().unwrap();
        println!("{}", &result.as_code());
    }
}
