pub mod configwriter;
mod lexer;
mod utils;

// Convert target Juniper configuration to 'set-style':
pub fn convert(configuration: &str) -> String {
    let mut config_writer = configwriter::ConfigWriter::new(configuration);
    let config_writer_result = config_writer.write_configs();
    return config_writer_result;
}

#[cfg(test)]
mod test {
    use super::convert;

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
}
