#[macro_export]
macro_rules! assert_fmt {
    ($src:expr) => {
        assert_fmt!($src, MFileSource::script());
    };

    ($src:expr, $file_type:expr) => {{
        use mlang_formatter::{IndentStyle, IndentWidth, LineWidth, MFormatOptions, format_node};
        use mlang_parser::parse;
        use mlang_syntax::MFileSource;

        let syntax = $file_type;
        let tree = parse($src, syntax);

        let options = MFormatOptions::new(syntax)
            .with_indent_style(IndentStyle::Space)
            .with_line_width(LineWidth::try_from(120).unwrap())
            .with_indent_width(IndentWidth::from(3));

        let doc = format_node(options, &tree.syntax());
        let result = doc.unwrap().print().unwrap();
        let result = result.as_code();
        assert_eq!(
            $src, result,
            "formatted code: \n======\n{}\n======\n",
            result
        );
    }};
}

#[macro_export]
macro_rules! assert_fmt_range {
    ($src:expr, $dest:expr, $range:expr) => {
        assert_fmt_range!($src, $dest, $range, MFileSource::script());
    };

    ($src:expr, $dest:expr, $range:expr, $file_type:expr) => {
        use mlang_formatter::{IndentStyle, IndentWidth, LineWidth, MFormatOptions, format_range};
        use mlang_parser::parse;
        use mlang_syntax::{MFileSource, TextRange, TextSize};

        let syntax = $file_type;
        let tree = parse($src, syntax);

        let options = MFormatOptions::new(syntax)
            .with_indent_style(IndentStyle::Space)
            .with_line_width(LineWidth::try_from(120).unwrap())
            .with_indent_width(IndentWidth::from(3));

        let doc = format_range(
            options,
            &tree.syntax(),
            TextRange::new(TextSize::from($range.start), TextSize::from($range.end)),
        );
        let result = doc.unwrap().into_code();
        assert_eq!(
            $dest, result,
            "formatted code: \n======\n{}\n======\n",
            result
        );
    };
}
