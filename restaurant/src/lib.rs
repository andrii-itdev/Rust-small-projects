#[cfg(test)]

mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Your order was added to waistlist!")
        }
        fn seat_at_table() {
            println!("Please sit down over there.");
        }
    }

    pub mod serving {
        fn take_order() {
            println!("Your order was received.");
        }
        pub fn serve_order() {
            println!("Bon' apetite!")
        }
        fn take_payment() {
            println!("Thank you!")
        }
    }
}


fn serve_order() {
    crate::front_of_house::serving::serve_order();
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    
    pub struct Breakfast {
        pub toast : String,
        seasonal_fruit : String,
    }
    
    impl Breakfast {
        pub fn summer(toast : &str) -> Breakfast {
            Breakfast {
                toast : String::from(toast),
                seasonal_fruit : String::from("peaches")
            }
        }
    }
    
    fn cook() { 
        println!("The dish is being cooked!");
    }
    
    pub fn fix_incorrect_order() {
        cook();
        super::serve_order();
    }
}

pub use crate::front_of_house::hosting as h;
use crate::front_of_house::hosting::add_to_waitlist;
use self::back_of_house::Appetizer as app;
use std::collections::HashMap;


pub fn eat_at_restaurant() {
    add_to_waitlist();
    h::add_to_waitlist();

    let order1 = app::Soup;
    let order2 = app::Salad;

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let mut map = HashMap::new();
    map.insert(1, 2);
    println!("{:?}", map);

}

use std::fmt;
use std::io::Result as IoResult;

fn function1() ->fmt::Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}
