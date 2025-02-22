fn main() {
    println!("Hello, world!");

    let x = five();
    let x = plus_one(x);

    print_labeled_measurement(x, 'h');
}

fn print_labeled_measurement(x: i32, unit_label: char) {
    println!("The measurement is {x}{unit_label}");
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}