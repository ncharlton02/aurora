
use std::collections::HashMap;
use super::{Token, Stmt, StmtType};
use super::data::*;

type LuaFunc = (fn(Vec<Token>, &Interpreter) -> ());

struct Interpreter{
    funcs: HashMap<String, LuaFunc>,
    variables: HashMap<String, LuaData>
}

impl Interpreter{

    pub fn new() -> Interpreter{
        Interpreter {funcs: HashMap::new(), variables: HashMap::new()}
    }

    pub fn register_func(&mut self, name: String, func: LuaFunc){
        self.funcs.insert(name, func);
    }

    pub fn assign_variable(&mut self, name: String, data: LuaData){
        self.variables.insert(name, data);
    }

    pub fn get_variable(&self, name: String) -> Option<&LuaData>{
        self.variables.get(&name)
    }

    pub fn run_stmt(&mut self, stmt: &mut Stmt){
        match stmt.stmt_type{
            StmtType::FunctionCall(ref name, ref args) => self.run_function_call(name, args.to_vec()),
            StmtType::Assignment(ref name, ref tokens) => self.handle_assignment(name, tokens.to_vec()),
            StmtType::EOF => (),
        }
    }

    fn handle_assignment(&mut self, name: &Token, tokens: Vec<Token>){
         let name = match name{
            Token::Identifier(n) => n,
            _ => panic!("Illegal Token: expected identifier but found {:?}", name),
        };

        let value = super::data::from_token(&tokens);

        self.assign_variable(name.to_string(), value);
    }

    fn run_function_call(&mut self, name: &Token, args: Vec<Token>){        
        let name = match name{
            Token::Identifier(string) => string,
            _ => panic!("Illegal Token: expected identifier but found {:?}", name),
        };
        
        let func = self.funcs.get(name);

        if let Some(func) = func{
            func(args, self);
        }
    }
}

pub fn run(stmts: &mut Vec<Stmt>){
    let mut interpreter = Interpreter::new();

    interpreter.register_func("print".to_string(), |tokens, interpreter|{
        for token in tokens{
            match token{
                Token::StringLiteral(string) => println!("{}", string),
                Token::Identifier(string) => println!("{}", interpreter.get_variable(string).unwrap()),
                _ => (),
            }
        }
    });

    for mut stmt in stmts.iter_mut(){
        interpreter.run_stmt(&mut stmt);
    }
}