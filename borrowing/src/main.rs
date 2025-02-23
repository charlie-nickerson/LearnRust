fn main() {
    let s1 = String::from("Hello");

    let len = calculate_len(&s1);

    println!("The lenght of {s1} is {len}")
}

fn calculate_len(s: &String) -> usize {
    s.len()
}
