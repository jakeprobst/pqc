use tokenizer::{Tokenizer, TokenizeError};
use parser::ParseError;
use semantic::SyntaxError;
use std;

#[derive(Debug)]
pub enum QuestError {
    Tokenizer(TokenizeError),
    Parser(ParseError),
    Semantic(SyntaxError),
}


impl std::convert::From<TokenizeError> for QuestError {
    fn from(err: TokenizeError) -> QuestError {
        QuestError::Tokenizer(err)
    }
}

impl std::convert::From<ParseError> for QuestError {
    fn from(err: ParseError) -> QuestError {
        QuestError::Parser(err)
    }
}

impl std::convert::From<SyntaxError> for QuestError {
    fn from(err: SyntaxError) -> QuestError {
        QuestError::Semantic(err)
    }
}

pub struct QuestBuilder {
}



impl QuestBuilder {
    pub fn new(script: String) -> Tokenizer {
        Tokenizer::new(script)
    }
}
