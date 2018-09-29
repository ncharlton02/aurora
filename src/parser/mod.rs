
pub mod scanner;
pub mod expr;

use std::collections::VecDeque;
use super::{Token, BinOp, Stmt, StmtType, Expr, Keyword};
use super::error::LuaError;

struct Parser{
    tokens: VecDeque<Token>,
    line: usize
}

impl Parser{

    fn new(tokens: Vec<Token>) -> Parser{
        let mut tokens_deque: VecDeque<Token> = VecDeque::new();

        for token in tokens{
            tokens_deque.push_back(token);
        }

        Parser {tokens: tokens_deque, line: 1}
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
        let token = self.next_token();

        if token == None{
            return Ok(Stmt {stmt_type: StmtType::EOF});
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
                error(format!("Stmt's cannot start with {:?}", token), self.line)
            },
            Token::Newline => {
                self.line += 1;
                self.scan_stmt()
            },
            Token::Semicolon => self.scan_stmt(),
            Token::EOF => return Ok(Stmt {stmt_type : StmtType::EOF}),
        }
    }

    fn handle_return_stmt(&mut self) -> Result<Stmt, LuaError>{
        let value_tokens = self.advance_to_mult(vec![Token::Newline, Token::Semicolon]);

        match expr::parse(value_tokens, self.line){
            Ok(expr) => Ok(Stmt{stmt_type: StmtType::Return(expr)}),
            Err(e) => Err(e)
        }
    }

    fn handle_if_stmt(&mut self) -> Result<Stmt, LuaError>{
        let expr_tokens = self.advance_to(Token::Keyword(Keyword::Then));
        let expr = expr::parse(expr_tokens, self.line)?;
        let (block_tokens, block_end) = self.advance_to_if_end();
        let block = parse(block_tokens)?;

        if block_end == Some(Keyword::Else) {
            let else_block_tokens = self.advance_to(Token::Keyword(Keyword::End));

            return Ok(Stmt {stmt_type : StmtType::If(expr, block, Some(parse(else_block_tokens)?))})
        }

        Ok(Stmt{stmt_type : StmtType::If(expr, block, None)})
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


    fn handle_func_dec(&mut self) -> Result<Stmt, LuaError>{
        let name = match self.next_token(){
            Some(x) => x,
            None => return error(format!("Expected to find function name but found None"), self.line),
        };

        //Remove left parenthesis
        match self.next_token(){
            Some(Token::LeftParenthesis) => (),
            x => return error(format!("Expected left parenthesis but found {:?}", x), self.line),
        }

        let mut args = self.advance_to(Token::RightParenthesis);
        args.retain(|t| t != &Token::Comma);

        let block_tokens = self.advance_to_function_end();
        let block = parse(block_tokens)?;

        //Remove 'End'
        self.next_token();

        Ok(Stmt{stmt_type : StmtType::FunctionDef(name, args, block)})
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

    fn handle_indentifier(&mut self, token: Token) -> Result<Stmt, LuaError>{
        let following_token = self.next_token();

        if let Some(following_token) = following_token{
            match following_token{
                Token::LeftParenthesis =>{
                    let args = self.advance_to_args_end();
                    let stmt_type = StmtType::FunctionCall(token, self.parse_args(args)?);

                    Ok(Stmt {stmt_type})
                },
                Token::Operator(BinOp::Equal) =>{
                  Ok(self.scan_assignment(token, false)?)
                },
                _ => error(format!("Unknown token following identifier: {:?}", token), self.line),
            }
        }else{
            error(format!("Files cannot end with identifiers!"), self.line)
        }
    }

    fn advance_to_args_end(&mut self) -> Vec<Token>{
        let mut tokens = Vec::new();
        let mut level = 0;

        loop{
            let token = self.next_token();

            if let Some(token) = token{
                if token == Token::LeftParenthesis{
                    level += 1;
                }else if token == Token::RightParenthesis{
                    if level == 0{
                        break;
                    }
                    level -= 1;
                }

                tokens.push(token);
            }else{
                break;
            }
        }

        tokens
    }

    fn handle_local(&mut self) -> Result<Stmt, LuaError>{
        let name = match self.next_token(){
            Some(x) => x,
            None => return error(format!("Expected token following keyword local, but found None!"), self.line)
        };

        let equal_token = match self.next_token(){
            Some(x) => x,
            None => return error(format!("Expected token '=' but found None!"), self.line)
        };

        if equal_token != Token::Operator(BinOp::Equal){
            return error(format!("Expected token '=' but found, {:?}", equal_token), self.line);
        }

        self.scan_assignment(name, true)
    }

    fn scan_assignment(&mut self, name: Token, is_local: bool) -> Result<Stmt, LuaError>{
        let expr = expr::parse(self.advance_to_mult(vec![Token::Newline, Token::Semicolon]), self.line)?;
        let stmt_type = StmtType::Assignment(name, expr, is_local);

        Ok(Stmt {stmt_type})
    }

    fn parse_args(&self, args: Vec<Token>) -> Result<Vec<Expr>, LuaError>{
        let mut exprs= Vec::new();
        let mut tokens = Vec::new();

        for token in args{
            if token == Token::Comma{
                let expr = expr::parse(tokens.clone(), self.line)?;
                exprs.push(expr);
                tokens.clear();
            }else{
                tokens.push(token);
            }
        }

        //Parse the last argument
        if tokens.len() > 0{
            let expr = expr::parse(tokens, self.line)?;
            exprs.push(expr);
        }

        Ok(exprs)
    }

    fn advance_to(&mut self, stop: Token) -> Vec<Token>{
        self.advance_to_mult(vec![stop])
    }

    fn advance_to_mult(&mut self, stop: Vec<Token>)-> Vec<Token>{
        let mut tokens = Vec::new();

        loop{
            let token = self.next_token();

            if let Some(token) = token{
                if stop.contains(&token) || token == Token::EOF{
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

fn error(message: String, line: usize) -> Result<Stmt, LuaError>{
    Err(LuaError::create_parse(&message, Some(format!("Line {}", line))))
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, LuaError>{
    let parser = Parser::new(tokens);

    parser.parse()
}