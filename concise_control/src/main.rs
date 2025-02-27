
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl UsState {
    fn existed_in(&self, year: u16) -> bool {
        match self {
            UsState::Alabama => year >= 1819,
            UsState::Alaska => year >= 1959,
        }
    }
}

fn describe_state_quarter(coin: Coin) -> Option<String> {
    if let Coin::Quarter(state) = coin {
        if state.existed_in(1900) {
            Some(format!("{state:?} is pretty old, for America"))
        }
        else {
            Some(format!("{state:?} is relatively new!"))
        }
    } else {
        None
    }
}

fn main() {
    // This code gets rid of exaustive checking in exchange for concise control flow
    // If we used match here we'd have to check for None
    // You should only use this instead of match if you are looking for one pattern
    // and want to ignore all other patterns
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is {max}!");
    }
    else {
        println!("There is no max");
    }

    describe_state_quarter(Coin::Quarter(UsState::Alaska));
}
