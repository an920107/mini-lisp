pub mod token;

use logos::Logos;
use std::io::{self, Stdin};

use crate::{error::Error, scanner::token::Token};

pub struct Scanner {
    stdin: Stdin,
    buffer: String,
    line_count: usize,
}

impl Scanner {
    pub fn new() -> Self {
        Self {
            stdin: io::stdin(),
            buffer: String::new(),
            line_count: 0,
        }
    }

    pub fn scan_line(&mut self) -> Result<Option<Vec<Token>>, Error> {
        self.line_count += 1;
        self.buffer.clear();

        let bytes = self
            .stdin
            .read_line(&mut self.buffer)
            .map_err(|e| Error::IOError(e))?;
        if bytes == 0 {
            return Ok(None);
        }

        let mut tokens: Vec<Token> = vec![];
        let lexer = Token::lexer(&self.buffer);
        for (token, span) in lexer.spanned() {
            match token {
                Ok(token) => tokens.push(token),
                Err(e) => return Err(Error::LexicalError((self.line_count, span), e)),
            }
        }

        Ok(Some(tokens))
    }
}
