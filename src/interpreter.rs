
use std::collections::HashMap;
use super::{Token, Stmt, StmtType, Expr, BinOp, Keyword, parser};
use super::data::*;

type RustFunc = fn(Vec<LuaData>, &Interpreter) -> Option<LuaData>;

#[derive(Clone)]
struct LuaFunc{
    arg_defs: Vec<Token>,
    stmts: Vec<Stmt>,
}

impl LuaFunc{

    fn execute(&mut self, arg_data: Vec<LuaData>, interpreter: &mut Interpreter) -> Option<LuaData>{
        if self.arg_defs.len() != arg_data.len(){
            panic!("Incorrect number of arguments found! Expected {} but found {}", self.arg_defs.len(), arg_data.len());
        }

        for x in 0..self.arg_defs.len(){
            let name = match self.arg_defs.get(x).unwrap(){
                Token::Identifier(x) => x,
                x => panic!("Expected identifier but found {:?}", x),    
            }.to_string();

            interpreter.assign_variable(name, arg_data.get(x).unwrap().clone())
        }

        let mut return_value = None;

        for stmt in &self.stmts{
            match stmt.stmt_type{
                StmtType::Return(ref expr) => {
                    return_value = Some(interpreter.evaluate_expr(expr));
                    break;
                }
                _ => interpreter.run_stmt(&mut stmt.clone()),
            }
        }

        for x in 0..self.arg_defs.len(){
            let name = match self.arg_defs.get(x).unwrap(){
                Token::Identifier(x) => x,
                x => panic!("Expected identifier but found {:?}", x),    
            }.to_string();

            interpreter.assign_variable(name, arg_data.get(x).unwrap().clone())
        }

        return_value
    }

}

#[derive(Clone)]
enum Function{
    Lua(LuaFunc),
    Rust(RustFunc)
}

struct Interpreter{
    funcs: HashMap<String, Function>,
    variables: HashMap<String, LuaData>
}

impl Interpreter{

    pub fn new() -> Interpreter{
        Interpreter {funcs: HashMap::new(), variables: HashMap::new()}
    }

    pub fn register_func(&mut self, name: String, func: Function){
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
            StmtType::FunctionDef(ref name, ref args, ref block) => self.handle_func_def(name, args, block),
            StmtType::If(ref expr, ref mut stmts, ref mut else_block) => self.run_if_stmt(expr, stmts, else_block),
            StmtType::FunctionCall(ref name, ref args) => {self.run_function_call(name, args.to_vec()); ()},
            StmtType::Assignment(ref name, ref expr) => self.handle_assignment(name, expr),
            StmtType::BinOp(_, _, _) | StmtType::Value(_) => panic!("Illegal Root Stmt: {:?}", stmt),
            StmtType::Return(_) => unreachable!(),
            StmtType::EOF => (),
        }
    }

    fn handle_func_def(&mut self, name: &Token, args: &Vec<Token>, stmts: &Vec<Stmt>){
        let name = match name{
            Token::Identifier(x) => x,
            x => panic!("Expected identifer but found {:?}", x),
        };

        let func = LuaFunc{arg_defs: args.to_vec(), stmts: stmts.to_vec()};

        self.register_func(name.to_string(), Function::Lua(func));
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

    fn evaluate_expr(&mut self, expr: &Expr) -> LuaData{
        let stmt = expr.stmts.get(0).unwrap();
        
        match stmt.stmt_type{
            StmtType::BinOp(ref operator, ref left, ref right) => self.evaluate_bin_op(operator, left, right),
            StmtType::Value(ref tokens) => self.evaluate_value_token(tokens),
            ref x => panic!("Couldn't evaluate expression: {:?}", x),
        }
    }

    fn evaluate_value_token(&mut self, tokens: &Vec<Token>) -> LuaData{
        let first_token = tokens.get(0).unwrap();

        match first_token{
            Token::NumberLiteral(x) => LuaData::Number(x.clone()),
            Token::StringLiteral(x) => LuaData::Str(x.clone()),
            Token::Keyword(Keyword::True) => LuaData::Bool(true),
            Token::Keyword(Keyword::False) => LuaData::Bool(false),
            Token::Identifier(x) => {
                match tokens.get(1){
                    Some(Token::LeftParenthesis) => {
                        let mut stmts = parser::parse(tokens.to_vec());

                        match stmts.remove(0).stmt_type{
                            StmtType::FunctionCall(ref name, ref args) => self.run_function_call(name, args.to_vec()),
                            x => panic!("Expected to find function but found {:?}", x),
                        }
                    },
                    None => {
                        if let Some(val) = self.get_variable(x.to_string()){
                            val.clone()
                        }else{
                            panic!("Coudn't find variable with name {}", x.to_string())
                        }
                    },
                    x => panic!("Unexpected token: {:?}", x),
                }
            },
            _ => panic!("Illegal Token: {:?} isn't a value", first_token),
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

    fn run_function_call(&mut self, name: &Token, args: Vec<Expr>) -> LuaData{        
        let name = match name{
            Token::Identifier(string) => string,
            _ => panic!("Illegal Token: expected identifier but found {:?}", name),
        };

        let arg_data = self.evaluate_args(args);;
        let func = self.funcs.get(name).unwrap_or_else(|| {
            panic!("Unable to find function with name: {}", name)
        }).clone();
      
        match func{
            Function::Rust(func) => func(arg_data, self),
            Function::Lua(mut func) => func.execute(arg_data, self),
        }.unwrap_or_else(||{LuaData::Nil})
    }

    fn evaluate_args(&mut self, exprs: Vec<Expr>) -> Vec<LuaData>{
        let mut data = Vec::new();

        for expr in exprs{
            data.push(self.evaluate_expr(&expr));
        }

        data
    }
}

pub fn run(stmts: &mut Vec<Stmt>){
    let mut interpreter = Interpreter::new();

    interpreter.register_func("print".to_string(), Function::Rust(|args, _| -> Option<LuaData>{
        for arg in args{
            print!("{}\t", arg);
        }

        println!();
        None
    }));

    for mut stmt in stmts.iter_mut(){
        interpreter.run_stmt(&mut stmt);
    }
}