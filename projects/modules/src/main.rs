use restaurant::food::Pizza;
use restaurant::front_of_house::{self, hosting, serving};

pub mod restaurant;
fn main() {
    let new_pizza = Pizza {
        sauce: String::from("tomato"),
        topping: String::from("pepperoni"),
    };
    hosting::add_to_waitlist();
    serving::take_order();
    println!("{:#?}", new_pizza);
    front_of_house::eat_pizza();
}
