#[derive(Debug, PartialEq, Copy, Clone)]
// This Enum stores values for Red and Blue shirt colors. Idk what type these values are
enum ShirtColor {
    Red,
    Blue,
}


// The Inventory struct stores a vector of shirt colors
struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    // Returns the users preference which is a shirt color
    // If the user preference is empty it will return the most stocked shirt color
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
    // 
    fn most_stocked(&self) -> ShirtColor {
        // Not sure why we initialize the number of shirts inside the most_stocked method
        // This value she be kept track of in a struct or something.
        let mut num_red = 0;
        let mut num_blue = 0;

        // This for loop seems to count how many of each color of shirt there is by iterating through the shirts vector
        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        // if num_red and num_blue are the same it will always return a blue shirt
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

fn main() {
    // Initializes the shirts inventory for the store
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    // Sets the preference for the first user to be red
    let user_pref1 = Some(ShirtColor::Red);
    // Sets the shirt to be given to the user using user 1's preference
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );
    

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    

}
