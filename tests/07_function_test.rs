mod common;

#[test]
fn test_function_1() {
    let code = r#"
        (print-num
            ((fun (x) (+ x 1)) 3))

        (print-num
            ((fun (a b) (+ a b)) 4 5))
    "#;
    common::assert_evaluation(code, &["4", "9"]);
}

#[test]
fn test_function_2() {
    let code = r#"
        (define x 0)

        (print-num
            ((fun (x y z) (+ x (* y z))) 10 20 30))

        (print-num x)
    "#;
    common::assert_evaluation(code, &["610", "0"]);
}
