#[macro_export]
macro_rules! impl_format {
    ($type:ty, $format:ty) => {
        impl AsFormat<MFormatContext> for $type {
            type Format<'a> = biome_formatter::FormatRefWithRule<'a, $type, $format>;
            fn format(&self) -> Self::Format<'_> {
                #![allow(clippy::default_constructed_unit_structs)]
                biome_formatter::FormatRefWithRule::new(self, <$format>::default())
            }
        }

        impl IntoFormat<MFormatContext> for $type {
            type Format = biome_formatter::FormatOwnedWithRule<$type, $format>;
            fn into_format(self) -> Self::Format {
                #![allow(clippy::default_constructed_unit_structs)]
                biome_formatter::FormatOwnedWithRule::new(self, <$format>::default())
            }
        }
    };
}
pub(crate) use impl_format;

#[macro_export]
macro_rules! impl_rule {
    ($type:ty, $format:ty) => {
        impl FormatRule<$type> for $format {
            type Context = MFormatContext;
            #[inline(always)]
            fn fmt(&self, node: &$type, f: &mut MFormatter) -> FormatResult<()> {
                FormatNodeRule::<$type>::fmt(self, node, f)
            }
        }
    };
}

pub(crate) use impl_rule;

#[macro_export]
macro_rules! impl_format_with_rule {
    ($type:ty, $format:ty) => {
        impl_format!($type, $format);
        impl_rule!($type, $format);
    };
}

pub(crate) use impl_format_with_rule;
