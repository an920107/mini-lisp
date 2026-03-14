mod common;

#[test]
fn test_nested_function_1() {
    let code = r#"
        (define dist-square
            (fun (x y)
                (define square (fun (x) (* x x)))
                (+ (square x) (square y))))

        (print-num (dist-square 3 4))
    "#;
    common::assert_evaluation(code, &["25"]);
}

#[test]
fn test_nested_function_2() {
    let code = r#"
        (define diff
            (fun (a b)
                (define abs
                    (fun (a)
                        (if (< a 0) (- 0 a) a)))
                (abs (- a b))))

        (print-num (diff 1 10))
        (print-num (diff 10 2))
    "#;
    common::assert_evaluation(code, &["9", "8"]);
}
