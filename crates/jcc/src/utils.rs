use log::info;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

// Open a configuration file and return the content as a String:
pub fn open_config_file(file_path: &str) -> String {
    let path = Path::new(file_path);
    let display = path.display();

    let mut file = match fs::File::open(&path) {
        Err(error) => panic!("could not open {}: {}", display, error),
        Ok(file) => file,
    };

    let mut string = String::new();

    match file.read_to_string(&mut string) {
        Err(error) => panic!("could not read {}: {}", display, error),
        Ok(_) => {
            return string;
        }
    }
}

// Take the stanza_stack_record and the config_line_stack to produce a syntactically valid Juniper
// set configuration command.
pub fn build_string(
    stanza_stack_record: &Vec<Vec<String>>,
    config_line_stack: &Vec<String>,
) -> String {
    let mut new_string = String::from("set");

    for vec in stanza_stack_record {
        for string in vec {
            new_string = new_string + " " + &string;
        }
    }
    for string in config_line_stack {
        new_string = new_string + " " + &string;
    }

    if new_string.ends_with(";") {
        new_string.pop();
    }
    info!("config_line:\n{new_string}");
    new_string
}

#[cfg(test)]
mod test {
    use super::build_string;

    #[test]
    fn test_build_string() {
        let stanza_stack_record = vec![
            vec![String::from("groups")],
            vec![String::from("BLOCK-V6")],
            vec![String::from("vlans")],
            vec![String::from("<*>")],
            vec![String::from("forwarding-options")],
            vec![String::from("filter")],
        ];
        let config_line_stack = vec![String::from("input"), String::from("BLOCK-IPv6;")];
        let result = build_string(&stanza_stack_record, &config_line_stack);
        let expected = String::from(
            "set groups BLOCK-V6 vlans <*> forwarding-options filter input BLOCK-IPv6",
        );
        assert_eq!(result, expected);
    }
}
