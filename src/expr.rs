
use std::collections::VecDeque;

use super::{Token, BinOp, Stmt, StmtType, Expr, ExprType};

struct ExprParser{
    expr_type: ExprType,
    tokens: VecDeque<Token>,
}

impl ExprParser{

    fn new(tokens: Vec<Token>) -> ExprParser{
        let mut tokens_deque: VecDeque<Token> = VecDeque::new();
        let mut expr_type = ExprType::SingleValue;

        for token in tokens{
            tokens_deque.push_back(token);
        }

        for token in &tokens_deque{

            // If the expression has a operator it have more than value 
            let token_type = match token{
                Token::Operator(BinOp::Concat) => Some(ExprType::Str),
                Token::Operator(_) => Some(ExprType::Number),
                _ => None,
            };

            if let Some(token_type) = token_type{
                expr_type = token_type;
                break;
            }
        }

        ExprParser {tokens: tokens_deque, expr_type: expr_type}
    }
    
    fn parse(mut self) -> Vec<Stmt>{
        let mut stmts: Vec<Stmt> = Vec::new();

        loop{
            let stmt = self.scan_stmt();

            if stmt.stmt_type == StmtType::EOF{
                stmts.push(stmt);
                break;
            }

            stmts.push(stmt);
        }

        stmts
    }

    fn scan_stmt(&mut self) -> Stmt{
        let token = self.next_token();

        if token == None{
            return Stmt {stmt_type: StmtType::EOF};
        }

        let token = token.unwrap();

        if self.expr_type == ExprType::SingleValue{
            return Stmt{stmt_type: StmtType::Value(token)};
        }

        match token{
            Token::NumberLiteral(val) => self.scan_num_literal(token),
            _ => panic!("Unexpected token type: {:?}", token)
        }
    }

    fn scan_num_literal(&mut self, left: Token) -> Stmt{
        let operator = match self.next_token(){
            Some(Token::Operator(operator)) => operator,
            x => panic!("Expected binary operator but found {:?}", x),
        };

        let right = match self.next_token(){
            Some(x) => x,
            x => panic!("Expected number literal but found: {:?}", x),
        };

        Stmt{stmt_type: StmtType::BinOp(operator, left, right)}
    }

    fn next_token(&mut self) -> Option<Token>{
        self.tokens.pop_front()
    }
}

pub fn parse(tokens: Vec<Token>) -> Expr{
    let parser = ExprParser::new(tokens);

    Expr{expr_type: parser.expr_type.clone(), stmts: parser.parse(), }
}