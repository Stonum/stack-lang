use crate::formatter::prelude::*;
use crate::formatter::rules::bindings::parameters::ParameterLayout;

use crate::formatter::context::trailing_commas::FormatTrailingCommas;
use crate::syntax::{AnyMConstructorParameter, AnyMParameter, MParameterList};
use crate::syntax::{AnyMParameterList, AnyParameter};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMParameterList;
impl_format!(MParameterList, FormatMParameterList);

impl FormatRule<MParameterList> for FormatMParameterList {
    type Context = MFormatContext;

    fn fmt(&self, node: &MParameterList, f: &mut MFormatter) -> FormatResult<()> {
        FormatMAnyParameterList::with_layout(
            &AnyMParameterList::from(node.clone()),
            ParameterLayout::Default,
        )
        .fmt(f)
    }
}

#[derive(Debug, Copy, Clone)]
pub(crate) struct FormatMAnyParameterList<'a> {
    list: &'a AnyMParameterList,
    layout: Option<ParameterLayout>,
}

impl<'a> FormatMAnyParameterList<'a> {
    pub fn with_layout(list: &'a AnyMParameterList, layout: ParameterLayout) -> Self {
        Self {
            list,
            layout: Some(layout),
        }
    }
}

impl Format<MFormatContext> for FormatMAnyParameterList<'_> {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        match self.layout {
            None | Some(ParameterLayout::Default | ParameterLayout::NoParameters) => {
                let has_trailing_rest = match self.list.last() {
                    Some(elem) => matches!(
                        elem?,
                        AnyParameter::AnyMParameter(AnyMParameter::MRestParameter(_))
                            | AnyParameter::AnyMConstructorParameter(
                                AnyMConstructorParameter::MRestParameter(_)
                            )
                    ),
                    None => false,
                };

                // If it's a rest parameter, the assumption is no more
                // parameters could be added afterward, so no separator is
                // added there either.
                let trailing_separator = if has_trailing_rest {
                    TrailingSeparator::Disallowed
                } else {
                    FormatTrailingCommas::All.trailing_separator(f.options())
                };

                let mut joiner = f.join_nodes_with_soft_line();
                join_parameter_list(&mut joiner, self.list, trailing_separator)?;
                joiner.finish()
            }
            Some(ParameterLayout::Hug) => {
                let mut join = f.join_with(space());

                match self.list {
                    AnyMParameterList::MParameterList(list) => join.entries(
                        list.format_separated(",")
                            .with_trailing_separator(TrailingSeparator::Omit),
                    ),
                    AnyMParameterList::MConstructorParameterList(list) => join.entries(
                        list.format_separated(",")
                            .with_trailing_separator(TrailingSeparator::Omit),
                    ),
                };

                join.finish()
            }
        }
    }
}

fn join_parameter_list<S>(
    joiner: &mut JoinNodesBuilder<'_, '_, S, MFormatContext>,
    list: &AnyMParameterList,
    trailing_separator: TrailingSeparator,
) -> FormatResult<()>
where
    S: Format<MFormatContext>,
{
    match list {
        AnyMParameterList::MParameterList(list) => {
            let entries = list
                .format_separated(",")
                .with_trailing_separator(trailing_separator)
                .zip(list.iter());

            for (format_entry, node) in entries {
                joiner.entry(node?.syntax(), &format_entry);
            }
        }
        AnyMParameterList::MConstructorParameterList(list) => {
            let entries = list
                .format_separated(",")
                .with_trailing_separator(trailing_separator)
                .zip(list.iter());

            for (format_entry, node) in entries {
                joiner.entry(node?.syntax(), &format_entry);
            }
        }
    }

    Ok(())
}
