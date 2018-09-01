
use std::fmt;

#[derive(Clone)]
pub enum LuaData{
    Str(String),
    Number(i32),
}

impl fmt::Display for LuaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            LuaData::Str(string) => write!(f, "{}", string),
            LuaData::Number(number) => write!(f, "{}", number),
        }
    }
}