use crate::restaurant::food::Pizza;

pub mod restaurant;

fn main() {
    let new_pizza = Pizza {
        sauce: String::from("tomato"),
        topping: String::from("pepperoni"),
    };

    println!("{:#?}", new_pizza);
}
