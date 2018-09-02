
use std::fmt;

#[derive(Clone, Debug)]
pub enum LuaData{
    Str(String),
    Number(f64),
    Bool(bool),
}

impl fmt::Display for LuaData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self{
            LuaData::Str(string) => write!(f, "{}", string),
            LuaData::Number(number) => write!(f, "{}", number),
            LuaData::Bool(b) => write!(f, "{}", b),
        }
    }
}