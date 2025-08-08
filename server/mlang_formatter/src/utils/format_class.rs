use crate::prelude::*;
use mlang_syntax::MClassDeclaration;
use biome_formatter::{format_args, write};

pub struct FormatClass<'a> {
    class: &'a MClassDeclaration,
}

impl FormatClass<'_> {
    fn should_group(&self, comments: &MComments) -> FormatResult<bool> {
        if comments.has_trailing_comments(self.class.id()?.syntax()) {
            return Ok(true);
        }

        if let Some(extends) = self.class.extends_clause() {
            if comments.has_trailing_comments(extends.syntax()) {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

impl<'a> From<&'a MClassDeclaration> for FormatClass<'a> {
    fn from(class: &'a MClassDeclaration) -> Self {
        Self { class }
    }
}

impl Format<MFormatContext> for FormatClass<'_> {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        let id = self.class.id();
        let extends = self.class.extends_clause();
        let class_token = self.class.class_token()?;
        let members = self.class.members();

        let group_mode = self.should_group(f.comments())?;

        write!(f, [class_token.format()])?;

        let head = format_with(|f| {
            write!(f, [space(), id.format()])?;

            Ok(())
        });

        let format_heritage_clauses = format_with(|f| {
            if let Some(extends) = &extends {
                if group_mode {
                    write!(f, [soft_line_break_or_space(), group(&extends.format())])?;
                } else {
                    write!(f, [space(), extends.format()])?;
                }
            }

            Ok(())
        });

        if group_mode {
            let indented =
                format_with(|f| write!(f, [indent(&format_args![head, format_heritage_clauses])]));

            let heritage_id = f.group_id("heritageGroup");
            write!(
                f,
                [group(&indented).with_group_id(Some(heritage_id)), space()]
            )?;

            if !members.is_empty() {
                write!(
                    f,
                    [if_group_breaks(&hard_line_break()).with_group_id(Some(heritage_id))]
                )?;
            }
        } else {
            write!(f, [head, format_heritage_clauses, space()])?;
        }

        if let Some(doc_string) = self.class.doc_string() {
            write!(
                f,
                [hard_line_break(), doc_string.format(), hard_line_break()]
            )?;
        }

        if members.is_empty() {
            write!(
                f,
                [
                    self.class.l_curly_token().format(),
                    format_dangling_comments(self.class.syntax()).with_block_indent(),
                    self.class.r_curly_token().format()
                ]
            )
        } else {
            write![
                f,
                [
                    hard_line_break(),
                    self.class.l_curly_token().format(),
                    block_indent(&members.format()),
                    self.class.r_curly_token().format()
                ]
            ]
        }
    }
}
