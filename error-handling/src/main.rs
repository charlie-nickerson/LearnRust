use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // if the file is not found create a file with name hello.txt
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating file: {e:?}"),
            },
            // Else if there still is an error print out the error
            other_error => { 
                panic!("Problem opening file: {other_error:?}");
            }
        },
    };
}
