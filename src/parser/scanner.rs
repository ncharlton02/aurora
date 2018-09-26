
use super::{Token, BinOp, Keyword};
use super::super::error::{LuaError};

struct Scanner{
    src: Vec<char>,
    curr: usize,
    line_num: usize,
}

impl Scanner{

    pub fn new(src: String) -> Scanner{
        Scanner{src : src.chars().collect(), curr : 0, line_num: 0}
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

        let token = if let Some(c) = self.advance_character(){
            match c {
                '(' => Some(Token::LeftParenthesis),
                ')' => Some(Token::RightParenthesis),
                ',' => Some(Token::Comma),
                '"' => Some(Token::StringLiteral(String::new())),
                '\n' | ';' => Some(Token::Newline),
                '=' => Some(Token::Operator(BinOp::Equal)),
                '+' => Some(Token::Operator(BinOp::Plus)),
                '-' => Some(Token::Operator(BinOp::Minus)),
                '*' => Some(Token::Operator(BinOp::Multiply)),
                '/' => Some(Token::Operator(BinOp::Divide)),
                '.' => Some(Token::Operator(BinOp::Concat)),  
                '<' => Some(Token::Operator(BinOp::LessThan)),
                '>' => Some(Token::Operator(BinOp::GreaterThan)),
                ' ' | '\t' | '\r' => None,
                x if x.is_alphabetic() => Some(Token::Identifier(String::new())),
                n if n.is_numeric() => Some(Token::NumberLiteral(0.0)),
                x => {
                    return error(format!("Unknown Character: {}", x), line);
                }
            }
        }else{
            Some(Token::EOF)
        };

        if let Some(token) = token{
            match token{
                Token::StringLiteral(_) => self.scan_string(),
                Token::Identifier(_) => self.scan_identifier(),
                Token::NumberLiteral(_) => self.scan_number(),
                Token::Operator(BinOp::Concat) => self.check_elipse(),
                Token::Operator(BinOp::GreaterThan) => self.scan_greater_than(),
                Token::Operator(BinOp::LessThan) => self.scan_less_than(),
                x => Ok(x)
            }
        }else{
            self.scan_token()
        }
    }

    fn scan_greater_than(&mut self) -> Result<Token, LuaError>{
        if self.advance_character().unwrap_or(&' ') == &'='{
            return Ok(Token::Operator(BinOp::GreaterEqualThan));
        }

        self.curr -= 1;

        Ok(Token::Operator(BinOp::GreaterThan))
    }

    fn scan_less_than(&mut self) -> Result<Token, LuaError>{
        if self.advance_character().unwrap_or(&' ') == &'='{
            return Ok(Token::Operator(BinOp::LessEqualThan));
        }

        self.curr -= 1;

        Ok(Token::Operator(BinOp::LessThan))
    }


    fn check_elipse(&mut self) -> Result<Token, LuaError>{
        let line = self.line_num.clone();
        let c = self.advance_character();

        if let Some(c) = c{
            if c != &'.'{
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

            if character == Some(&'"'){
                break;
            }

            char_vec.push(*character.unwrap());
        }

        Ok(Token::StringLiteral(char_vec.iter().collect()))
    }

    fn scan_number(&mut self) -> Result<Token, LuaError>{
        let mut had_decimal = false;
        let mut char_vec: Vec<char> = Vec::new();
        self.curr -= 1;

        loop{
            if !self.peek().unwrap_or(&' ').is_numeric() {
                if self.peek().unwrap_or(&' ') != &'.' || had_decimal{
                    break;
                }

                had_decimal = true;
            }

            char_vec.push(*self.advance_character().unwrap());
        }

        let string: String = char_vec.iter().collect();

        match string.parse::<f64>(){
            Ok(n) => Ok(Token::NumberLiteral(n)),
            Err(e) => error(format!("Unable to parse number literal {}: {}", string, e), self.line_num),
        }
    }

    fn scan_identifier(&mut self) -> Result<Token, LuaError>{
        let mut char_vec: Vec<char> = vec![*self.char_at(self.curr - 1).unwrap()];
        let stop_chars = vec![Some(&' '), Some(&'\n'), Some(&'\t'), Some(&'('), 
            Some(&')'), Some(&','), Some(&'\r'), Some(&';')];

        loop{
            if stop_chars.contains(&self.peek()){
                break;
            }

            let character_option = self.advance_character();

            if let Some(c) = character_option{
                char_vec.push(*c);   
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

    fn char_at(&self, i: usize) -> Option<&char>{
        self.src.get(i)
    }

    fn peek(&self) -> Option<&char>{
        self.char_at(self.curr)
    }

    fn advance_character(&mut self) -> Option<&char>{
        let c = self.src.get(self.curr);
        self.curr += 1;

        if c == Some(&'\n'){
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