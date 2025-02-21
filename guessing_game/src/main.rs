use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess The Number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    
    loop {
        println!("Please input your guess.");
    
        let mut guess = String::new(); // Mutable
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");   

        // Trim is necessary because when you hit ENTER while inputing
        // a number it adds \r\n to the end of the string on windows
        
        // Parse allows for error handeling. If the parse method fails to convert
        // the value into a number it will through an error
        // Using the .expect method at the end will tell the user to make a valid input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("The secret number is {}", secret_number);
    }
}
