use clap::Parser;
use env_logger::Env;
use jcc::convert;
use log::info;
use std::fs;
use std::io::prelude::*;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to open, e.g.: /tmp/config.txt
    #[arg(short, long)]
    file: String,

    /// Verbosity, e.g.: vv or v
    #[arg(short, long, default_value_t = String::from("v"))]
    verbose: String,
}

fn get_log_level(verbose: &str) -> &str {
    match verbose {
        "v" => "error",
        "vv" => "warning",
        "vvv" => "info",
        "vvvv" => "debug",
        _ => panic!("Verbosity ranges from v to vvvv"),
    }
}

//Open target configuration file.
fn open_config_file(file: &str) -> String {
    let path = Path::new(file);
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

fn main() {
    let args = Args::parse();

    let env = Env::default()
        .filter_or("RUST_LOG", get_log_level(&args.verbose))
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let config = open_config_file(&args.file);

    let converted_config = convert(&config);
    info!("Config to translate:\n{}", config);
    info!("Translated configuration:\n");
    println!("{}", converted_config);
}
