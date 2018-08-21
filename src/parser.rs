
use std::collections::VecDeque;

use super::{Token, Stmt, StmtType};

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
            return Stmt {stmt_type: StmtType::EOF, tokens: Vec::new()};
        }

        let token = token.unwrap();

        match token{
            Token::Identifier(_) => self.handle_indentifier(token),
            Token::LeftParenthesis | Token::RightParenthesis | Token::StringLiteral(_) =>{ 
                panic!("Stmt's cannot start with {:?}", token)
            },
            Token::EOF => return Stmt {stmt_type : StmtType::EOF, tokens: vec![token]},
        }
    }

    fn handle_indentifier(&mut self, token: Token) -> Stmt{
        if self.match_next(Token::LeftParenthesis){
            let stmt_type = StmtType::FunctionCall;
            let tokens = self.create_token_group(token);

            return Stmt {stmt_type, tokens};
        } 

        panic!("Unknown token following identifier: {:?}", token);
    }

    fn create_token_group(&mut self, first: Token) -> Vec<Token>{
        let mut vec: Vec<Token> = vec![first];

        loop{
            let token = self.next_token();

            if let Some(token) = token{
                if token == Token::EOF{
                    break;
                }

                vec.push(token);
            }else{
                break;
            }
        }
        
        vec
    }

    fn match_next(&self, token: Token) -> bool{
        if let Some(next) = self.tokens.get(0){
            return *next == token 
        }

        false
    }

    fn next_token(&mut self) -> Option<Token>{
        self.tokens.pop_front()
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Stmt>{
    let parser = Parser::new(tokens);

    parser.parse()
}