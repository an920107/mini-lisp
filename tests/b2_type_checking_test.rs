use std::io::Cursor;

use mini_lisp::error::Error;

mod common;

#[test]
fn test_type_checking_1() {
    let code = "(+ 1 2 3 (or #t #f))";
    let mut evaluator = common::setup_evaluator(code.into());
    let mut buffer = Cursor::new(Vec::new());
    assert!(matches!(
        evaluator.evaluate(&mut buffer),
        Err(Error::SemanticError(_))
    ));
}

#[test]
fn test_type_checking_2() {
    let code = r#"
        (define f
            (fun (x)
                (if (> x 10) 10 (= x 5))))

        (print-num (* 2 (f 4)))
    "#;
    let mut evaluator = common::setup_evaluator(code.into());
    let mut buffer = Cursor::new(Vec::new());
    assert!(matches!(
        evaluator.evaluate(&mut buffer),
        Err(Error::SemanticError(_))
    ));
}
