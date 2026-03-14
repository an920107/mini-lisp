mod common;

#[test]
fn test_print_1() {
    let code = r#"
        (print-num 1)
        (print-num 2)
        (print-num 3)
        (print-num 4)
    "#;
    common::assert_evaluation(code, &["1", "2", "3", "4"]);
}

#[test]
fn test_print_2() {
    let code = r#"
        (print-num 0)
        (print-num -123)
        (print-num 456)
    "#;
    common::assert_evaluation(code, &["0", "-123", "456"]);
}
