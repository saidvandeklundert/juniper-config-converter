use log::{debug, warn};
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

    // read a character from the source code:
    #[inline]
    fn read_char(&mut self) {
        if self.read_position >= self.source.len() {
            self.character = '0';
        } else {
            self.character = self.source[self.read_position];
        }

        self.position = self.read_position;
        self.read_position += 1;
        debug!("read_char character: {:#?}", self.character);
        if self.read_position < self.end {
            debug!(
                "read_char read_position: {:#?}",
                self.source[self.read_position]
            );
        }
    }

    // check what character is available at the next read_position
    fn peek(&self) -> char {
        return self.source[self.read_position];
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
            'a'..='z' | 'A'..='Z' => Token::Identifier(self.read_identifier()),
            '0'..='9' => Token::Identifier(self.read_identifier()),
            '{' => Token::LeftSquirly,
            '}' => Token::RightSquirly,
            ';' => Token::Semicolon,
            '[' => Token::LeftBracket,
            ']' => Token::RightBracket,
            '#' => Token::Pound,
            '\n' => Token::NewLine,
            ':' => Token::Identifier(self.read_identifier()),
            '"' => Token::Identifier(self.read_identifier()),
            '-' => Token::Identifier(self.read_identifier()),
            '^' | '<' | '*' | '.' | '\\' | '/' | '>' => Token::Identifier(self.read_identifier()),
            _ => {
                warn!("unkown token: {:#?}", self.character);
                Token::Unkown(String::from(self.character as char))
            }
        };

        self.read_char();
        debug!("next token:\n{token}");
        return token;
    }

    // move the position forward as long as the character being read is a whitespace:
    fn skip_whitespace(&mut self) {
        while self.character == ' ' {
            self.read_char();
        }
    }

    #[inline]
    fn read_identifier(&mut self) -> String {
        let position = self.position;

        // deal with idenfiers that are enclosed with a comma:
        if self.character == '"' {
            self.read_char(); // read the first comma
            while self.character != '"' {
                self.read_char();
            }
        }

        // read identifiers that are not enclosed in a comma:
        while !self.character.is_whitespace() {
            self.read_char();
            if self.peek() == '\n' {
                break;
            }
        }
        let mut word: String = String::from_iter(&self.source[position..self.position + 1]);
        word = remove_trailing_whitespace(&word);
        debug!("read_identifier word: {:#?}", word);
        return word;
    }
}

fn remove_trailing_whitespace(s: &str) -> String {
    s.trim().to_string()
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    #[test]
    fn instantiate_lexer() {
        let input = String::from("{ example }");
        let _ = Lexer::new(&input);
    }

    #[test]
    fn tokens() {
        let input = String::from(
            r##"word1234 {
        } } ;
        "some comment" ;
        { }
        # words [ ] 
        "##,
        );
        let expected = vec![
            Token::Identifier("word1234".to_string()),
            Token::LeftSquirly,
            Token::NewLine,
            Token::RightSquirly,
            Token::RightSquirly,
            Token::Semicolon,
            Token::NewLine,
            Token::Identifier(r#""some comment""#.to_string()),
            Token::Semicolon,
            Token::NewLine,
            Token::LeftSquirly,
            Token::RightSquirly,
            Token::NewLine,
            Token::Pound,
            Token::Identifier("words".to_string()),
            Token::LeftBracket,
            Token::RightBracket,
            Token::NewLine,
            Token::Eof,
        ];
        let mut lexer = Lexer::new(&input);
        let tokens = lexer.tokenize();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn tokenize_system_config() {
        let input = String::from(
            "system {
            host-name myrouter;
            services {
                ftp;
                ssh;
                telnet;
                netconf {
                    ssh;
                }
            }
        }
        ",
        );
        let expected = vec![
            Token::Identifier("system".to_string()),
            Token::LeftSquirly,
            Token::NewLine,
            Token::Identifier("host-name".to_string()),
            Token::Identifier("myrouter;".to_string()),
            Token::NewLine,
            Token::Identifier("services".to_string()),
            Token::LeftSquirly,
            Token::NewLine,
            Token::Identifier("ftp;".to_string()),
            Token::NewLine,
            Token::Identifier("ssh;".to_string()),
            Token::NewLine,
            Token::Identifier("telnet;".to_string()),
            Token::NewLine,
            Token::Identifier("netconf".to_string()),
            Token::LeftSquirly,
            Token::NewLine,
            Token::Identifier("ssh;".to_string()),
            Token::NewLine,
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
