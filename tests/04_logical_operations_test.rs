mod common;

#[test]
fn test_logical_operations_1() {
    let code = r#"
        (print-bool #t)
        (print-bool #f)

        (print-bool (and #t #f))
        (print-bool (and #t #t))

        (print-bool (or #t #f))
        (print-bool (or #f #f))

        (print-bool (not #t))
        (print-bool (not #f))
    "#;
    common::assert_evaluation(code, &["#t", "#f", "#f", "#t", "#t", "#f", "#f", "#t"]);
}

#[test]
fn test_logical_operations_2() {
    let code = r#"
        (print-bool (or #t #t #f))
        (print-bool (or #f (and #f #t) (not #f)))
        (print-bool (and #t (not #f) (or #f #t) (and #t (not #t))))
    "#;
    common::assert_evaluation(code, &["#t", "#t", "#f"]);
}
