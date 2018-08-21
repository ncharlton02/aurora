
use super::Token;

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
                '"' => Some(Token::StringLiteral(String::new())),
                '\n' => Some(Token::Newline),
                ' ' | '\t' | '\r' => None,
                x if x.is_alphabetic() => Some(Token::Identifier(String::new())),
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
                x => x
            }
        }else{
            self.scan_token()
        };

        token
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

    fn scan_identifier(&mut self) -> Token{
        let mut char_vec: Vec<char> = vec![*self.char_at(self.curr - 1).unwrap()];
        let stop_chars = vec![Some(&' '), Some(&'\n'), Some(&'\t'), Some(&'(')];

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