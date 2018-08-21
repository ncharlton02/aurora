
mod scanner;
mod parser;

#[derive(Debug)]
pub enum LuaResult{
    Successful, 
    Failure
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token{
    Identifier(String), 
    StringLiteral(String),
    LeftParenthesis,
    RightParenthesis,
    EOF 
}

#[derive(Debug)]
pub struct Stmt{
    stmt_type: StmtType,
    tokens: Vec<Token>
}

#[derive(Debug, PartialEq)]
pub enum StmtType{
    FunctionCall,
    Assignment,
    EOF
}


pub fn run(src: String) -> LuaResult{
    let tokens = scanner::scan(src);
    print_token_info(&tokens);
    println!("\n");

    let stmts = parser::parse(tokens);
    print_stmt_info(&stmts);
    println!("\n");

    LuaResult::Successful
}

fn print_stmt_info(stmts: &Vec<Stmt>){
    println!("Stmt Count: {}", stmts.len());

    for stmt in stmts{
        println!("{:?}", stmt);
    }
}

fn print_token_info(tokens: &Vec<Token>){
    println!("Token Count: {}", tokens.len());

    for token in tokens{
        println!("{:?}", token);
    }
}