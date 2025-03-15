use std::env;
use std::process;

use minigrep::Config;

fn main() {

    // .collect() can return many types of collections so we
    // specify the args to be of type Vec<String>
    let args: Vec<String> = env::args().collect();

    // We use build instead of new because programmers expect the new
    // method to never throw an error
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }

}
