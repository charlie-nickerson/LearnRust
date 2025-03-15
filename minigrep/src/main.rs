use std::env;
use std::fs;
use std::process;

fn main() {

    // .collect() can return many types of collections so we
    // specify the args to be of type Vec<String>
    let args: Vec<String> = env::args().collect();

    // We use build instead of new because programmers expect the new
    // method to never throw an error
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let contents = fs::read_to_string(config.file_path).expect("Should have been able to read from file");

    println!("Within text : \n{contents}");

}


// Putting query and file_path under the same struct indicates
// to programmers that these variables are related to each other
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // Changing the parse_config function into a method for config
    // shows the programmer that the function is strongly related
    // to the variables in config. 
    fn build(args: &[String]) -> Result<Config, &'static str>  {
                // the first argument in args[0]
                if args.len() < 3 {
                    // Error methods always return string literals with 'static lifetime
                    return Err("Too few arguments");
                }
                let query = args[1].clone();
                let file_path = args[2].clone();
            
                println!("Searching for {query}");
                println!("in file {file_path}");
        
                Ok(Config {query, file_path})
    }
}