use log::debug;
use std::fmt::Display;

#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    Eof,
    LeftSquirly,
    RightSquirly,
    Semicolon,
    Unkown(String),
    Start,
    LeftBracket,
    RightBracket,
    Pound,
    NewLine,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Token::Unkown(x) => write!(f, "Unkown({:#?})", x),
            Token::Identifier(x) => write!(f, "Identifier(\"{}\")", x),
            Token::Eof => write!(f, "Eof"),
            Token::LeftSquirly => write!(f, "LeftSquirly"),
            Token::RightSquirly => write!(f, "RightSquirly"),
            Token::Semicolon => write!(f, "SemiColon"),
            Token::Start => write!(f, "Start"),
            Token::LeftBracket => write!(f, "LeftBracket"),
            Token::RightBracket => write!(f, "RightBracket"),
            Token::Pound => write!(f, "Pound"),
            Token::NewLine => write!(f, "NewLine"),
        };
    }
}

pub struct Lexer {
    source: Vec<char>,
    position: usize,
    read_position: usize,
    character: char,
    end: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Lexer {
        let characters: Vec<char> = source.chars().collect();
        let end = characters.len();
        let mut lex = Lexer {
            source: characters,
            position: 0,
            read_position: 0,
            character: '0',
            end: end,
        };

        lex.read_char();
        return lex;
    }
    fn read_char(&mut self) {
        if self.read_position >= self.source.len() {
            self.character = '0';
        } else {
            self.character = self.source[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        {
            // fill tokens:
            let mut n = 0;
            let mut token = self.next_token();
            tokens.push(token.clone());
            debug!("{token}");
            while token != Token::Eof {
                n = n + 1;
                token = self.next_token();
                tokens.push(token.clone());
            }
        }
        tokens
    }
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position == self.end {
            return Token::Eof;
        }
        let token = match self.character {
            '{' => Token::LeftSquirly,
            '}' => Token::RightSquirly,
            ';' => Token::Semicolon,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '#' => Token::Pound,
            '\n' => Token::NewLine,
            'a'..='z'
            | 'A'..='Z'
            | '"'
            | '-'
            | '0'..='9'
            | '^'
            | '<'
            | '*'
            | '.'
            | '\\'
            | '/'
            | '>' => {
                let statement = self.read_identifier();
                match statement.as_str() {
                    "placeholder" => Token::Identifier(String::from("placeholder")),
                    _ => Token::Identifier(statement),
                }
            }
            _ => Token::Unkown(String::from(self.character as char)),
        };

        self.read_char();
        debug!("next token:\n{token}");
        return token;
    }
    fn skip_whitespace(&mut self) {
        while self.character == ' ' {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let position = self.position;

        // deal with idenfiers that are enclosed with a comma:
        if self.character == '"' {
            self.read_char(); // read the first comma
            while self.character != '"' {
                self.read_char();
            }
        }

        while !self.character.is_whitespace() && self.character != '\n' {
            self.read_char();
        }
        let word: String = String::from_iter(&self.source[position..self.position]);
        return word;
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};
    use crate::utils::open_config_file;

    #[test]
    fn instantiate_lexer() {
        let input = String::from("{ example }");
        let _ = Lexer::new(&input);
    }

    #[test]
    fn tokenize_file_test() {
        let input = open_config_file("config_3.txt");
        let expected = vec![
            Token::Identifier("system".to_string()),
            Token::LeftSquirly,
            Token::NewLine,
            Token::Identifier("host-name".to_string()),
            Token::Identifier("myrouter;".to_string()),
            Token::Identifier("services".to_string()),
            Token::LeftSquirly,
            Token::NewLine,
            Token::Identifier("ftp;".to_string()),
            Token::Identifier("ssh;".to_string()),
            Token::Identifier("telnet;".to_string()),
            Token::Identifier("netconf".to_string()),
            Token::LeftSquirly,
            Token::NewLine,
            Token::Identifier("ssh;".to_string()),
            Token::RightSquirly,
            Token::NewLine,
            Token::RightSquirly,
            Token::NewLine,
            Token::RightSquirly,
            Token::NewLine,
            Token::Eof,
        ];
        let mut lexer = Lexer::new(&input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens, expected);
    }
}
