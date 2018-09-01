
use super::{Token, BinOp};

struct Scanner{
    src: Vec<char>,
    curr: usize
}

impl Scanner{

    pub fn new(src: String) -> Scanner{
        Scanner{src : src.chars().collect(), curr : 0}
    }

    pub fn scan(mut self) -> Vec<Token>{
        let mut tokens = Vec::new();

        loop{
            let token = self.scan_token();

            if token == Token::EOF{
                tokens.push(token);
                break;
            }

            tokens.push(token);
        }

        println!();
        tokens
    }

    fn scan_token(&mut self) -> Token{
        let token = if let Some(c) = self.advance_character(){
            match c {
                '(' => Some(Token::LeftParenthesis),
                ')' => Some(Token::RightParenthesis),
                ',' => Some(Token::Comma),
                '"' => Some(Token::StringLiteral(String::new())),
                '\n' => Some(Token::Newline),
                '=' => Some(Token::Operator(BinOp::Equal)),
                '+' => Some(Token::Operator(BinOp::Plus)),
                '-' => Some(Token::Operator(BinOp::Minus)),
                '*' => Some(Token::Operator(BinOp::Multiply)),
                '/' => Some(Token::Operator(BinOp::Divide)),
                '.' => Some(Token::Operator(BinOp::Concat)),  
                ' ' | '\t' | '\r' => None,
                x if x.is_alphabetic() => Some(Token::Identifier(String::new())),
                n if n.is_numeric() => Some(Token::NumberLiteral(0)),
                x => {
                    panic!("Unknown Character: {}", x); 
                }
            }
        }else{
            Some(Token::EOF)
        };

        let token = if let Some(token) = token{
            match token{
                Token::StringLiteral(_) => self.scan_string(),
                Token::Identifier(_) => self.scan_identifier(),
                Token::NumberLiteral(_) => self.scan_number(),
                Token::Operator(BinOp::Concat) => self.check_elipse(),
                x => x
            }
        }else{
            self.scan_token()
        };

        token
    }

    fn check_elipse(&mut self) -> Token{
        let c = self.advance_character();

        if let Some(c) = c{
            if c != &'.'{
                panic!("Expected ellipse, found: {}", c);
            }
        }else{
            panic!("File cannot end with character '.'");
        }

        Token::Operator(BinOp::Concat)
    }

    fn scan_string(&mut self) -> Token{
        let mut char_vec: Vec<char> = Vec::new();

        loop{
            let character = self.advance_character();

            if character == None{
                return Token::EOF;
            }

            if character == Some(&'"'){
                break;
            }

            char_vec.push(*character.unwrap());
        }

        let string = char_vec.iter().collect();
        Token::StringLiteral(string)
    }

    fn scan_number(&mut self) -> Token{
        let mut char_vec: Vec<char> = Vec::new();
        self.curr -= 1;

        loop{
            let character = self.advance_character().unwrap_or_else(||{
                &' '
            });

            if !character.is_numeric(){
                break;
            }

            char_vec.push(*character);
        }

        let string: String = char_vec.iter().collect();

        match string.parse::<i32>(){
            Ok(n) => Token::NumberLiteral(n),
            Err(e) => panic!("Unable to parse number literal {}: {}", string, e),
        }
    }

    fn scan_identifier(&mut self) -> Token{
        let mut char_vec: Vec<char> = vec![*self.char_at(self.curr - 1).unwrap()];
        let stop_chars = vec![Some(&' '), Some(&'\n'), Some(&'\t'), Some(&'('), 
            Some(&')'), Some(&',')];

        loop{
            if stop_chars.contains(&self.peek()){
                break;
            }

            let character_option = self.advance_character();

            if let Some(c) = character_option{
                char_vec.push(*c);   
            }else{
                return Token::EOF;
            }
        }

        let string = char_vec.iter().collect();
        Token::Identifier(string)
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

        c
    }

}

pub fn scan(src: String) -> Vec<Token>{
    let scanner = Scanner::new(src);

    scanner.scan()
}