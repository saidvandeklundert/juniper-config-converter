/*

    RUST_LOG=debug cargo run -- --help
    cargo run -- --file config_17
    cargo run -- --file config_17 -v v
*/
use clap::Parser;
use env_logger::Env;
use jcc::convert;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the file to open, e.g.: /home/klundert/rust/projects/jsc
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
fn main() {
    let args = Args::parse();

    let env = Env::default()
        .filter_or("RUST_LOG", get_log_level(&args.verbose))
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    println!("Hello {}!", args.file);
    convert(&args.file);
}
