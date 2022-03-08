mod front_of_house;

mod back_of_house;


// pub use crate::back_of_house::{Breakfast,Appetizer};


pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please",meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::eat_at_restaurant();
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}