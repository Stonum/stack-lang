#[macro_use]
mod helper;

#[test]
fn format_function_variable() {
    assert_fmt!(
        r#"#
перем проверитьВложение = Функция(_вложение) {
   Вернуть true;
};
"#
    );
}
