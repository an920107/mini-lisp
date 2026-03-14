use mini_lisp::error::Error;

mod common;

#[test]
fn test_syntax_validation_1() {
    let mut parser = common::setup_parser("(+)");
    assert!(matches!(parser.parse(), Err(Error::SyntaxError(_))));
}

#[test]
fn test_syntax_validation_2() {
    let mut parser = common::setup_parser("(+ (* 5 2) -)");
    assert!(matches!(parser.parse(), Err(Error::SyntaxError(_))));
}
