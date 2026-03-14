mod common;

#[test]
fn test_recursion_1() {
    let code = r#"
        (define fact
        (fun (n) (if (< n 3) n
                    (* n (fact (- n 1))))))

        (print-num (fact 2))
        (print-num (fact 3))
        (print-num (fact 4))
        (print-num (fact 10))

        (define fib (fun (x)
        (if (< x 2) x (+
                        (fib (- x 1))
                        (fib (- x 2))))))

        (print-num (fib 1))
        (print-num (fib 3))
        (print-num (fib 5))
        (print-num (fib 10))
        (print-num (fib 20))
    "#;
    common::assert_evaluation(
        code,
        &["2", "6", "24", "3628800", "1", "2", "5", "55", "6765"],
    );
}

#[test]
fn test_recursion_2() {
    let code = r#"
        (define min
            (fun (a b)
                (if (< a b) a b)))

        (define max
            (fun (a b)
                (if (> a b) a b)))

        (define gcd
            (fun (a b)
                (if (= 0 (mod (max a b) (min a b)))
                    (min a b)
                    (gcd (min a b) (mod (max a b) (min a b))))))

        (print-num (gcd 100 88))
        (print-num (gcd 1234 5678))
        (print-num (gcd 81 54))
    "#;
    common::assert_evaluation(code, &["4", "2", "27"]);
}
