use std::num::ParseIntError;

use logos::{Lexer, Logos};

#[derive(Logos, Debug, Clone, PartialEq, Eq)]
#[logos(skip r"[ \t\n\r]+")]
#[logos(error(String, handle_lexing_error_callback))]
pub enum Token {
    #[token("(")]
    LeftBracket,
    #[token(")")]
    RightBracket,

    #[regex(r"[\+\-]?[0-9]+", handle_integer_callback)]
    Integer(i128),
    #[regex(r"#t|#f", handle_boolean_callback)]
    Boolean(bool),

    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Multiply,
    #[token("/")]
    Divide,
    #[token("mod")]
    Modulo,
    #[token(">")]
    Greater,
    #[token("<")]
    Less,
    #[token("=")]
    Equal,
    #[token("and")]
    And,
    #[token("or")]
    Or,
    #[token("not")]
    Not,

    #[token("print-num")]
    PrintInteger,
    #[token("print-bool")]
    PrintBoolean,
    #[token("define")]
    Define,
    #[token("fun")]
    Function,
    #[token("if")]
    If,

    #[regex(r"[a-z][a-z\-]*", handle_string_callback)]
    Symbol(String),
}

fn handle_lexing_error_callback(_: &mut Lexer<Token>) -> String {
    format!("unexpected input")
}

fn handle_integer_callback(lex: &mut Lexer<Token>) -> Result<i128, String> {
    lex.slice()
        .parse()
        .map_err(|e: ParseIntError| e.to_string())
}

fn handle_boolean_callback(lex: &mut Lexer<Token>) -> bool {
    match lex.slice() {
        "#t" => true,
        "#f" => false,
        _ => unreachable!(),
    }
}

fn handle_string_callback(lex: &mut Lexer<Token>) -> String {
    lex.slice().to_string()
}
