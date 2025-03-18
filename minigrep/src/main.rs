use std::env;
use std::process;

use minigrep::Config;

fn main() {

    // We use build instead of new because programmers expect the new
    // method to never throw an error
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}
