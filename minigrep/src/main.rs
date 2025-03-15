use std::env;
use std::fs;

fn main() {

    // .collect() can return many types of collections so we
    // specify the args to be of type Vec<String>
    let args: Vec<String> = env::args().collect();

    // Must start at index 1 because the programs name takes up
    let config = parse_config(&args);

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read from file");

    println!("Within text : \n{contents}");

}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
        // the first argument in args[0]
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        println!("Searching for {query}");
        println!("in file {file_path}");

        Config {query, file_path}
}
