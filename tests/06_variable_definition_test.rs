mod common;

#[test]
fn test_variable_definition_1() {
    let code = r#"
        (define x 1)
        (print-num x)

        (define y (+ 1 2 3))
        (print-num y)
    "#;
    common::assert_evaluation(code, &["1", "6"]);
}

#[test]
fn test_variable_definition_2() {
    let code = r#"
        (define a (* 1 2 3 4))
        (define b (+ 10 -5 -2 -1))
        (print-num (+ a b))
    "#;
    common::assert_evaluation(code, &["26"]);
}
