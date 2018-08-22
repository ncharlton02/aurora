
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
            StmtType::FunctionCall => self.run_function_call(&mut stmt.tokens),
            StmtType::Assignment => self.handle_assignment(&mut stmt.tokens),
            StmtType::EOF => (),
        }
    }

    fn handle_assignment(&mut self, tokens: &mut Vec<Token>){
        let value = super::data::from_token(tokens.remove(1));

        let name = match tokens.remove(0){
            Token::Identifier(n) => n,
            _ => panic!("oops"),
        };

        self.assign_variable(name, value);
    }

    fn run_function_call(&mut self, tokens: &mut Vec<Token>){
        let call = self.parse_function_call(tokens);
        
        let func = self.funcs.get(&call.name);

        if let Some(func) = func{
            func(call.args, self);
        }
    }
    
    fn parse_function_call(&self, tokens: &mut Vec<Token>) -> FunctionCall{
        let name = tokens.remove(0);
        let mut args = Vec::new();

        for token in tokens{
            if token.can_be_arg(){
                args.push(token.clone());
            }
        }

        let name = match name{
            Token::Identifier(x) => x,
             x => panic!("Unknown name token: {:?}", x),
        };

        FunctionCall{args, name}
    }
}

struct FunctionCall{
    args: Vec<Token>,
    name: String
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