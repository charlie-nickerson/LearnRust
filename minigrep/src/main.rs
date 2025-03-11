use std::env;
use std::fs;

fn main() {

    // .collect() can return many types of collections so we
    // specify the args to be of type Vec<String>
    let args: Vec<String> = env::args().collect();

    // Must start at index 1 because the programs name takes up
    // the first argument in args[0]
    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {query}");
    println!("in file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read from file");

    println!("Within text : \n{contents}");

}
