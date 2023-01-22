fn main() {
    let five = Some(5); //Some is an option which is neede to pass in to plus_one
    let six = plus_one(five);
    let none = plus_one(None);

    dbg!(five.unwrap(), six, none); // .unwrap() gives us back the value if Some but panics if None
    dice_roll(3);
    dice_roll(9);
    dice_roll(5);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => {
            dbg!(x);
            None
        }
        Some(i) => Some(i + 1),
    }
}

fn dice_roll(num: u32) {
    let dice_roll = num;
    match dice_roll {
        3 => println!("three"),
        9 => println!("nine"),
        other => println!("lame {}", other),
        //^ catchall with a usable variable
        // _ => {...code} for a catchall which runs something
        // _ => () for a catchall that doesnt do anything
    }
}
