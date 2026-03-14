mod common;

#[test]
fn test_named_function_1() {
    let code = r#"
        (define foo
            (fun (a b c) (+ a b (* b c))))

        (print-num (foo 10 9 8))
    "#;
    common::assert_evaluation(code, &["91"]);
}

#[test]
fn test_named_function_2() {
    let code = r#"
        (define bar (fun (x) (+ x 1)))

        (define bar-z (fun () 2))

        (print-num (bar (bar-z)))
    "#;
    common::assert_evaluation(code, &["3"]);
}
