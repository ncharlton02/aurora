
pub mod parser;
pub mod interpreter;
pub mod data;
pub mod error;
pub mod config;

use config::{Config, LogLevel};
use error::LuaError;
use interpreter::Interpreter;

#[derive(Debug, PartialEq, Clone)]
pub enum BinOp{
    Concat,
    LessThan,
    LessEqualThan,
    GreaterThan,
    GreaterEqualThan,
    EqualEqual,
    Plus,
    Minus, 
    Multiply,
    Divide
}

#[derive(Debug, PartialEq, Clone)]
pub enum Keyword{
    True, False, If, Then, Else, End, Function, Return, Local, While, Do
}

impl Keyword{

    pub fn vec() -> Vec<String>{
        vec!["true", "false", "if", "else", "then", "end", "function", "return", "local", "while", "do"].iter().map(|x| x.to_string()).collect()
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
            "local" => Keyword::Local,
            "while" => Keyword::While,
            "do" => Keyword::Do,
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
    Semicolon,
    Newline,
    Comma,
    LeftBrace,
    RightBrace,
    Equal,
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
    ///Name, Assignment, Is Local,
    Assignment(Token, Expr, bool),
    ///Operator, Left Token, Right Token
    BinOp(BinOp, Expr, Expr),
    ///A single token value
    Value(Vec<Token>),
    ///Condition, Stmts, Else
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    //Condition, Block
    While(Expr, Vec<Stmt>),
    Return(Expr),
    EOF
}

impl StmtType{

    fn stmt_count_recursive(&self) -> u32{
        match self{
            StmtType::Return(_) | StmtType::Assignment(_, _, _) | StmtType::BinOp(_, _, _) | 
            StmtType::FunctionCall(_, _) | StmtType::EOF | StmtType::Value(_) => 1,
            StmtType::If(_, block, else_block) => {
                let mut count = 1 + count_stmts_recur(block);

                if let Some(else_block) = else_block{
                    count += count_stmts_recur(else_block);
                }

                count
            },
            StmtType::FunctionDef(_, _, block) | StmtType::While(_, block) => {
                1 + count_stmts_recur(block)
            } 
        }
    }

}

pub fn count_stmts_recur(stmts: &Vec<Stmt>) -> u32{
    let mut count = 0;

    for x in stmts {
        count += x.stmt_type.stmt_count_recursive();
    }

    count
}

#[derive(Debug, PartialEq, Clone)]
enum ExprType{
    /// Contains a string concat
    Str, 
    // Contains a number bin op (+, -, etc.)
    Number, 
    // Contains a bool bin op (==, <, <=, etc.)
    Bool,
    // A single value i.e. '55' or '"Hello World"'
    SingleValue
}


#[derive(Debug, PartialEq, Clone)]
pub struct Expr{
    stmts: Vec<Stmt>,
    expr_type: ExprType
}

pub struct Aurora{
    interpreter: Interpreter,
    config: Config
}

impl Aurora{
    
    pub fn new(config: Config) -> Aurora{
        let mut interpreter = Interpreter::new();

        interpreter.load_library(interpreter::library::new_std());
        Aurora{interpreter: interpreter, config: config}
    }

    pub fn register_function(&mut self, name: String, function: interpreter::function::FunctionDef){
        self.interpreter.register_func(name, function);
    }

    pub fn run(&mut self, src: String) -> Result<(), Vec<LuaError>>{
        let tokens = parser::scanner::scan(src)?;
        self.print_token_info(&tokens);

        let mut stmts = match parser::parse(tokens){
            Ok(x) => x,
            Err(e) => return Err(vec![e])
        };
        self.print_stmt_info(&stmts);

        match self.run_stmts(&mut stmts){
            Err(e) => return Err(vec![e]),
            _ => (),
        };

        Ok(())
    }

    pub fn run_stmts(&mut self, stmts: &mut Vec<Stmt>) -> Result<(), LuaError>{
        if self.config.log_level == LogLevel::Verbose{
            println!("\n---------- Running -------");
        }

        for stmt in stmts.iter_mut(){
            self.interpreter.run_stmt(stmt)?;
        }

        if self.config.log_level == LogLevel::Verbose{
            println!("\n---------- Finished -------");
        }

        Ok(())
    }

    fn print_token_info(&self, tokens: &Vec<Token>){
        if self.config.log_level != LogLevel::Verbose{
            return;
        }

        println!("Token Count: {}", tokens.len());
        for token in tokens{
            println!("{:?}", token);
        }
    }

    fn print_stmt_info(&self, stmts: &Vec<Stmt>){
        if self.config.log_level != LogLevel::Verbose{
            return;
        }

        println!("Stmt Count: {}", count_stmts_recur(stmts));
        for stmt in stmts{
            println!("{:#?}", stmt);
        }
    }


}