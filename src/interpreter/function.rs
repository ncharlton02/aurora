
use super::{Interpreter};
use std::collections::{HashMap, HashSet};
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

///
/// Function Containers
/// 

pub struct FunctionManager{
    funcs: HashMap<i64, Function>,
    func_names: HashMap<String, i64>,
    func_count: i64,
}

impl FunctionManager{
    
    pub fn new() -> FunctionManager{
        FunctionManager{
            funcs: HashMap::new(), 
            func_names: HashMap::new(), 
            func_count : 0,
        }
    }

    pub fn register_func(&mut self, name: String, def: FunctionDef) -> i64{
        let id = self.func_count;
        self.func_count += 1;
        self.func_names.insert(name, id);
        self.funcs.insert(id, create_function(id, def));

        id
    }

    pub fn get_func_id(&self, name: &str) -> i64{
        *self.func_names.get(name).unwrap_or(&-1)
    }

    pub fn get_func(&self, id: i64) -> Option<&Function>{
        self.funcs.get(&id)
    }

}