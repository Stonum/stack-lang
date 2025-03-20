//! This module provides important and useful traits to help to format tokens and nodes
//! when implementing a syntax formatter.

pub(crate) use super::AsFormat;
pub(crate) use super::IntoFormat;
pub(crate) use super::{
    comments::MComments, AsFormat as _, FormatNodeRule, FormattedIterExt, MFormatContext,
    MFormatter,
};
pub use biome_formatter::prelude::*;
pub use biome_formatter::separated::TrailingSeparator;
pub use biome_rowan::{AstNode as _, AstNodeList as _, AstSeparatedList as _};

pub(crate) use super::macros::{impl_format, impl_format_with_rule, impl_rule};
pub(crate) use super::separated::FormatAstSeparatedListExtension;
