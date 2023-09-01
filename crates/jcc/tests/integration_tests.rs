use jcc::convert;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
// Open a configuration file and return the content as a String:
fn open_config_file(file_path: &str) -> String {
    let to_open = "tests/data/".to_owned() + &file_path.to_owned();

    let path = Path::new(&to_open);
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

#[test]
fn test_library_interface() {
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
    let result = convert(&input);
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
        "config_18",
        "config_19",
        "config_20",
        "config_21",
    ];
    for filename in files {
        let filename_text = filename.to_owned() + ".txt";
        let file_name_set = filename.to_owned() + "_set.txt";
        let config = open_config_file(&filename_text);

        let expected = open_config_file(&file_name_set);

        let result = convert(&config);
        assert_eq!(result, expected);
    }
}
