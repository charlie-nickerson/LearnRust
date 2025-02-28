mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}


use crate::front_of_house::hosting;

    // Relative path
mod customer {

    pub fn eat_at_restaurant() {
        super::hosting::add_to_waitlist();
    }
}

fn main(){

}