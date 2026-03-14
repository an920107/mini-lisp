mod common;

#[test]
fn test_first_class_function_1() {
    let code = r#"
        (define add-x
            (fun (x) (fun (y) (+ x y))))

        (define z (add-x 10))

        (print-num (z 1))
    "#;
    common::assert_evaluation(code, &["11"]);
}

#[test]
fn test_first_class_function_2() {
    let code = r#"
    (define foo
        (fun (f x) (f x)))

    (print-num
        (foo (fun (x) (- x 1)) 10))
    "#;
    common::assert_evaluation(code, &["9"]);
}
