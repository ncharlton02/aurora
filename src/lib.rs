
mod scanner;

#[derive(Debug)]
pub enum LuaResult{
    Successful, 
    Failure
}

#[derive(Debug, PartialEq)]
pub enum Token{
    Identifier(String), 
    StringLiteral(String),
    LeftParenthesis,
    RightParenthesis,
    Unknown,
    EOF 
}

pub fn run(src: String) -> LuaResult{
    let tokens = scanner::scan(src);

    println!("Token Count: {}", tokens.len());

    for token in tokens{
        println!("{:?}", token);
    }

    LuaResult::Successful
}