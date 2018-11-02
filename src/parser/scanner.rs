
use super::{Token, BinOp, Keyword};
use super::super::error::{LuaError};

struct Scanner{
    src: Vec<char>,
    curr: usize,
    line_num: usize,
}

impl Scanner{

    pub fn new(src: String) -> Scanner{
        Scanner{src : src.chars().collect(), curr : 0, line_num: 1}
    }

    pub fn scan(mut self) -> Result<Vec<Token>, Vec<LuaError>>{
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        loop{
            let token = match self.scan_token(){
                Ok(x)=> x,
                Err(e) => {
                    errors.push(e); 
                    continue;
                },
            };

            if token == Token::EOF{
                tokens.push(token);
                break;
            }

            tokens.push(token);
        }

        if errors.len() > 0{
            return Err(errors);
        }

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Token, LuaError>{
        let line = self.line_num.clone();
        let next_char = self.advance_character().clone();

        if let Some(c) = next_char{
            match c {
                '(' => Ok(Token::LeftParenthesis),
                ')' => Ok(Token::RightParenthesis),
                ',' => Ok(Token::Comma),
                '"' => self.scan_string(),
                '\n' => Ok(Token::Newline),
                ';' => Ok(Token::Semicolon),
                '=' => Ok(Token::Operator(BinOp::Equal)),
                '+' => Ok(Token::Operator(BinOp::Plus)),
                '-' => self.check_comment(),
                '*' => Ok(Token::Operator(BinOp::Multiply)),
                '/' => Ok(Token::Operator(BinOp::Divide)),
                '.' => self.check_elipse(),  
                '<' => self.scan_less_than(),
                '>' => self.scan_greater_than(),
                ' ' | '\t' | '\r' => self.scan_token(),
                x if x.is_alphabetic() => self.scan_identifier(),
                n if n.is_numeric() => self.scan_number(),
                x => {
                    return error(format!("Unknown Character: {}", x), line);
                }
            }
        }else{
            Ok(Token::EOF)
        }
    }

    fn check_comment(&mut self) -> Result<Token, LuaError>{
        if self.peek() == Some('-'){
            //Scan comment
            return self.scan_until_comment_end();
        }

        Ok(Token::Operator(BinOp::Minus))
    }

    fn scan_until_comment_end(&mut self) -> Result<Token, LuaError>{
        loop{
            let c = self.advance_character();

            if let Some(c) = c{
                if c == '\n'{
                    return Ok(Token::Newline);
                }
            }else{
                return Ok(Token::EOF);
            }
        }
    }

    fn scan_greater_than(&mut self) -> Result<Token, LuaError>{
        if self.advance_character().unwrap_or(' ') == '='{
            return Ok(Token::Operator(BinOp::GreaterEqualThan));
        }

        self.curr -= 1;

        Ok(Token::Operator(BinOp::GreaterThan))
    }

    fn scan_less_than(&mut self) -> Result<Token, LuaError>{
        if self.advance_character().unwrap_or(' ') == '='{
            return Ok(Token::Operator(BinOp::LessEqualThan));
        }

        self.curr -= 1;

        Ok(Token::Operator(BinOp::LessThan))
    }


    fn check_elipse(&mut self) -> Result<Token, LuaError>{
        let line = self.line_num.clone();
        let c = self.advance_character();

        if let Some(c) = c{
            if c != '.'{
                return error(format!("Expected ellipse, found: {}", c), line);
            }
        }else{
            return error(format!("File cannot end with character '.'"), line);
        }

        Ok(Token::Operator(BinOp::Concat))
    }

    fn scan_string(&mut self) -> Result<Token, LuaError>{
        let mut char_vec: Vec<char> = Vec::new();

        loop{
            let character = self.advance_character();

            if character == None{
                return Ok(Token::EOF);
            }

            if character == Some('"'){
                break;
            }

            char_vec.push(character.unwrap());
        }

        Ok(Token::StringLiteral(char_vec.iter().collect()))
    }

    fn scan_number(&mut self) -> Result<Token, LuaError>{
        let mut had_decimal = false;
        let mut char_vec: Vec<char> = Vec::new();
        self.curr -= 1;

        loop{
            if !self.peek().unwrap_or(' ').is_numeric() {
                if self.peek().unwrap_or(' ') != '.' || had_decimal{
                    break;
                }

                had_decimal = true;
            }

            char_vec.push(self.advance_character().unwrap());
        }

        let string: String = char_vec.iter().collect();

        match string.parse::<f64>(){
            Ok(n) => Ok(Token::NumberLiteral(n)),
            Err(e) => error(format!("Unable to parse number literal {}: {}", string, e), self.line_num),
        }
    }

    fn scan_identifier(&mut self) -> Result<Token, LuaError>{
        let mut char_vec: Vec<char> = vec![self.char_at(self.curr - 1).unwrap()];
        let stop_chars = vec![Some(' '), Some('\n'), Some('\t'), Some('('), 
            Some(')'), Some(','), Some('\r'), Some(';')];

        loop{
            if stop_chars.contains(&self.peek()){
                break;
            }

            let character_option = self.advance_character();

            if let Some(c) = character_option{
                char_vec.push(c);   
            }else{
                return Ok(Token::EOF);
            }
        }

        let string: String = char_vec.iter().collect();

        if Keyword::is_keyword(&string){
            return Ok(Token::Keyword(Keyword::from_string(&string)));
        }

        Ok(Token::Identifier(string))
    }

    fn char_at(&self, i: usize) -> Option<char>{
        if let Some(c) = self.src.get(i){
            Some(*c)
        }else{
            None
        }
    }

    fn peek(&self) -> Option<char>{
        self.char_at(self.curr)
    }

    fn advance_character(&mut self) -> Option<char>{
        let c = self.char_at(self.curr);
        self.curr += 1;

        if c == Some('\n'){
            self.line_num += 1;
        }

        c
    }

}

fn error(message: String, line: usize) -> Result<Token, LuaError>{
        Err(LuaError::create_lexical(&message, Some(format!("[Line {}]", line))))
    }

pub fn scan(src: String) -> Result<Vec<Token>, Vec<LuaError>>{
    let scanner = Scanner::new(src);

    scanner.scan()
}