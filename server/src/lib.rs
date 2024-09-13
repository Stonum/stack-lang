use ropey::Rope;
use tower_lsp::lsp_types::{Position, Range};

pub mod def;
pub mod fmt;
pub mod lexer;
pub mod parser;

pub fn position(rope: &Rope, span: std::ops::Range<usize>) -> Option<Range> {
    let start = get_position_from_offset(&rope, span.start)?;
    let end = get_position_from_offset(&rope, span.end)?;
    Some(Range { start, end })
}

fn get_position_from_offset(rope: &Rope, offset: usize) -> Option<Position> {
    let line = rope.try_byte_to_line(offset).ok()?;
    let first_char_of_line = rope.try_line_to_char(line).ok()?;
    let offset_char = rope.try_byte_to_char(offset).ok()?;
    let column = offset_char - first_char_of_line;
    Some(Position::new(line as u32, column as u32))
}
