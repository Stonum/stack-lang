//! The crate contains tools for converting between byte offsets and line / column positions.
use biome_text_size::TextSize;
pub use line_index::LineIndex;

mod line_index;

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LineCol {
    /// Zero-based
    pub line: u32,
    /// Zero-based utf8 offset
    pub col: u32,
}

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LineColRange {
    pub start: LineCol,
    pub end: LineCol,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct WideChar {
    /// Start offset of a character inside a line, zero-based
    pub start: TextSize,
    /// End offset of a character inside a line, zero-based
    pub end: TextSize,
}

impl WideChar {
    /// Returns the length in 8-bit UTF-8 code units.
    fn len(&self) -> TextSize {
        self.end - self.start
    }
}

#[cfg(test)]
mod tests {
    use crate::line_index::LineIndex;
    use crate::{LineCol, LineColRange};
    use biome_text_size::TextSize;

    macro_rules! check_conversion {
        ($line_index:ident : $line_col:expr => $offset:expr ) => {
            let offset = $line_index.offset($line_col);
            assert_eq!(
                offset,
                Some($offset),
                "for {:?} => {:?}",
                $line_col,
                $offset
            );

            let line_col = $line_index.line_col($offset);
            assert_eq!(
                line_col,
                Some($line_col),
                "for {:?} => {:?}",
                $line_col,
                $offset
            );
        };
    }

    #[test]
    fn empty_string() {
        let line_index = LineIndex::new("");
        check_conversion!(line_index: LineCol { line: 0, col: 0 } => TextSize::from(0));
    }

    #[test]
    fn empty_line() {
        let line_index = LineIndex::new("\n\n");
        check_conversion!(line_index: LineCol { line: 1, col: 0 } => TextSize::from(1));
    }

    #[test]
    fn line_end() {
        let line_index = LineIndex::new("abc\ndef\nghi");
        check_conversion!(line_index: LineCol { line: 1, col: 3 } => TextSize::from(7));
    }

    #[test]
    fn out_of_bounds_line() {
        let line_index = LineIndex::new("abcde\nfghij\n");

        let offset = line_index.offset(LineCol { line: 5, col: 0 });
        assert!(offset.is_none());
    }

    #[test]
    fn unicode() {
        let line_index = LineIndex::new("перем в = 123;");

        check_conversion!(line_index: LineCol { line: 0, col: 0 } => TextSize::from(0));
        check_conversion!(line_index: LineCol { line: 0, col: 1 } => TextSize::from(2));
        check_conversion!(line_index: LineCol { line: 0, col: 5 } => TextSize::from(10));
        check_conversion!(line_index: LineCol { line: 0, col: 7 } => TextSize::from(13));
        check_conversion!(line_index: LineCol { line: 0, col: 9 } => TextSize::from(15));
        check_conversion!(line_index: LineCol { line: 0, col: 14 } => TextSize::from(20));

        let line_index = LineIndex::new("строка1\n22с𐀁трока");
        check_conversion!(line_index: LineCol { line: 1, col: 5 } => TextSize::from(24));
    }

    #[test]
    fn chars_count() {
        let line_index = LineIndex::new("qwertйцуке12345\nasdfgфывап54321\nячсмиzxcvb09876");

        let range = LineColRange {
            start: LineCol { line: 1, col: 0 },
            end: LineCol { line: 2, col: 10 },
        };

        let chars_count = line_index.chars_count(range);
        assert_eq!(chars_count, Some(26));

        let range = LineColRange {
            start: LineCol { line: 0, col: 0 },
            end: LineCol { line: 2, col: 5 },
        };

        let chars_count = line_index.chars_count(range);
        assert_eq!(chars_count, Some(37));
    }

    #[ignore]
    #[test]
    fn test_every_chars() {
        let text: String = {
            let chars: Vec<char> = ((0 as char)..char::MAX).collect();
            chars
                .chunks(chars.len() / 1024)
                .map(|chunk| chunk.iter().collect::<String>())
                .collect::<Vec<_>>()
                .join("\n")
        };

        let line_index = LineIndex::new(&text);

        let mut lin_col = LineCol { line: 0, col: 0 };
        for (offset, char) in text.char_indices() {
            let got_offset = line_index.offset(lin_col).unwrap();
            assert_eq!(usize::from(got_offset), offset, "{:?} {char}", offset);

            let got_lin_col = line_index.line_col(got_offset).unwrap();
            assert_eq!(got_lin_col, lin_col, "{:?} {char}", offset);

            if char == '\n' {
                lin_col.line += 1;
                lin_col.col = 0;
            } else {
                lin_col.col += 1;
            }
        }
    }
}
