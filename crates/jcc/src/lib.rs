pub mod configwriter;
mod lexer;
mod utils;

// Convert target Juniper configuration to 'set-style':
pub fn convert(configuration: &str) -> String {
    let mut config_writer = configwriter::ConfigWriter::new(configuration);
    let config_writer_result = config_writer.write_configs();
    return config_writer_result;
}
