use line_index::{LineCol, LineColRange};
use mlang_formatter::{IndentStyle, IndentWidth, LineWidth, MFormatOptions, format_range};
use mlang_syntax::MFileSource;

use tower_lsp::lsp_types::{FormattingOptions, Position};
use tower_lsp::lsp_types::{Range, TextEdit};

use crate::document::CurrentDocument;

pub fn format(
    document: &CurrentDocument,
    options: FormattingOptions,
    range: Range,
) -> Option<Vec<TextEdit>> {
    let format_options = MFormatOptions::new(MFileSource::module())
        .with_indent_style(match options.insert_spaces {
            true => IndentStyle::Space,
            false => IndentStyle::Tab,
        })
        .with_line_width(LineWidth::try_from(150).unwrap())
        .with_indent_width(IndentWidth::from(options.tab_size as u8));

    let line_index = document.line_index();
    let text_range = {
        let Range { start, end } = range.clone();
        let range = LineColRange {
            start: LineCol {
                col: start.character,
                line: start.line,
            },
            end: LineCol {
                col: end.character,
                line: end.line,
            },
        };
        line_index.range(range)?
    };

    let formatted_text = format_range(format_options, &document.syntax(), text_range).ok()?;

    let range = {
        let range = formatted_text.range()?;
        let LineColRange { start, end } = line_index.line_col_range(range)?;
        Range {
            start: Position::new(start.line, start.col),
            end: Position::new(end.line, end.col),
        }
    };

    let new_text = formatted_text.into_code();
    let edits = vec![TextEdit { range, new_text }];

    Some(edits)
}
