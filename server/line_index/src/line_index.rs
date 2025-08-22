//! `LineIndex` maps flat `TextSize` offsets into `(Line, Column)`
//! representation.

use std::mem;

use biome_rowan::TextRange;
use biome_text_size::TextSize;
use rustc_hash::FxHashMap;

use crate::{LineCol, LineColRange, WideChar};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LineIndex {
    /// Offset the beginning of each line, zero-based.
    pub newlines: Vec<TextSize>,
    /// List of non-ASCII characters on each line.
    pub line_wide_chars: FxHashMap<u32, Vec<WideChar>>,
}

impl LineIndex {
    pub fn new(text: &str) -> Self {
        let mut line_wide_chars = FxHashMap::default();
        let mut wide_chars = Vec::new();

        let mut newlines = vec![TextSize::from(0)];

        let mut current_col = TextSize::from(0);

        let mut line = 0;
        for (offset, char) in text.char_indices() {
            let char_size = TextSize::of(char);

            if char == '\n' {
                // SAFETY: the conversion from `usize` to `TextSize` can fail if `offset`
                // is larger than 2^32. We don't support such large files.
                let char_offset = TextSize::try_from(offset).expect("TextSize overflow");
                newlines.push(char_offset + char_size);

                // Save any utf-16 characters seen in the previous line
                if !wide_chars.is_empty() {
                    line_wide_chars.insert(line, mem::take(&mut wide_chars));
                }

                // Prepare for processing the next line
                current_col = TextSize::from(0);
                line += 1;
                continue;
            }

            if !char.is_ascii() {
                wide_chars.push(WideChar {
                    start: current_col,
                    end: current_col + char_size,
                });
            }

            current_col += char_size;
        }

        // Save any utf-16 characters seen in the last line
        if !wide_chars.is_empty() {
            line_wide_chars.insert(line, wide_chars);
        }

        Self {
            newlines,
            line_wide_chars,
        }
    }

    /// Return the number of lines in the index, clamped to [u32::MAX]
    pub fn len(&self) -> u32 {
        self.newlines.len().try_into().unwrap_or(u32::MAX)
    }

    /// Return `true` if the index contains no lines.
    pub fn is_empty(&self) -> bool {
        self.newlines.is_empty()
    }

    pub fn line_col(&self, offset: TextSize) -> Option<LineCol> {
        let line = self.newlines.partition_point(|&it| it <= offset) - 1;
        let line_start_offset = self.newlines.get(line)?;

        let line = u32::try_from(line).ok()?;
        let col_offset = offset - line_start_offset;
        let mut col: u32 = u32::from(offset - line_start_offset);

        if let Some(wide_chars) = self.line_wide_chars.get(&line) {
            for c in wide_chars {
                if c.end <= col_offset {
                    col -= u32::from(c.len()) - 1; // delta between ascii and non-ascii chars
                } else {
                    // From here on, all utf16 characters come *after* the character we are mapping,
                    // so we don't need to take them into account
                    break;
                }
            }
        }

        Some(LineCol { line, col })
    }

    pub fn offset(&self, line_col: LineCol) -> Option<TextSize> {
        let line_start_offset = *self.newlines.get(line_col.line as usize)?;
        let mut col_offset = line_col.col;

        if let Some(wide_chars) = self.line_wide_chars.get(&line_col.line) {
            for c in wide_chars {
                if col_offset > u32::from(c.start) {
                    col_offset += u32::from(c.len()) - 1; // delta between ascii and non-ascii chars
                } else {
                    // From here on, all utf16 characters come *after* the character we are mapping,
                    // so we don't need to take them into account
                    break;
                }
            }
        }

        Some(line_start_offset + TextSize::from(col_offset))
    }

    pub fn line_col_range(&self, range: TextRange) -> Option<LineColRange> {
        let start = self.line_col(range.start())?;
        let end = self.line_col(range.end())?;
        Some(LineColRange { start, end })
    }

    pub fn range(&self, line_col_range: LineColRange) -> Option<TextRange> {
        let start = self.offset(line_col_range.start)?;
        let end = self.offset(line_col_range.end)?;
        Some(TextRange::new(start, end))
    }

    pub fn chars_count(&self, start: LineColRange) -> Option<u32> {
        let (start, end) = (start.start, start.end);
        if start.line == end.line {
            return end.col.checked_sub(start.col);
        }

        let mut wide_chars_len = 0; //u32::from(end_offset) - u32::from(start_offset); // count ascii chars

        let start_line_offset = u32::from(*self.newlines.get(start.line as usize)?);
        let end_line_offset = u32::from(*self.newlines.get(end.line as usize)?);

        let mut start_col_offset = start.col;
        let mut end_col_offset = end.col;

        // count wide chars in each line
        for line in start.line..=end.line {
            if let Some(wide_chars) = self.line_wide_chars.get(&line) {
                for c in wide_chars {
                    let (c_start, c_end) = (u32::from(c.start), u32::from(c.end));
                    let c_delta = u32::from(c.len()) - 1; // delta between ascii and non-ascii chars

                    // calculate offsets for column
                    if line == start.line && start_col_offset > c_start {
                        start_col_offset += c_delta;
                    }
                    if line == end.line && end_col_offset > c_start {
                        end_col_offset += c_delta;
                    }

                    // calculate wide chars length
                    if line == start.line {
                        if c_start >= start_col_offset {
                            wide_chars_len += c_delta;
                        }
                    } else if line == end.line {
                        if c_end <= end_col_offset {
                            wide_chars_len += c_delta;
                        }
                    } else {
                        wide_chars_len += c_delta;
                    }
                }
            }
        }

        Some(
            (end_line_offset + end_col_offset)
                - (start_line_offset + start_col_offset)
                - wide_chars_len,
        )
    }
}
