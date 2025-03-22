// What is Box<T>?
// - A smart pointer
// - Allows you to store data on the heap
// - Adds a pointer to the heap on the stack
// When to use Box<T>
// - When you have a type whose size is unknown at compile time
// - When you a large amount of data you want to transfer ownership but want to avoid the change of
// the data getting copied
// - When you want to own a value and you only care about its specific trait rather than it's type

// This fails because rusts compiler doesn't know how big Cons is. Its size could be infinite
// due to its recursive nature
// enum List {
//     Cons(i32, List),
//     Nil,
// }

enum List {
    Cons(i32, Box<List>),
    Nil,
}

// fn main() {
//     let b = Box::new(5); // b has a value 5 that is located on the heap
//     println!("b = {b}");
// }

use crate::List::{Cons, Nil};


fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
