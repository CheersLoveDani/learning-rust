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
fn main() {
    let penny_value: u8 = value_in_cents(Coin::Penny);
    let quarter_value = value_in_cents(Coin::Quarter(UsState::Alabama));

    println!("{}", penny_value);
    println!("{}", quarter_value);
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        //you can use match to run code based on what object enum type the coin is
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("This quarter is from {:?}", state); //you can also set an enum type within another enum type
            25
        }
    }
}
