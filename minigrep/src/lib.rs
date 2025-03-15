use std::fs;
use std::error::Error;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("Within text : \n{contents}");

    Ok(())
}


// Putting query and file_path under the same struct indicates
// to programmers that these variables are related to each other
pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // Changing the parse_config function into a method for config
    // shows the programmer that the function is strongly related
    // to the variables in config. 
    pub fn build(args: &[String]) -> Result<Config, &'static str>  {
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