
use std::collections::VecDeque;

use super::{Token, BinOp, Stmt, StmtType, Expr, error, Keyword};
use super::super::{ExprType, error::LuaError};

struct ExprParser{
    expr_type: ExprType,
    tokens: VecDeque<Token>,
    line: usize
}

impl ExprParser{

    fn new(tokens: Vec<Token>, line: usize) -> ExprParser{
        let mut tokens_deque: VecDeque<Token> = VecDeque::new();
        let mut expr_type = ExprType::SingleValue;

        for token in tokens{
            tokens_deque.push_back(token);
        }

        for token in &tokens_deque{

            // If the expression has a operator it have more than value 
            let token_type = match token{
                Token::Operator(BinOp::Concat) => Some(ExprType::Str),
                Token::Operator(BinOp::LessThan) | Token::Operator(BinOp::LessEqualThan) |
                Token::Operator(BinOp::GreaterThan) | Token::Operator(BinOp::GreaterEqualThan) => Some(ExprType::Bool),
                Token::Operator(_) => Some(ExprType::Number),
                _ => None,
            };

            if let Some(token_type) = token_type{
                expr_type = token_type;
                break;
            }
        }

        ExprParser {tokens: tokens_deque, expr_type: expr_type, line}
    }
    
    fn parse(mut self) -> Result<Vec<Stmt>, LuaError>{
        let mut stmts: Vec<Stmt> = Vec::new();

        loop{
            let stmt = self.scan_stmt()?;

            if stmt.stmt_type == StmtType::EOF{
                stmts.push(stmt);
                break;
            }

            stmts.push(stmt);
        }

        Ok(stmts)
    }

    fn scan_stmt(&mut self) -> Result<Stmt, LuaError>{
        if let Some(token) = self.peek(){
            if token == &Token::Newline || token == &Token::Semicolon{
                return error("Expressions cannot have newlines or semicolons!".to_string(), self.line);
            }
        }else{
            return Ok(Stmt {stmt_type: StmtType::EOF});
        }

        match self.expr_type{
            ExprType::Number | ExprType::Bool => self.scan_num_expr(),
            ExprType::Str => self.scan_string_expr(),
            ExprType::SingleValue => self.scan_value(),
        }
    }

    fn scan_value(&mut self) -> Result<Stmt, LuaError>{
        let token = self.next_token().unwrap();
        let mut tokens = vec![token];

        if is_literal_value(&tokens[0]){
            return Ok(Stmt{stmt_type: StmtType::Value(tokens)});
        }

        loop{
            let next_token = self.next_token();

            if let Some(token) = next_token{
                 if token == Token::EOF || token == Token::Newline{
                    tokens.push(token);
                    break;
                }

                tokens.push(token);
            }else{
                break;
            }
        }

        Ok(Stmt{stmt_type: StmtType::Value(tokens)})
    }

    fn scan_num_expr(&mut self) -> Result<Stmt, LuaError>{
        let (left, operator_token) = self.scan_until(|x| is_operator(x))?;

        let operator = match operator_token{
            Token::Operator(operator) => operator,
            x => return error(format!("Expected binary operator but found {:?}; {:?}", x, left), self.line),
        };

        let right = self.scan_until(|x| {
            match x{
                Token::Semicolon | Token::Newline | Token::RightParenthesis => false,
                _ => false,
            }
        })?.0;

        Ok(Stmt{stmt_type: StmtType::BinOp(operator, parse(left, self.line)?, parse(right, self.line)?)})
    }

    fn scan_string_expr(&mut self) -> Result<Stmt, LuaError>{
        let (left, operator_token) = self.scan_until(|x| is_operator(x))?;

        let operator = match operator_token{
            Token::Operator(BinOp::Concat) => BinOp::Concat,
            x => return error(format!("Expected string concat operator but found {:?}", x), self.line),
        };

        let right = self.scan_until(|x| {
            match x{
                Token::Semicolon | Token::Newline | Token::RightParenthesis => false,
                _ => false,
            }
        })?.0;

        Ok(Stmt{stmt_type: StmtType::BinOp(operator, parse(left, self.line)?, parse(right, self.line)?)})
    }

    fn scan_until<F>(&mut self, mut f: F) -> Result<(Vec<Token>, Token), LuaError> where F: FnMut(&Token) -> bool{
        let mut tokens = Vec::new();
        let mut level = 0;

        loop{
            let token = self.next_token().unwrap_or(Token::EOF);

            if token == Token::LeftParenthesis{
                level += 1;
                continue;
            }else if token == Token::RightParenthesis{
                level -= 1;
                continue;
            }

            if (f(&token) && level == 0)|| token == Token::EOF{
                break Ok((tokens, token));
            }

            tokens.push(token);
        }
    }

    fn peek(&self) -> Option<&Token>{
        self.tokens.get(0)
    }

    fn next_token(&mut self) -> Option<Token>{
        let token = self.tokens.pop_front();
        token
    }
}

fn is_operator(token: &Token) -> bool{
    match token{
        Token::Operator(_) => true,
        _ => false,
    }
}

fn is_literal_value(token: &Token) -> bool{
    match token{
        Token::NumberLiteral(_) | Token::StringLiteral(_) | 
        Token::Keyword(Keyword::False) | Token::Keyword(Keyword::True) => true,
        _ => false
    }
}

pub fn parse(tokens: Vec<Token>, line: usize) -> Result<Expr, LuaError>{
    let parser = ExprParser::new(tokens, line);
    let expr_type = parser.expr_type.clone();
    
    Ok(Expr{expr_type: expr_type, stmts: parser.parse()?})
}