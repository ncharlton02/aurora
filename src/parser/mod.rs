
pub mod scanner;
pub mod expr;

use std::collections::VecDeque;
use super::{Token, BinOp, Stmt, StmtType, Expr, Keyword};

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
            Token::Keyword(Keyword::Local) => self.handle_local(),
            Token::Keyword(Keyword::If) => self.handle_if_stmt(),
            Token::Keyword(Keyword::Function) => self.handle_func_dec(),
            Token::Keyword(Keyword::Return) => self.handle_return_stmt(),
            Token::LeftParenthesis | Token::RightParenthesis | Token::StringLiteral(_) | 
            Token::Operator(_) | Token::NumberLiteral(_) | Token::Comma | Token::Keyword(_) =>{ 
                panic!("Stmt's cannot start with {:?}", token)
            },
            Token::Newline => self.scan_stmt(),
            Token::EOF => return Stmt {stmt_type : StmtType::EOF},
        }
    }

    fn handle_return_stmt(&mut self) -> Stmt{
        let value_tokens = self.advance_to(Token::Newline);

        let expr = expr::parse(value_tokens);

        Stmt{stmt_type: StmtType::Return(expr)}
    }

    fn handle_if_stmt(&mut self) -> Stmt{
        let expr_tokens = self.advance_to(Token::Keyword(Keyword::Then));
        let expr = expr::parse(expr_tokens);
        let (block_tokens, block_end) = self.advance_to_if_end();
        let block = parse(block_tokens);

        if block_end == Some(Keyword::Else) {
            let else_block_tokens = self.advance_to(Token::Keyword(Keyword::End));

            return Stmt {stmt_type : StmtType::If(expr, block, Some(parse(else_block_tokens)))}
        }

        Stmt{stmt_type : StmtType::If(expr, block, None)}
    }

    fn advance_to_if_end(&mut self) -> (Vec<Token>, Option<Keyword>){
        let mut tokens = Vec::new();
        let stop_keywords: Vec<Keyword> = vec![Keyword::End, Keyword::Else];

        loop{
            let token = self.next_token();

            if let Some(token) = token{
                match token{
                    Token::Keyword(ref k) if stop_keywords.contains(k) => return (tokens, Some(k.clone())), 
                    _ => (), 
                }

                tokens.push(token);
            }else{
                break;
            }
        }

        (tokens, None)
    }


    fn handle_func_dec(&mut self) -> Stmt{
        let name = self.next_token().unwrap_or_else(||{
            panic!("Expected token following keyword function!");
        });

        //Remove left parenthesis
        match self.next_token(){
            Some(Token::LeftParenthesis) => (),
            x => panic!("Expected left parenthesis but found {:?}", x),
        }

        let mut args = self.advance_to(Token::RightParenthesis);
        args.retain(|t| t != &Token::Comma);

        let block_tokens = self.advance_to_function_end();
        let block = parse(block_tokens);

        //Remove 'End'
        self.next_token();

        Stmt{stmt_type : StmtType::FunctionDef(name, args, block)}
    }

    fn advance_to_function_end(&mut self) -> Vec<Token>{
        let mut tokens = Vec::new();
        let mut level = 0;

        loop{
            let token = self.next_token();

            if let Some(token) = token{
                match token{
                    Token::Keyword(Keyword::If)  => level+=1,
                    Token::Keyword(Keyword::End) => {
                        if level == 0{
                            break;
                        }

                        level -= 1;
                    } 
                    _ => (), 
                }

                tokens.push(token);
            }else{
                break;
            }
        }

        tokens
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
                   self.scan_assignment(token, false)
                },
                _ => panic!("Unknown token following identifier: {:?}", token),
            }
        }else{
            panic!("Files cannot end with identifiers!");
        }
    }

    fn handle_local(&mut self) -> Stmt{
        let name = self.next_token().unwrap_or_else(||{panic!("Expected token following keyword local, but found None!")});

        let equal_token = self.next_token().unwrap_or_else(||{panic!("Expected token '=' but found None!")});

        if equal_token != Token::Operator(BinOp::Equal){
            panic!("Expected token '=' but found, {:?}", equal_token);
        }

        self.scan_assignment(name, true)
    }

    fn scan_assignment(&mut self, name: Token, is_local: bool) -> Stmt{
        let expr = expr::parse(self.advance_to(Token::Newline));
        let stmt_type = StmtType::Assignment(name, expr, is_local);

        return Stmt {stmt_type};
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