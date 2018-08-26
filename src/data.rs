
use std::fmt;
use super::Token;
use std::collections::VecDeque;

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

pub fn from_token(tokens: &Vec<Token>) -> LuaData{
    match tokens.get(0).unwrap(){
        Token::StringLiteral(string) => LuaData::Str(string.to_owned()),
        Token::NumberLiteral(num) => parse_num_statement(*num, tokens),
        _ => panic!("Unable to convert token to data type: {:?}", tokens)
    }
}

pub fn parse_num_statement(start: i32, tokens: &Vec<Token>) -> LuaData{
    let val: i32 = start;

     let mut tokens_deque: VecDeque<Token> = VecDeque::new();

    for token in tokens{
        tokens_deque.push_back(token.clone());
    }

    loop{
        let token = tokens_deque.pop_front().unwrap_or_else(||{
            Token::EOF
        });

        match token{
            Token::EOF => break,
            _ => panic!("Unknown token found while parsing number statement: {:?}", token),
        }
    }

    LuaData::Number(val)
}
