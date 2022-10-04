use std::env;
use std::process;
use s43_minigrep_optmize::Config;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = s43_minigrep_optmize::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

