
use std::collections::VecDeque;

use super::{Token, BinOp, Stmt, StmtType, Expr, expr};

struct Parser{
    tokens: VecDeque<Token>,
}

impl Parser{

    fn new(tokens: Vec<Token>) -> Parser{
        let mut tokens_deque: VecDeque<Token> = VecDeque::new();

        for token in tokens{
            tokens_deque.push_back(token);
        }

        Parser {tokens: tokens_deque}
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

        match token{
            Token::Identifier(_) => self.handle_indentifier(token),
            Token::LeftParenthesis | Token::RightParenthesis | Token::StringLiteral(_) | 
            Token::Operator(_) | Token::NumberLiteral(_) | Token::Comma | Token::Keyword(_) =>{ 
                panic!("Stmt's cannot start with {:?}", token)
            },
            Token::Newline => self.scan_stmt(),
            Token::EOF => return Stmt {stmt_type : StmtType::EOF},
        }
    }

    fn handle_indentifier(&mut self, token: Token) -> Stmt{
        let following_token = self.next_token();

        if let Some(following_token) = following_token{
            match following_token{
                Token::LeftParenthesis =>{
                    let args = self.advance_to(Token::RightParenthesis);
                    let stmt_type = StmtType::FunctionCall(token, self.parse_args(args));

                    return Stmt {stmt_type};
                },
                Token::Operator(BinOp::Equal) =>{
                    let expr = expr::parse(self.advance_to(Token::Newline));
                    let stmt_type = StmtType::Assignment(token, expr);

                    return Stmt {stmt_type};
                },
                _ => panic!("Unknown token following identifier: {:?}", token),
            }
        }else{
            panic!("Files cannot end with identifiers!");
        }
    }

    fn parse_args(&self, args: Vec<Token>) -> Vec<Expr>{
        let mut exprs = Vec::new();
        let mut tokens = Vec::new();

        for token in args{
            if token == Token::Comma{
                let expr = expr::parse(tokens.clone());
                exprs.push(expr);
                tokens.clear();
            }else{
                tokens.push(token);
            }
        }

        //Parse the last argument
        let expr = expr::parse(tokens);
        exprs.push(expr);

        exprs
    }

    fn advance_to(&mut self, stop: Token) -> Vec<Token>{
        let mut tokens = Vec::new();

        loop{
            let token = self.next_token();

            if let Some(token) = token{
                if token == stop || token == Token::EOF{
                    break;
                }

                tokens.push(token);
            }else{
                break;
            }
        }

        tokens
    }

    fn next_token(&mut self) -> Option<Token>{
        self.tokens.pop_front()
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt>{
    let parser = Parser::new(tokens);

    parser.parse()
}