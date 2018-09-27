
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
    location: Option<String>,
    message: String
}

impl fmt::Display for LuaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let location = if let Some(ref x) = self.location{
            format!(" at {}", x)
        }else{
            String::new()
        };

        match self.error_type{
            ErrorType::Lexical => write!(f, "[Lexical Exception{}] {}", location, self.message),
            ErrorType::Parse => write!(f, "[Parse Exception{}] {}", location, self.message),
            ErrorType::Runtime => write!(f, "[Runtime Exception{}] {}", location, self.message),
        }
    }
}

impl LuaError{

    pub fn create(message: &str, error_type: ErrorType, location: Option<String>) -> LuaError{
        LuaError {error_type, message: message.to_string(), location}
    }

    pub fn create_lexical(message: &str, location: Option<String>) -> LuaError{
        LuaError::create(message, ErrorType::Lexical, location)
    }

    pub fn create_parse(message: &str, location: Option<String>) -> LuaError{
        LuaError::create(message, ErrorType::Parse, location)
    }

    pub fn create_runtime(message: &str) -> LuaError{
        LuaError::create(message, ErrorType::Runtime, None)
    }

}