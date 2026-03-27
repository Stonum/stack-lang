use biome_formatter::prelude::*;
use biome_formatter::{Argument, Arguments, write};

#[inline]
pub fn soft_block_indent_with_same_line<Context>(
    content: &impl Format<Context>,
) -> BlockIndentWithSameLine<'_, Context> {
    BlockIndentWithSameLine {
        content: Argument::new(content),
        mode: BreakMode::Soft,
    }
}

#[inline]
pub fn soft_block_indent_with_same_line_without_brake<Context>(
    content: &impl Format<Context>,
) -> BlockIndentWithSameLine<'_, Context> {
    BlockIndentWithSameLine {
        content: Argument::new(content),
        mode: BreakMode::None,
    }
}

// #[inline]
// pub fn block_indent_with_same_line<Context>(
//     content: &impl Format<Context>,
// ) -> BlockIndentWithSameLine<Context> {
//     BlockIndentWithSameLine {
//         content: Argument::new(content),
//         mode: IndentMode::Block,
//     }
// }

#[derive(Copy, Clone)]
pub struct BlockIndentWithSameLine<'a, Context> {
    content: Argument<'a, Context>,
    mode: BreakMode,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum BreakMode {
    Soft,
    // Block,
    None,
}

impl<Context> Format<Context> for BlockIndentWithSameLine<'_, Context> {
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        let snapshot = f.snapshot();

        f.write_element(FormatElement::Tag(Tag::StartIndent))?;

        let is_empty = {
            let mut recording = f.start_recording();
            recording.write_fmt(Arguments::from(&self.content))?;
            recording.stop().is_empty()
        };

        if is_empty {
            f.restore_snapshot(snapshot);
            return Ok(());
        }

        f.write_element(FormatElement::Tag(Tag::EndIndent))?;

        match self.mode {
            BreakMode::Soft => write!(f, [soft_line_break()]),
            // IndentMode::Block => write!(f, [hard_line_break()]),
            BreakMode::None => Ok(()),
        }
    }
}
