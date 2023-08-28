use crate::lexer::{Lexer, Token};
use crate::utils::build_string;
use log::{debug, info};

pub struct ConfigWriter {
    tokens: Vec<Token>,
    token: Token,
    position: usize,
    read_position: usize,
    output: Vec<String>,
}

impl ConfigWriter {
    pub fn new(config: String) -> ConfigWriter {
        let mut lexer = Lexer::new(config);
        let mut tokens = Vec::new();

        {
            // fill tokens:
            let mut n = 0;
            let mut token = lexer.next_token();
            tokens.push(token.clone());
            debug!("{token}");
            while token != Token::Eof {
                n = n + 1;
                token = lexer.next_token();
                info!("{token}");
                tokens.push(token.clone());
            }
        }
        debug!("Tokenization done!!\n\n\n");
        info!("{:#?}", tokens);
        let output: Vec<String> = Vec::new();
        return ConfigWriter {
            tokens: tokens,
            token: Token::Start,
            position: 0,
            read_position: 0,
            output: output,
        };
    }

    pub fn write_configs(&mut self) -> String {
        self.read_token();
        let mut stanza_stack_record: Vec<Vec<String>> = Vec::new();
        let mut stanza_stack: Vec<String> = Vec::new();
        let mut stanza_pointer: usize = 0;
        let mut config_line_stack: Vec<String> = Vec::new();
        let mut inside_bracket_array: bool = false;
        let mut next_inactive: bool = false;
        let mut next_protect: bool = false;

        while self.token != Token::Eof {
            debug!("write_configs: {} {}", self.read_position, self.token);
            match &self.token {
                Token::LeftSquirly => {
                    debug!("LeftSquirly");
                    if next_inactive {
                        let addition = build_string(&stanza_stack_record, &config_line_stack)
                            .replace("set", "deactivate");
                        self.output.push(addition);
                        next_inactive = false;
                    } else if next_protect {
                        let addition = build_string(&stanza_stack_record, &config_line_stack)
                            .replace("set", "protect");
                        self.output.push(addition);
                        next_protect = false;
                    }
                    stanza_pointer += 1;
                    stanza_stack_record.push(stanza_stack.clone());
                    stanza_stack.clear();
                    config_line_stack.clear();
                }
                Token::LeftBracket => {
                    debug!("LeftBracket");
                    inside_bracket_array = true;
                }
                Token::RightBracket => {
                    debug!("RightBracket");
                    inside_bracket_array = false;
                    config_line_stack.clear();
                }
                Token::Semicolon => {
                    debug!("Semicolon");
                    if self.tokens[self.read_position - 2] == Token::RightBracket {
                        // this means we have a lost ';' after iterating the values
                        // in brackets and we need to wipe the stanza_stack
                        stanza_stack.clear();
                    }
                    config_line_stack.clear();
                }
                Token::RightSquirly => {
                    debug!("RightSquirly");
                    stanza_pointer -= 1;
                    stanza_stack.clear();
                    stanza_stack_record.pop();
                }
                Token::Pound => {
                    debug!("Pound");
                    self.move_past_comment();
                }
                Token::Identifier(string) => {
                    debug!("Identifier");
                    let statement = string;
                    if statement.ends_with(';') {
                        config_line_stack.push(statement.clone().to_owned());
                        let addition = build_string(&stanza_stack_record, &config_line_stack);

                        if next_inactive {
                            let deactivate = addition.clone().replace("set", "deactivate");
                            next_inactive = false;
                            self.output.push(addition);
                            self.output.push(deactivate);
                        } else if next_protect {
                            let protect = addition.clone().replace("set", "protect");
                            next_protect = false;
                            self.output.push(addition);
                            self.output.push(protect);
                        } else {
                            self.output.push(addition);
                        }
                        config_line_stack.clear();
                        stanza_stack.clear();
                    } else if inside_bracket_array {
                        // bracket array statements are build in this condition
                        // the config_line_stack is reset when the right bracket is encountered
                        config_line_stack.push(statement.clone().to_owned());
                        let addition = build_string(&stanza_stack_record, &config_line_stack);
                        self.output.push(addition);
                        config_line_stack.pop();
                    } else if statement == "inactive:" {
                        next_inactive = true;
                    } else if statement == "protect:" {
                        next_protect = true;
                    } else {
                        debug!("non terminating statement {statement}");
                        stanza_stack.push(statement.clone().to_owned());
                        config_line_stack.push(statement.clone().to_owned());
                    }
                }
                _ => debug!("hit default case for {}", self.token),
            }

            self.read_token();
            debug!("stanza_stack {:#?}", stanza_stack);
            debug!("stanza_pointer {stanza_pointer}");
            debug!("stanza_stack_record {:#?}", stanza_stack_record);
            debug!("config_line_stack {:#?}", config_line_stack);
        }
        return self.output.join("\n");
    }

    fn read_token(&mut self) {
        if self.read_position >= self.tokens.len() {
            self.token = Token::Eof;
        } else {
            self.token = self.tokens[self.read_position].clone();
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    // Process tokens after a pound indicated comment.
    //
    // This means that tokens following a pound will be ignored untill
    // one of following tokens appear:
    // - newline
    // - RightSquirly ('}') Tokens
    // - an identifier that can be interpreted as a terminating statment
    fn move_past_comment(&mut self) {
        info!("move_past_comment");
        loop {
            let next_token = self.tokens[self.read_position].clone();
            if next_token == Token::NewLine || next_token == Token::RightSquirly {
                break;
            }

            match &next_token {
                Token::Identifier(string) => {
                    let statement = string;
                    if statement.ends_with(';') {
                        break;
                    }
                }
                _ => (),
            }

            self.token = self.tokens[self.read_position].clone();
            info!("skip {}", self.token);
            self.position = self.read_position;
            self.read_position += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::{ConfigWriter, Lexer, Token};
    use crate::utils::open_config_file;

    #[test]
    fn basic_identifiers_test() {
        let input = String::from("{ as-path test another-statement-123 } } }");

        let mut lexer = Lexer::new(input);

        let tokens: Vec<Token> = vec![
            Token::LeftSquirly,
            Token::Identifier("as-path".to_string()),
            Token::Identifier("test".to_string()),
            Token::Identifier("another-statement-123".to_string()),
            Token::RightSquirly,
            Token::RightSquirly,
            Token::RightSquirly,
            Token::Eof,
        ];

        for token in tokens {
            let next_token = lexer.next_token();
            println!("expected: {:?}, received {:?}", token, next_token);
            assert_eq!(token, next_token);
        }
    }

    #[test]
    fn basic_config_convert_1() {
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
        }",
        );

        let expected = String::from(
            "set system host-name myrouter
set system services ftp
set system services ssh
set system services telnet
set system services netconf ssh",
        );
        let mut config_writer = ConfigWriter::new(input.clone());
        let result = config_writer.write_configs();
        assert_eq!(result, expected);
    }

    #[test]
    fn basic_config_convert_2() {
        let input = String::from(
            "policy-options {
                policy-statement directs {
                    term Lo0 {
                        from {
                            protocol direct;
                            route-filter 192.168.100.0/24 orlonger;
                        }
                        then accept;
                    }
                }   
            }",
        );

        let expected = String::from(
            "set policy-options policy-statement directs term Lo0 from protocol direct
set policy-options policy-statement directs term Lo0 from route-filter 192.168.100.0/24 orlonger
set policy-options policy-statement directs term Lo0 then accept",
        );
        let mut config_writer = ConfigWriter::new(input.clone());
        let result = config_writer.write_configs();
        assert_eq!(result, expected);
    }

    #[test]
    fn config_convert_files() {
        let files: Vec<&str> = vec![
            "config_1",
            "config_2",
            "config_3",
            "config_4",
            "config_5",
            "config_6",
            "config_7",
            "config_8",
            "config_9",
            "config_10",
            "config_11",
            "config_12",
            "config_13",
            "config_14",
            "config_15",
            "config_16",
            "config_17",
        ];
        for filename in files {
            let filename_text = filename.to_owned() + ".txt";
            let file_name_set = filename.to_owned() + "_set.txt";
            let config = open_config_file(&filename_text);

            let expected = open_config_file(&file_name_set);

            let mut config_writer = ConfigWriter::new(config);
            let result = config_writer.write_configs();
            assert_eq!(result, expected);
        }
    }
}
