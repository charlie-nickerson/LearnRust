#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}


fn main() {

    let scale = 2;
    let rect1 = Rect {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);

    println!("The area of the rectangle is {} square pixels",
     area(&rect1));

    println!("{rect1:#?}")
}

fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}