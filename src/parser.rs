
use std::collections::VecDeque;

use super::{Token, BinOp, Stmt, StmtType, Expr, expr, Keyword};

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
            Token::Keyword(Keyword::If) => self.handle_if_stmt(),
            Token::LeftParenthesis | Token::RightParenthesis | Token::StringLiteral(_) | 
            Token::Operator(_) | Token::NumberLiteral(_) | Token::Comma | Token::Keyword(_) =>{ 
                panic!("Stmt's cannot start with {:?}", token)
            },
            Token::Newline => self.scan_stmt(),
            Token::EOF => return Stmt {stmt_type : StmtType::EOF},
        }
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