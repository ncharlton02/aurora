
mod scanner;
mod parser;
mod interpreter;
mod data;
mod expr;

#[cfg(test)]
mod test;

#[derive(Debug)]
pub enum LuaResult{
    Successful, 
    Failure
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinOp{
    Concat,
    Equal,
    Plus,
    Minus, 
    Multiply,
    Divide
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token{
    Identifier(String), 
    StringLiteral(String),
    NumberLiteral(i32),
    Operator(BinOp),
    LeftParenthesis,
    RightParenthesis,
    Newline,
    EOF 
}

impl Token{

    pub fn can_be_arg(&self) -> bool{
        match self{
            Token::Identifier(_) | Token::StringLiteral(_) => true,
            _ => false
        }
    }

} 

#[derive(Debug, PartialEq)]
pub struct Stmt{
    stmt_type: StmtType,
}

#[derive(Debug, PartialEq)]
pub enum StmtType{
    ///Name, Arguments
    FunctionCall(Token, Vec<Token>),
    ///Name, Assignment
    Assignment(Token, Expr),
    ///Operator, Left Token, Right Token
    BinOp(BinOp, Token, Token),
    ///A single token value
    Value(Token),
    EOF
}

#[derive(Debug, PartialEq, Clone)]
enum ExprType{
    Str, Number, SingleValue
}


#[derive(Debug, PartialEq)]
pub struct Expr{
    stmts: Vec<Stmt>,
    expr_type: ExprType
}

pub fn run(src: String) -> LuaResult{
    let tokens = scanner::scan(src);
    print_token_info(&tokens);
    println!("");

    let mut stmts = parser::parse(tokens);
    print_stmt_info(&stmts);

    println!("\n---------- Running -------");
    interpreter::run(&mut stmts);
    println!("---------- Finished -------");

    LuaResult::Successful
}

fn print_stmt_info(stmts: &Vec<Stmt>){
    println!("Stmt Count: {}", stmts.len());

    for stmt in stmts{
        println!("{:#?}", stmt);
    }
}

fn print_token_info(tokens: &Vec<Token>){
    println!("Token Count: {}", tokens.len());

    for token in tokens{
        println!("{:?}", token);
    }
}