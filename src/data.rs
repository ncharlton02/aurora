
use std::fmt;
use super::Token;

pub enum LuaData{
    Str(String),
    Number(i32)
}

impl fmt::Display for LuaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            LuaData::Str(string) => write!(f, "{}", string),
            LuaData::Number(number) => write!(f, "{}", number),
        }
    }
}

pub fn from_token(token: Token) -> LuaData{
    match token{
        Token::StringLiteral(string) => LuaData::Str(string),
        Token::NumberLiteral(num) => LuaData::Number(num),
        _ => panic!("Unable to convert token to data type: {:?}", token)
    }
}
