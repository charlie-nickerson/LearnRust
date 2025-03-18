use std::fs;
use std::error::Error;
use std::env;


pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}


// Putting query and file_path under the same struct indicates
// to programmers that these variables are related to each other
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
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

                let ignore_case = env::var("IGNORE_CASE").is_ok();
            
                println!("Searching for {query}");
                println!("in file {file_path}");
        
                Ok(Config {query, file_path, ignore_case,})
    }
}

// We indicate that the lifetime of the data returned
// is as long as the lifetime of the data passed into
// the contents argument of the search function
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let results = contents
                    .lines()
                    .into_iter()
                    .filter(|l| l.contains(query)).collect();
    results
}

pub fn search_case_insensitive<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let results = contents
                        .lines()
                        .into_iter()
                        .filter(|l| l.to_lowercase().contains(&query)).collect();
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}