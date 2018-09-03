
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
    LessThan,
    LessEqualThan,
    GreaterThan,
    GreaterEqualThan,
    Equal,
    Plus,
    Minus, 
    Multiply,
    Divide
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword{
    True, False, If, Then, Else, End, Function, Return
}

impl Keyword{

    pub fn vec() -> Vec<String>{
        vec!["true", "false", "if", "else", "then", "end", "function", "return"].iter().map(|x| x.to_string()).collect()
    }

    pub fn is_keyword(string: &str) -> bool{
        if Keyword::vec().contains(&string.to_string()){
            return true;
        }
        
        false
    }

    pub fn from_string(string: &str) -> Keyword{
        match string{
            "true" => Keyword::True,
            "false" => Keyword::False,
            "if" => Keyword::If,
            "else" => Keyword::Else,
            "then" => Keyword::Then,
            "end" => Keyword::End,
            "function" => Keyword::Function,
            "return" => Keyword::Return,
            _ => panic!("Couldn't convert string to keyword: {}", string),
        }
    }

}

#[derive(Debug, PartialEq, Clone)]
pub enum Token{
    Identifier(String), 
    StringLiteral(String),
    NumberLiteral(f64),
    Operator(BinOp),
    Keyword(Keyword),
    LeftParenthesis,
    RightParenthesis,
    Newline,
    Comma,
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

#[derive(Debug, PartialEq, Clone)]
pub struct Stmt{
    stmt_type: StmtType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StmtType{
    ///Name, Arguments
    FunctionCall(Token, Vec<Expr>),
    ///Name, Arguments, Stmts
    FunctionDef(Token, Vec<Token>, Vec<Stmt>),
    ///Name, Assignment
    Assignment(Token, Expr),
    ///Operator, Left Token, Right Token
    BinOp(BinOp, Token, Token),
    ///A single token value
    Value(Vec<Token>),
    ///Condition, Stmts, Else
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    ///
    Return(Expr),
    EOF
}

#[derive(Debug, PartialEq, Clone)]
enum ExprType{
    Str, Number, Bool, SingleValue
}


#[derive(Debug, PartialEq, Clone)]
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