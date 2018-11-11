
use std::io::prelude::*;
use std::fs::File;
use std::collections::{HashMap};
use super::{Token, Stmt, StmtType, Expr, BinOp, Keyword, parser};
use super::{data::*, error::LuaError};

use self::function::{Function, FunctionDef, LuaFunc};
use self::table::Table;

pub mod function;
pub mod table;

pub struct Interpreter{
    funcs: HashMap<i64, Function>,
    func_names: HashMap<String, i64>,
    func_count: i64,
    tables: HashMap<i64, Table>,
    table_count: i64,
    globals: HashMap<String, LuaData>,
    stack: Vec<HashMap<String, LuaData>>,
    return_val: Option<LuaData>
}

impl Interpreter{

    pub fn new() -> Interpreter{
        let mut interpreter = Interpreter {
            funcs: HashMap::new(), 
            func_names: HashMap::new(), 
            func_count : 0,
            tables: HashMap::new(),
            table_count: 0,
            globals: HashMap::new(),
            stack: vec![HashMap::new()],
            return_val: None,
        };

        interpreter.register_func("print".to_string(), FunctionDef::Rust(|args, _| -> Result<Option<LuaData>, LuaError>{
            for arg in args{
                print!("{}\t", arg);
            }

            println!();
            Ok(None)
        }));

        interpreter.register_func("fail".to_string(), FunctionDef::Rust(|args, _| -> Result<Option<LuaData>, LuaError>{
            if args.len() != 1{
                return Err(error(format!("Expected one argument, found {}", args.len())));
            }

            let message = match args.get(0).unwrap(){
                LuaData::Str(x) => x,
                x => return Err(error(format!("Expected string, found {}", x)))
            };

            Err(error(format!("{}", message)))
        }));

        interpreter.register_func("require".to_string(), FunctionDef::Rust(|args, interpreter| -> Result<Option<LuaData>, LuaError>{
            if args.len() != 1{
                return Err(error(format!("Expected one argument, found {}", args.len())));
            }

            let path = match args.get(0).unwrap(){
                LuaData::Str(x) => x,
                x => return Err(error(format!("Expected string, found {}", x)))
            };

            let src = load_file(path)?;
            let module = load_module(src, interpreter)?;
    
            Ok(Some(module))
        }));

        interpreter
    }

    pub fn register_func(&mut self, name: String, def: FunctionDef) -> i64{
        let id = self.func_count;
        self.func_count += 1;
        self.func_names.insert(name, id);
        self.funcs.insert(id, function::create_function(id, def));

        id
    }

    fn create_table(&mut self) -> i64{
        let id = self.table_count;
        self.table_count += 1;
        self.tables.insert(id, Table::new());

        id
    }

    pub fn assign_variable(&mut self, name: String, data: LuaData, is_local: bool) -> Result<(), LuaError>{
        if name.contains('.'){
            let (path, variable_name) = split_name_path(name);

            let table = self.get_variable(path)?.unwrap_or(&LuaData::Nil).clone();

            match table{
                LuaData::Table(id) => {
                let table = self.get_table_mut(id);

                    if let Some(table) = table{
                        table.assign_variable(variable_name, data);
                    }else{
                        panic!("Error: found invalid table id: {}", id);
                    }
                }
                x => return Err(error(format!("Expected table, found: {}", x))),
            }
            

            return Ok(());
        }

        if is_local || self.stack.last().unwrap().contains_key(&name){
            let index = self.stack.len() - 1;
            let frame = &mut self.stack[index];

            frame.insert(name, data);
            return Ok(());
        }

        self.globals.insert(name, data);
        Ok(())
    }

    pub fn get_variable(&self, name: String) -> Result<Option<&LuaData>, LuaError>{
        if name.contains('.'){
            let (table, variable) = split_name_path(name);

            return self.get_table_variable(table, variable);
        }

        if let Some(var) = self.stack.last().unwrap().get(&name){
            return Ok(Some(var));
        }

        Ok(self.globals.get(&name))
    }

     pub fn get_table_variable(&self, table: String, name: String) -> Result<Option<&LuaData>, LuaError>{
        let table = self.get_variable(table)?;

        if let Some(table) = table{
            match table{
                LuaData::Table(id) => {
                    let table = self.get_table(*id);

                    if let Some(table) = table{
                        Ok(table.get_variable(name))
                    }else{
                        panic!("Error: found invalid table id: {}", id);
                    }
                }
                x => Err(error(format!("Expected table found: {}", x)))
            }
        }else{
            Ok(None)
        }
    }

    pub fn get_table(&self, id: i64) -> Option<&Table>{
        self.tables.get(&id)
    }

     pub fn get_table_mut(&mut self, id: i64) -> Option<&mut Table>{
        self.tables.get_mut(&id)
    }

    pub fn get_variable_mut(&mut self, name: &str) -> Result<Option<&mut LuaData>, LuaError>{
         if name.contains('.'){
            let (path, variable_name) = split_name_path(name.to_string());

            return self.get_table_variable_mut(path, variable_name);
        }

        if let Some(var) = self.stack.last_mut().unwrap().get_mut(name){
            return Ok(Some(var));
        }

        Ok(self.globals.get_mut(name))
    }

    pub fn get_table_variable_mut(&mut self, table: String, name: String) -> Result<Option<&mut LuaData>, LuaError>{
        let table = self.get_variable(table)?.unwrap_or(&LuaData::Nil).clone();

        match table{
            LuaData::Table(id) => {
                let table = self.get_table_mut(id);

                if let Some(table) = table{
                    Ok(table.get_variable_mut(name))
                }else{
                    panic!("Error: found invalid table id: {}", id);
                }
            },
            x => Err(error(format!("Expected table found: {}", x)))
        }
    }
    
    pub fn run_stmt(&mut self, stmt: &mut Stmt) -> Result<(), LuaError>{
        if let Some(_) = self.return_val{
            return Ok(());
        }

        match stmt.stmt_type{
            StmtType::FunctionDef(ref name, ref args, ref block) => self.handle_func_def(name, args, block),
            StmtType::If(ref expr, ref mut stmts, ref mut else_block) => self.run_if_stmt(expr, stmts, else_block),
            StmtType::Assignment(ref name, ref expr, ref is_local) => self.handle_assignment(name, expr, *is_local),
            StmtType::BinOp(_, _, _) | StmtType::Value(_) => panic!("Illegal Root Stmt: {:?}", stmt),
            StmtType::Return(ref expr) => self.handle_return(expr),
            StmtType::While(ref expr, ref mut stmts) => self.run_while_loop(expr, stmts),
            StmtType::FunctionCall(ref name, ref args) => {
                match self.run_function_call(name, args.to_vec()){
                    Ok(_) => Ok(()),
                    Err(e) => Err(e),
                }
            },
            StmtType::EOF => Ok(()),
        }
    }

    fn handle_return(&mut self, expr: &Expr) -> Result<(), LuaError>{
        self.return_val = Some(self.evaluate_expr(expr)?);
        Ok(())
    }

    fn handle_func_def(&mut self, name: &Token, args: &Vec<Token>, stmts: &Vec<Stmt>) -> Result<(), LuaError>{
        let name = match name{
            Token::Identifier(x) => x,
            x => return Err(error(format!("Expected identifer but found {:?}", x))),
        };

        let func = LuaFunc::new(args.to_vec(), stmts.to_vec());

        let id = self.register_func(name.to_string(), FunctionDef::Lua(func));

        if name.contains('.'){
            let (path, variable_name) = split_name_path(name.to_string());

            let table = self.get_variable(path)?.unwrap_or(&LuaData::Nil).clone();

            //TODO do we need this?
            match table{
                LuaData::Table(table_id) => {
                    let table = self.get_table_mut(table_id);

                    if let Some(table) = table{
                        table.assign_variable(variable_name, LuaData::Func(id));
                    }else{
                        panic!("Error: found invalid table id: {}", id);
                    }
                },
                x => return Err(error(format!("Expected table, found {:?}", x))),
            }
        }

        Ok(())
    }

    fn run_while_loop(&mut self, expr: &Expr, stmts: &mut Vec<Stmt>) -> Result<(), LuaError>{
        while self.should_run(expr)?{
            for stmt in stmts.iter_mut(){
                self.run_stmt(stmt)?;
            }
        }

        Ok(())
    }

    fn run_if_stmt(&mut self, expr: &Expr, stmts: &mut Vec<Stmt>, else_block: &mut Option<Vec<Stmt>>) -> Result<(), LuaError>{
        let should_run = self.should_run(expr)?;

        if should_run{
            for stmt in stmts{
                self.run_stmt(stmt)?;
            }
        }else if let Some(else_block) = else_block{
            for stmt in else_block{
                self.run_stmt(stmt)?;
            }
        }

        Ok(())
    }

    fn should_run(&mut self, expr: &Expr) -> Result<bool, LuaError>{
        Ok(self.evaluate_expr(expr)?.to_bool())
    }

    fn handle_assignment(&mut self, name: &Token, expr: &Expr, is_local: bool) -> Result<(), LuaError>{
         let name = match name{
            Token::Identifier(n) => n,
            _ => return Err(error(format!("Illegal Token: expected identifier but found {:?}", name))),
        };

        let value = self.evaluate_expr(expr)?;

        self.assign_variable(name.to_string(), value, is_local)
    }

    fn evaluate_expr(&mut self, expr: &Expr) -> Result<LuaData, LuaError>{
        let stmt = expr.stmts.get(0).unwrap();
        
        match stmt.stmt_type{
            StmtType::BinOp(ref operator, ref left, ref right) => Ok(self.evaluate_bin_op(operator, left, right)?),
            StmtType::Value(ref tokens) => Ok(self.evaluate_value_expr(tokens)?),
            ref x => Err(error(format!("Couldn't evaluate expression: {:?}", x))),
        }
    }

    fn evaluate_value_expr(&mut self, tokens: &Vec<Token>) -> Result<LuaData, LuaError>{
        let first_token = tokens.get(0).unwrap();

        Ok(match first_token{
            Token::NumberLiteral(x) => LuaData::Number(x.clone()),
            Token::StringLiteral(x) => LuaData::Str(x.clone()),
            Token::Keyword(Keyword::True) => LuaData::Bool(true),
            Token::Keyword(Keyword::False) => LuaData::Bool(false),
            Token::LeftBrace =>{
                if tokens.len() == 2{
                    let next = tokens.get(1);

                    if next == Some(&Token::RightBrace){
                        LuaData::Table(self.create_table())
                    }else{
                        return Err(error(format!("Expected right curly brace but found: {:?}", next)))
                    }
                }else{
                    return Err(error(format!("Failed to parse value for left curly brace!")))
                }
            },
            Token::Identifier(x) => {
                match tokens.get(1){
                    Some(Token::LeftParenthesis) => {
                        let mut stmts = match parser::parse(tokens.to_vec()){
                            Ok(x) => x,
                            Err(e) => return Err(e),
                        };

                        match stmts.remove(0).stmt_type{
                            StmtType::FunctionCall(ref name, ref args) => self.run_function_call(name, args.to_vec())?,
                            x => return Err(error(format!("Expected to find function but found {:?}", x))),
                        }
                    },
                    None => {
                        if let Some(val) = self.get_variable(x.to_string())?{
                            val.clone()
                        }else{
                            LuaData::Nil
                        }
                    },
                    x => return Err(error(format!("Unexpected token: {:?}", x))),
                }
            },
            _ => return Err(error(format!("Illegal Token: {:?} isn't a value", first_token))),
        })
    }

    fn evaluate_bin_op(&mut self, operator: &BinOp, left: &Expr, right: &Expr) -> Result<LuaData, LuaError>{   
        Ok(match operator{
            BinOp::Concat => self.evaluate_str_binop(left, right)?,
            BinOp::EqualEqual => self.evaluate_equallity_expr(left, right)?,
            _ => self.evaluate_num_binop(operator, left, right)?,
        })
    }

    fn evaluate_num_binop(&mut self, operator: &BinOp, left: &Expr, right: &Expr) -> Result<LuaData, LuaError>{
        let left_num = self.expr_to_num(left)?;

        let right_num = self.expr_to_num(right)?;

        Ok(match operator{
            BinOp::Plus => LuaData::Number(left_num + right_num),
            BinOp::Minus => LuaData::Number(left_num - right_num),
            BinOp::Multiply => LuaData::Number(left_num * right_num),
            BinOp::Divide => LuaData::Number(left_num / right_num),
            BinOp::LessThan => LuaData::Bool(left_num < right_num),
            BinOp::LessEqualThan => LuaData::Bool(left_num <= right_num),
            BinOp::GreaterThan => LuaData::Bool(left_num > right_num),
            BinOp::GreaterEqualThan => LuaData::Bool(left_num >= right_num),
            BinOp::EqualEqual => LuaData::Bool(left_num == right_num),
            _ => return Err(error(format!("Unknown num operator: {:?}!", operator))),
        })
    }

    fn evaluate_str_binop(&mut self, left: &Expr, right: &Expr) -> Result<LuaData, LuaError>{
        let left_string = self.expr_to_string(left)?;
        let right_string = self.expr_to_string(right)?;

        Ok(LuaData::Str(format!("{}{}", left_string, right_string)))
    }

    fn evaluate_equallity_expr(&mut self, left: &Expr, right: &Expr) -> Result<LuaData, LuaError>{
        let left = self.evaluate_expr(left)?;
        let right = self.evaluate_expr(right)?;

        return Ok(LuaData::Bool(right.to_string() == left.to_string()));
    }

    fn expr_to_string(&mut self, expr: &Expr) -> Result<String, LuaError>{
        let value = self.evaluate_expr(expr)?;

        Ok(value.to_string())
    }

     fn expr_to_num(&mut self, expr: &Expr) -> Result<f64, LuaError>{
        let value = self.evaluate_expr(expr)?;

        Ok(value.to_num())
    }

    fn run_function_call(&mut self, name: &Token, args: Vec<Expr>) -> Result<LuaData, LuaError>{        
        let name = match name{
            Token::Identifier(string) => string,
            _ => return Err(error(format!("Illegal Token: expected identifier but found {:?}", name))),
        };
        
        let func_id = self.get_function_id_from_identifier(name)?;
        let arg_data = self.evaluate_args(args)?;
        let func = match self.funcs.get(&func_id){
            Some(x) => x,
            None => return Err(error(format!("Unable to find function with name: {}", name))),   
        }.clone();

        self.stack.push(HashMap::new());
      
        let result = match func.def{
            FunctionDef::Rust(func) => func(arg_data, self)?,
            FunctionDef::Lua(mut func) => func.execute(arg_data, self)?,
        }.unwrap_or(LuaData::Nil);

        self.stack.pop();
        self.return_val = None;

        Ok(result)
    }

    fn get_function_id_from_identifier(&self, id: &String) -> Result<i64, LuaError>{
        if id.contains('.'){
            let(path, variable) = split_name_path(id.to_string());

            if let Some(LuaData::Func(id)) = self.get_table_variable(path, variable)?{
                Ok(*id)
            }else{
                Ok(-1)
            }
        }else{
            Ok(*self.func_names.get(id).unwrap_or(&-1))
        }
    }

    fn evaluate_args(&mut self, exprs: Vec<Expr>) -> Result<Vec<LuaData>, LuaError>{
        let mut data = Vec::new();

        for expr in exprs{
            data.push(self.evaluate_expr(&expr)?);
        }

        Ok(data)
    }

    fn load_module(&mut self, mut stmts: Vec<Stmt>) -> Result<LuaData, LuaError>{
        self.stack.push(HashMap::new());
        //run code
        //get return value
        for stmt in stmts.iter_mut(){
            self.run_stmt(stmt)?;

            match self.return_val{
                Some(LuaData::Table(_)) => break,
                _ => (),
            }
        }

        let return_value = self.return_val.clone().unwrap_or(LuaData::Nil);
        self.return_val = None;
        self.stack.pop();

        Ok(return_value)
    }
}

/// Splits a variable name multiple parts
/// Ex: 'foo.bar.baz' returns ('foo.bar', 'baz')
/// 
/// ```
/// use aurora::interpreter;
/// 
/// let input  = "foo.bar.baz".to_string();
/// 
/// let (path, name) = interpreter::split_name_path(input);
/// 
/// assert_eq!("foo.bar", path);
/// assert_eq!("baz", name);
/// ```
pub fn split_name_path(input: String) -> (String, String){
    let mut split_string: Vec<String> = input.split('.').map(|s| s.to_string()).collect();
    let variable = split_string.pop().unwrap();
    let mut path = String::new();

    for s in split_string{
        path.push_str(&s);
        path.push_str(".");
    }
    let len = path.len();
    path.truncate(len - 1);

    (path, variable)
}


fn error(message: String) -> LuaError{
    LuaError::create_runtime(&message)
}

fn load_file(name: &str) -> Result<String, LuaError>{
    let path = format!("assets/{}.lua", name);

    let mut file = match File::open(&path){
        Ok(x) => x,
        Err(e) => return Err(error(format!("Failed to load file {}.lua: {}", name, e)))
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents){
        Ok(_) => (),
        Err(e) => return Err(error(format!("Failed to load file {}.lua: {}", name, e)))
    }
    
    Ok(contents)
}

fn load_module(src: String, interpreter: &mut Interpreter) -> Result<LuaData, LuaError>{
    let tokens = match super::parser::scanner::scan(src){
        Ok(x) => x,
        Err(errors) => {
            let mut message = String::new();

            for error in errors{
                message.push_str(&error.message);
                message.push_str("\n")
            }

            return Err(error(message));
        },
    };
    let stmts = super::parser::parse(tokens)?;

    Ok(interpreter.load_module(stmts)?)
}

pub fn run(stmts: &mut Vec<Stmt>) -> Result<Interpreter, LuaError>{
    let mut interpreter = Interpreter::new();

    for mut stmt in stmts.iter_mut(){
        match interpreter.run_stmt(&mut stmt){
            Err(e) => return Err(e),
            _ => (),
        };
    }

    Ok(interpreter)
}