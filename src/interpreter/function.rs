
use super::{Interpreter, error};
use super::super::{Token, Stmt, data::LuaData, error::LuaError};

type RustFunc = fn(Vec<LuaData>, &mut Interpreter) -> Result<Option<LuaData>, LuaError>;

#[derive(Clone)]
pub struct LuaFunc{
    pub arg_defs: Vec<Token>,
    pub stmts: Vec<Stmt>,
}

impl LuaFunc{

    pub fn new(arg_defs: Vec<Token>, stmts: Vec<Stmt>) -> LuaFunc{
        LuaFunc{arg_defs: arg_defs, stmts: stmts}
    }

    pub fn execute(&mut self, arg_data: Vec<LuaData>, interpreter: &mut Interpreter) -> Result<Option<LuaData>, LuaError>{
        if self.arg_defs.len() != arg_data.len(){
            return Err(interpreter.error(format!("Incorrect number of arguments found! Expected {} but found {}", self.arg_defs.len(), arg_data.len())));
        }

        self.add_args(interpreter, arg_data)?;

        for stmt in &self.stmts{
            if let Some(_) = interpreter.return_val{
                break;
            }

            interpreter.run_stmt(&mut stmt.clone())?;
        }

        Ok(interpreter.return_val.clone())
    }

    fn add_args(&self, interpreter: &mut Interpreter, data: Vec<LuaData>) -> Result<(), LuaError>{
        for x in 0..self.arg_defs.len(){
            let name = match self.arg_defs.get(x).unwrap(){
                Token::Identifier(x) => x,
                x => return Err(interpreter.error(format!("Expected identifier but found {:?}", x))),    
            }.to_string();

            interpreter.assign_variable(name, data.get(x).unwrap().clone(), true)?
        }

        Ok(())
    }
}

#[derive(Clone)]
pub enum FunctionDef{
    Lua(LuaFunc),
    Rust(RustFunc)
}

#[derive(Clone)]
pub struct Function{
    pub def: FunctionDef,
    pub id: i64
}

pub fn create_function(id: i64, def: FunctionDef) -> Function{
    Function{def, id}
}