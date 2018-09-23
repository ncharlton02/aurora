
use std::collections::{HashMap};
use super::{Token, Stmt, StmtType, Expr, BinOp, Keyword, parser};
use super::{data::*, error::LuaError};

use self::function::{Function, LuaFunc};

mod function;

pub struct Interpreter{
    funcs: HashMap<String, Function>,
    globals: HashMap<String, LuaData>,
    stack: Vec<HashMap<String, LuaData>>,
    return_val: Option<LuaData>
}

impl Interpreter{

    pub fn new() -> Interpreter{
        Interpreter {
            funcs: HashMap::new(), 
            globals: HashMap::new(),
            stack: vec![HashMap::new()],
            return_val: None,
        }
    }

    pub fn register_func(&mut self, name: String, func: Function){
        self.funcs.insert(name, func);
    }

    pub fn assign_variable(&mut self, name: String, data: LuaData, is_local: bool){
        if is_local || self.stack.last().unwrap().contains_key(&name){
            let index = self.stack.len() - 1;
            let frame = &mut self.stack[index];

            frame.insert(name, data);
            return;
        }

        self.globals.insert(name, data);
    }

    pub fn get_variable(&self, name: String) -> Option<&LuaData>{
        if let Some(var) = self.stack.last().unwrap().get(&name){
            return Some(var);
        }

        self.globals.get(&name)
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

        self.register_func(name.to_string(), Function::Lua(func));

        Ok(())
    }


    fn run_if_stmt(&mut self, expr: &Expr, stmts: &mut Vec<Stmt>, else_block: &mut Option<Vec<Stmt>>) -> Result<(), LuaError>{
        let should_run = match self.evaluate_expr(expr)?{
            LuaData::Bool(b) => b,
            x => return Err(error(format!("Expected boolean but found {}", x))),
        };

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

    fn handle_assignment(&mut self, name: &Token, expr: &Expr, is_local: bool) -> Result<(), LuaError>{
         let name = match name{
            Token::Identifier(n) => n,
            _ => return Err(error(format!("Illegal Token: expected identifier but found {:?}", name))),
        };

        let value = self.evaluate_expr(expr)?;

        self.assign_variable(name.to_string(), value, is_local);
        Ok(())
    }

    fn evaluate_expr(&mut self, expr: &Expr) -> Result<LuaData, LuaError>{
        let stmt = expr.stmts.get(0).unwrap();
        
        match stmt.stmt_type{
            StmtType::BinOp(ref operator, ref left, ref right) => Ok(self.evaluate_bin_op(operator, left, right)?),
            StmtType::Value(ref tokens) => Ok(self.evaluate_value_token(tokens)?),
            ref x => Err(error(format!("Couldn't evaluate expression: {:?}", x))),
        }
    }

    fn evaluate_value_token(&mut self, tokens: &Vec<Token>) -> Result<LuaData, LuaError>{
        let first_token = tokens.get(0).unwrap();

        Ok(match first_token{
            Token::NumberLiteral(x) => LuaData::Number(x.clone()),
            Token::StringLiteral(x) => LuaData::Str(x.clone()),
            Token::Keyword(Keyword::True) => LuaData::Bool(true),
            Token::Keyword(Keyword::False) => LuaData::Bool(false),
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
                        if let Some(val) = self.get_variable(x.to_string()){
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

    fn evaluate_bin_op(&self, operator: &BinOp, left: &Token, right: &Token) -> Result<LuaData, LuaError>{   
        Ok(match operator{
            BinOp::Concat => self.evaluate_str_binop(left, right)?,
            _ => self.evaluate_num_binop(operator, left, right)?,
        })
    }

    fn evaluate_num_binop(&self, operator: &BinOp, left: &Token, right: &Token) -> Result<LuaData, LuaError>{
        let left_num = self.token_to_num(left)?;

        let right_num = self.token_to_num(right)?;

        Ok(match operator{
            BinOp::Plus => LuaData::Number(left_num + right_num),
            BinOp::Minus => LuaData::Number(left_num - right_num),
            BinOp::Multiply => LuaData::Number(left_num * right_num),
            BinOp::Divide => LuaData::Number(left_num / right_num),
            BinOp::LessThan => LuaData::Bool(left_num < right_num),
            BinOp::LessEqualThan => LuaData::Bool(left_num <= right_num),
            BinOp::GreaterThan => LuaData::Bool(left_num > right_num),
            BinOp::GreaterEqualThan => LuaData::Bool(left_num >= right_num),
            _ => return Err(error(format!("Unknown num operator: {:?}!", operator))),
        })
    }

    fn evaluate_str_binop(&self, left: &Token, right: &Token) -> Result<LuaData, LuaError>{
        let left_string = self.token_to_string(left)?;
        let right_string = self.token_to_string(right)?;

        Ok(LuaData::Str(format!("{}{}", left_string, right_string)))
    }

    fn token_to_string(&self, token: &Token) -> Result<String, LuaError>{
        match token{
            Token::StringLiteral(x) => Ok(x.to_string()),
            Token::NumberLiteral(x) => Ok(format!("{}", x)),
            Token::Identifier(x) => {
                let var = self.get_variable(x.to_string());

                if let Some(var) = var{
                    Ok(format!("{}", var).to_string())
                }else{
                    Ok("nil".to_string())
                }
            }
            _ => Err(error(format!("Couldn't convert token to string: {:?}", token))),
        }
    }

     fn token_to_num(&self, token: &Token) -> Result<f64, LuaError>{
        match token{
            Token::NumberLiteral(x) => Ok(*x),
            Token::Identifier(x) => {
                let var = self.get_variable(x.to_string());

                if let Some(var) = var{
                    match var{
                        LuaData::Number(x) => Ok(*x),
                        LuaData::Bool(true) => Ok(1.0),
                        LuaData::Bool(false) => Ok(0.0),
                        x => Err(error(format!("Couldn't convert type to number: {:?}", x))),
                    }
                }else{
                    Ok(0.0)
                }
            }
            _ => Err(error(format!("Couldn't convert token to string: {:?}", token))),
        }
    }

    fn run_function_call(&mut self, name: &Token, args: Vec<Expr>) -> Result<LuaData, LuaError>{        
        let name = match name{
            Token::Identifier(string) => string,
            _ => return Err(error(format!("Illegal Token: expected identifier but found {:?}", name))),
        };

        let arg_data = self.evaluate_args(args)?;
        let func = match self.funcs.get(name){
            Some(x) => x,
            None => return Err(error(format!("Unable to find function with name: {}", name))),   
        }.clone();

        self.stack.push(HashMap::new());
      
        let result = match func{
            Function::Rust(func) => func(arg_data, self)?,
            Function::Lua(mut func) => func.execute(arg_data, self)?,
        }.unwrap_or(LuaData::Nil);

        self.stack.pop();
        self.return_val = None;

        Ok(result)
    }

    fn evaluate_args(&mut self, exprs: Vec<Expr>) -> Result<Vec<LuaData>, LuaError>{
        let mut data = Vec::new();

        for expr in exprs{
            data.push(self.evaluate_expr(&expr)?);
        }

        Ok(data)
    }
}

fn error(message: String) -> LuaError{
    LuaError::create_runtime(&message)
}

pub fn run(stmts: &mut Vec<Stmt>) -> Result<Interpreter, LuaError>{
    let mut interpreter = Interpreter::new();

    interpreter.register_func("print".to_string(), Function::Rust(|args, _| -> Result<Option<LuaData>, LuaError>{
        for arg in args{
            print!("{}\t", arg);
        }

        println!();
        Ok(None)
    }));

    for mut stmt in stmts.iter_mut(){
        match interpreter.run_stmt(&mut stmt){
            Err(e) => return Err(e),
            _ => (),
        };
    }

    Ok(interpreter)
}