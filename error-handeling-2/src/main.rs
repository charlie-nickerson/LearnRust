use std::fs::File;
use std::error::Error;

// In production-quality code, most Rustaceans choose expect rather than unwrap 
// and give more context about why the operation is expected to always succeed.
// That way, if your assumptions are ever proven wrong, you have more information
// to use in debugging.


// you can read Box<dyn Error> to mean “any kind of error.”

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?; // If error, return it immediately
    let mut username = String::new();
    file.read_to_string(&mut username)?; // If error, return it immediately
    Ok(username) // Otherwise return the username wrapped in Ok
}

// Key points:

// The function using ? must return a compatible type (Result or Option)
// ? performs implicit type conversion via the From trait when needed
// You can chain multiple operations with ? on a single line
// It works with both Result and Option types
// In Rust 2018+, it can be used in main() if it returns Result

// The ? operator is one of Rust's most elegant features for writing concise, readable error handling code.
