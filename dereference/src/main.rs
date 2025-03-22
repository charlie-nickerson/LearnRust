


// fn main() {
//     let x = 5;
//     let y = &x;

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }


// USING Box<T> like a reference

// fn main() {
//     let x = 5;
//     let y = Box::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }

// How is Box Defined?

// Tuple struct with one value
struct MyBox<T>(T);

// Does not work unless we havent implemented teh Dereference trait
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


// Using the MyBox<T> implementation

// fn main() {
//     let x = 5;
//     let y = MyBox::new(x);

//     assert_eq!(5, x);
//     assert_eq!(5, *y);
// }

fn hello(name: &str) {
    println!("Hello, {name}");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    // Deref coersion makes it possible to call hello with a reference to a value of type 
    // MyBox<String>
    hello(&m);
}