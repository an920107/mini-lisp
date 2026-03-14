use std::{fmt, io, ops};

#[derive(Debug)]
pub enum Error {
    IOError(io::Error),
    LexicalError((usize, ops::Range<usize>), String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IOError(e) => write!(f, "io error: {}", e),
            Error::LexicalError((line, range), e) => write!(
                f,
                "lexical error at (line {}, column {}): {}",
                line,
                range.start + 1,
                e
            ),
        }
    }
}
