mod front_of_house;

mod back_of_house {

    pub enum Appetizer {
        Soup,
        Salad
    }


    fn fix_incorrect_order() {
        cook_order(); // sibling
        super::deliver_order(); // calling parent moduler
    }

    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn get_fruit(&self) {
            println!("{}", self.seasonal_fruit);
        }
    }

}

fn deliver_order() {}

use crate::front_of_house::hosting;
use std::collections::HashMap;
use std::fmt::Result; // rust does not allow same name into scope
                      //
pub use crate::front_of_house::serving; // exporting so external can use restaurant::serving
                                        // directly without the front_of_house

// multiple imports
use std::{cmp::Ordering};
use std::io::{self, Write, Result as IoResult};

pub fn eat_at_restaurant() {


    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();
    
    // Relative path
    front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("I'd like {} toas please", meal.toast);

    // meal.seasonal_fruit = String::from("test"); // won't work because private
    meal.get_fruit();

    let order = back_of_house::Appetizer::Soup;

    hosting::add_to_waitlist(); // the convention is to use the parent mod of the fn

    let mut map = HashMap::new();
    map.insert(1, 2); // struct
}


