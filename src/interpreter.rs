
use std::collections::HashMap;
use super::{Token, Stmt, StmtType, Expr, BinOp, Keyword};
use super::data::*;

type LuaFunc = (fn(Vec<LuaData>, &Interpreter) -> ());

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
            StmtType::If(ref expr, ref mut stmts, ref mut else_block) => self.run_if_stmt(expr, stmts, else_block),
            StmtType::FunctionCall(ref name, ref args) => self.run_function_call(name, args.to_vec()),
            StmtType::Assignment(ref name, ref expr) => self.handle_assignment(name, expr),
            StmtType::BinOp(_, _, _) | StmtType::Value(_) => panic!("Illegal Root Stmt: {:?}", stmt),
            StmtType::EOF => (),
        }
    }

    fn run_if_stmt(&mut self, expr: &Expr, stmts: &mut Vec<Stmt>, else_block: &mut Option<Vec<Stmt>>){
        let should_run = match self.evaluate_expr(expr){
            LuaData::Bool(b) => b,
            x => panic!("Expected boolean but found {}", x),
        };

        if should_run{
            for stmt in stmts{
                self.run_stmt(stmt);
            }
        }else if let Some(else_block) = else_block{
            for stmt in else_block{
                self.run_stmt(stmt);
            }
        }
    }

    fn handle_assignment(&mut self, name: &Token, expr: &Expr){
         let name = match name{
            Token::Identifier(n) => n,
            _ => panic!("Illegal Token: expected identifier but found {:?}", name),
        };

        let value = self.evaluate_expr(expr);

        self.assign_variable(name.to_string(), value);
    }

    fn evaluate_expr(&self, expr: &Expr) -> LuaData{
        let stmt = expr.stmts.get(0).unwrap();
        
        match stmt.stmt_type{
            StmtType::BinOp(ref operator, ref left, ref right) => self.evaluate_bin_op(operator, left, right),
            StmtType::Value(ref token) => self.evaluate_value_token(token),
            ref x => panic!("Couldn't evaluate expression: {:?}", x),
        }
    }

    fn evaluate_value_token(&self, token: &Token) -> LuaData{
        match token{
            Token::NumberLiteral(x) => LuaData::Number(x.clone()),
            Token::StringLiteral(x) => LuaData::Str(x.clone()),
            Token::Keyword(Keyword::True) => LuaData::Bool(true),
            Token::Keyword(Keyword::False) => LuaData::Bool(false),
            Token::Identifier(x) => {
                if let Some(val) = self.get_variable(x.to_string()){
                    return val.clone();
                }

                panic!("Undefined token used in expr: {}", x);
            },
            _ => panic!("Illegal Token: {:?} isn't a value", token),
        }
    }

    fn evaluate_bin_op(&self, operator: &BinOp, left: &Token, right: &Token) -> LuaData{   
        match operator{
            BinOp::Concat => self.evaluate_str_binop(left, right),
            _ => self.evaluate_num_binop(operator, left, right),
        }
    }

    fn evaluate_num_binop(&self, operator: &BinOp, left: &Token, right: &Token) -> LuaData{
        let left_num = self.token_to_num(left);

        let right_num = self.token_to_num(right);

        match operator{
            BinOp::Plus => LuaData::Number(left_num + right_num),
            BinOp::Minus => LuaData::Number(left_num - right_num),
            BinOp::Multiply => LuaData::Number(left_num * right_num),
            BinOp::Divide => LuaData::Number(left_num / right_num),
            BinOp::LessThan => LuaData::Bool(left_num < right_num),
            BinOp::LessEqualThan => LuaData::Bool(left_num <= right_num),
            BinOp::GreaterThan => LuaData::Bool(left_num > right_num),
            BinOp::GreaterEqualThan => LuaData::Bool(left_num >= right_num),
            _ => panic!("Unknown num operator: {:?}!", operator),
        }
    }

    fn evaluate_str_binop(&self, left: &Token, right: &Token) -> LuaData{
        let left_string = self.token_to_string(left);
        let right_string = self.token_to_string(right);

        LuaData::Str(format!("{}{}", left_string, right_string))
    }

    fn token_to_string(&self, token: &Token) -> String{
        match token{
            Token::StringLiteral(x) => x.to_string(),
            Token::NumberLiteral(x) => format!("{}", x),
            Token::Identifier(x) => {
                let var = self.get_variable(x.to_string());

                if let Some(var) = var{
                    format!("{}", var).to_string()
                }else{
                    "nil".to_string()
                }
            }
            _ => panic!("Couldn't convert token to string: {:?}", token),
        }
    }

     fn token_to_num(&self, token: &Token) -> f64{
        match token{
            Token::NumberLiteral(x) => *x,
            Token::Identifier(x) => {
                let var = self.get_variable(x.to_string());

                if let Some(var) = var{
                    match var{
                        LuaData::Number(x) => *x,
                        LuaData::Bool(true) => 1.0,
                        LuaData::Bool(false) => 0.0,
                        x => panic!("Couldn't convert type to number: {:?}", x),
                    }
                }else{
                    0.0
                }
            }
            _ => panic!("Couldn't convert token to string: {:?}", token),
        }
    }

    fn run_function_call(&mut self, name: &Token, args: Vec<Expr>){        
        let name = match name{
            Token::Identifier(string) => string,
            _ => panic!("Illegal Token: expected identifier but found {:?}", name),
        };
        
        let args = self.evaluate_args(args);

        let func = self.funcs.get(name);

        if let Some(func) = func{
            func(args, self);
        }
    }

    fn evaluate_args(&self, exprs: Vec<Expr>) -> Vec<LuaData>{
        let mut data = Vec::new();

        for expr in exprs{
            data.push(self.evaluate_expr(&expr));
        }

        data
    }
}

pub fn run(stmts: &mut Vec<Stmt>){
    let mut interpreter = Interpreter::new();

    interpreter.register_func("print".to_string(), |args, _|{
        for arg in args{
            print!("{}\t", arg);
        }

        println!();
    });

    for mut stmt in stmts.iter_mut(){
        interpreter.run_stmt(&mut stmt);
    }
}