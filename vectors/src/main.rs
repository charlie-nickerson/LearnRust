
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}


fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[3];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(8);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Vectors can have multiple different types stored in them using enums
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(1.3),
        SpreadsheetCell::Text(String::from("blue")),
    ];

}
