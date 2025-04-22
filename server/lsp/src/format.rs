use m_lang::{
    formatter::{format_range, IndentStyle, IndentWidth, LineWidth, MFormatOptions},
    parser::parse,
    syntax::MFileSource,
};

use ropey::Rope;
use tower_lsp::lsp_types::FormattingOptions;
use tower_lsp::lsp_types::{Range, TextEdit};

use crate::{position, text_range};

pub async fn format(
    text: &Rope,
    options: FormattingOptions,
    range: Range,
) -> Option<Vec<TextEdit>> {
    let parsed = parse(&text.to_string(), MFileSource::module());

    let format_options = MFormatOptions::new(MFileSource::module())
        .with_indent_style(match options.insert_spaces {
            true => IndentStyle::Space,
            false => IndentStyle::Tab,
        })
        .with_line_width(LineWidth::try_from(80).unwrap())
        .with_indent_width(IndentWidth::try_from(options.tab_size as u8).unwrap_or_default());
    let formatted_text =
        format_range(format_options, &parsed.syntax(), text_range(&text, range)?).ok()?;

    let range = formatted_text.range()?;
    let range = position(&text, range)?;
    let new_text = formatted_text.into_code();

    let edits = vec![TextEdit { range, new_text }];

    Some(edits)
}
