mod front_of_house;
pub use crate::front_of_house::hosting;
/* 1-
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    pub mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}
*/
mod back_of_house {
    // struct need to make each element pub
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // need a pub fu to create a Breakfast instance
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peach"),
            }
        }
    }

    // but we can just make enumerate pub here  
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        // 1- super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}


// 1 - use crate::front_of_house::hosting;
// 1 - use self::front_of_house::hosting as front_host;
pub fn eat_at_restaurant() {
    // abs path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();

    // use
    hosting::add_to_waitlist();
    // front_host::add_to_waitlist();

    // struct
    // order summer breakfast
    let mut meal = back_of_house::Breakfast::summer("black choco");
    // change to white choco, can't change seasonal fruit
    meal.toast = String::from("white choco");
    println!("i want {} bread", meal.toast);

    // enum
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

