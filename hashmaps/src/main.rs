fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");

    // .get returns and Option<&i32>
    // .copied handles Option by getting Option<i32> instead of Option<&i32>
    // .unwrap_or(0) sets score to zero if the team name does not have an entry.
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("{}", &score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }


}
