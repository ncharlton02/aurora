
use std::fmt;

#[derive(Debug)]
pub enum ErrorType{
    Lexical,
    Parse,
    Runtime,
}

#[derive(Debug)]
pub struct LuaError{
    error_type: ErrorType,
    message: String
}

impl fmt::Display for LuaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.error_type{
            ErrorType::Lexical => write!(f, "[Lexical Exception] {}", self.message),
            ErrorType::Parse => write!(f, "[Parse Exception] {}", self.message),
            ErrorType::Runtime => write!(f, "[Runtime Exception] {}", self.message),
        }
    }
}

impl LuaError{

    pub fn create(message: &str, error_type: ErrorType) -> LuaError{
        LuaError {error_type, message: message.to_string()}
    }

    pub fn create_lexical(message: &str) -> LuaError{
        LuaError::create(message, ErrorType::Lexical)
    }

    pub fn create_parse(message: &str) -> LuaError{
        LuaError::create(message, ErrorType::Parse)
    }

    pub fn create_runtime(message: &str) -> LuaError{
        LuaError::create(message, ErrorType::Runtime)
    }

}