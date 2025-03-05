fn main() {
    let word: String = String::from("blust");
    // Pass in a string reference
    let pig_word: String = convert_to_pig(&word);
    // Must return a new string reference
    println!("{pig_word}");

}


fn convert_to_pig(word: &str) -> String {
    let mut pig_word: String = word.to_string();
    if pig_word.starts_with(&['a', 'e', 'i', 'o', 'u']) {
        pig_word.push_str("hay");
        return pig_word;
    }
    
    let first_letter: char = word.chars().nth(0).unwrap();
    
    if pig_word.len() > 0 {
        pig_word.remove(0);
        pig_word.push(first_letter);
        pig_word.push_str("ay");
    }

    return pig_word;
}