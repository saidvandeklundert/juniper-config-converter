use jcc::convert;
use std::fs::{self};
use std::io::prelude::*;
use std::path::Path;

// Read the file paths in a directory so that everytime we place a config and set config file
// in tests/data/, we can automatically collect them as test cases:
fn read_dirs() -> Vec<String> {
    let paths = fs::read_dir(&Path::new("tests/data/")).unwrap();

    let names = paths
        .map(|entry| {
            let entry = entry.unwrap();

            let entry_path = entry.path();

            let file_path_as_str = entry_path.to_str().unwrap();

            let file_path_as_string = String::from(file_path_as_str);

            file_path_as_string
        })
        .filter(|element| !element.contains("_set"))
        .collect::<Vec<String>>();
    names
}
// Open a configuration file and return the content as a String:
fn open_config_file(file_path: &str) -> String {
    let mut file = match fs::File::open(&file_path) {
        Err(error) => panic!("could not open {}: {}", file_path, error),
        Ok(file) => file,
    };

    let mut string = String::new();

    match file.read_to_string(&mut string) {
        Err(error) => panic!("could not read {}: {}", file_path, error),
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

// Integration test that:
// - reads all filenames from /test/data
// - opens all the combinations of config_x.txt and config_x_set.txt
// - converts the config_x.txt to 'set'-style config
// - asserts the converted configuration is the same as the config_x_set.txt file content
#[test]
fn config_convert_files() {
    let files: Vec<String> = read_dirs();
    for filename in files {
        let filename_text = filename.to_owned();
        let file_name_set = filename.to_owned().replace(".txt", "_set.txt");
        let config = open_config_file(&filename_text);

        let expected = open_config_file(&file_name_set);

        let result = convert(&config);
        assert_eq!(result, expected);
    }
}
