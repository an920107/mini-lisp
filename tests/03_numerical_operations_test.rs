mod common;

#[test]
fn test_numerical_operations_1() {
    let code = r#"
        (+ 1 2 3)
        (* 4 5 6)

        (print-num (+ 1 (+ 2 3 4) (* 4 5 6) (/ 8 3) (mod 10 3)))

        (print-num (mod 10 4))

        (print-num (- (+ 1 2) 4))

        (print-num -256)
    "#;
    common::assert_evaluation(code, &["133", "2", "-1", "-256"]);
}

#[test]
fn test_numerical_operations_2() {
    let code = r#"
        (print-num (mod 10 (+ 1 2)))

        (print-num (* (/ 1 2) 4))

        (print-num (- (+ 1 2 3 (- 4 5) 6 (/ 7 8) (mod 9 10))
                    11))
    "#;
    common::assert_evaluation(code, &["1", "0", "9"]);
}
