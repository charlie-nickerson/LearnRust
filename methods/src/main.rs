#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.area() > rect.area()
    }

    fn square(size: u32) -> Self{
        self.width: size,
        self.height: size,
    }
}

fn main() {

    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rec2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rec3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("{}", rec1.can_hold(&rec2));
    println!("{}", rec1.can_hold(&rec3));




    // Calling the method using ::
}


