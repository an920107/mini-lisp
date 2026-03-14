mod common;

#[test]
fn if_expression_test_1() {
    let code = r#"
        (print-num (if #t 1 2))
        (print-num (if #f 1 2))
    "#;
    common::assert_evaluation(code, &["1", "2"]);
}

#[test]
fn if_expression_test_2() {
    let code = r#"
        (print-num (if (< 1 2) (+ 1 2 3) (* 1 2 3 4 5)))
        (print-num (if (= 9 (* 2 5))
                    0
                    (if #t 1 2)))
    "#;
    common::assert_evaluation(code, &["6", "1"]);
}
