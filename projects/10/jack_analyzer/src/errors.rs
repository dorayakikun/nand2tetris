use std::fmt::{Display, Formatter, Result};
#[derive(Debug)]
pub enum JackAnalyzerError {
    InvalidArgumentLength(usize),
}

impl Display for JackAnalyzerError {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use self::JackAnalyzerError::*;
        match self {
            InvalidArgumentLength(length) => writeln!(f, "InvalidArgumentLength: {}", length),
        }
    }
}

impl Error for JackAnalyzerError {}
