use m_lang::syntax::{TextRange, TextSize};
use ropey::Rope;
use tower_lsp::lsp_types::{Position, Range};

pub mod definition;
pub mod document;
pub mod format;

pub fn position(rope: &Rope, span: TextRange) -> Option<Range> {
    let start = get_position_from_offset(rope, span.start().into())?;
    let end = get_position_from_offset(rope, span.end().into())?;
    Some(Range { start, end })
}

fn get_position_from_offset(rope: &Rope, offset: usize) -> Option<Position> {
    let line = rope.try_byte_to_line(offset).ok()?;
    let first_char_of_line = rope.try_line_to_char(line).ok()?;
    let offset_char = rope.try_byte_to_char(offset).ok()?;
    let column = offset_char - first_char_of_line;
    Some(Position::new(line as u32, column as u32))
}

pub fn text_range(rope: &Rope, range: Range) -> Option<TextRange> {
    let start = get_text_size_from_position(rope, range.start)?;
    let end = get_text_size_from_position(rope, range.end)?;
    Some(TextRange::new(start, end))
}

pub fn get_text_size_from_position(rope: &Rope, pos: Position) -> Option<TextSize> {
    let line = pos.line as usize;
    let character = pos.character as usize;

    if line >= rope.len_lines() {
        return None;
    }

    let line_text = rope.line(line);
    let line_start = rope.line_to_byte(line);

    let byte_idx = line_start + line_text.try_char_to_byte(character).ok()?;

    TextSize::try_from(byte_idx).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_range() {
        let src = r#"
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
        "#;

        let rope = Rope::from_str(src);
        let range = Range::new(Position::new(2, 9), Position::new(4, 4));

        let t_range = text_range(&rope, range).expect("Convert range is failed");

        let start: usize = t_range.start().into();
        let end: usize = t_range.end().into();
        assert_eq!(
            &rope.to_string()[start..end],
            "массив1 = @[1\n   ,2,3,\n   ]"
        );

        let range = Range::new(Position::new(14, 9), Position::new(16, 4));
        let t_range = text_range(&rope, range).expect("Convert range failed");

        let start: usize = t_range.start().into();
        let end: usize = t_range.end().into();
        assert_eq!(
            &rope.to_string()[start..end],
            "массив3 = @[7\n   ,8,9,\n   ]"
        );
    }

    #[test]
    fn test_len_lines() {
        let src = "
SSSSSS
FFFFFF
";

        let rope = Rope::from_str(&src);
        assert_eq!(rope.len_lines(), 4);
    }
}
