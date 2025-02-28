fn main() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[3];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(8);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

}
