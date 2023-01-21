fn main() {
    hello_world("Functions n' stuff");
    let x = plus_one(1);
    let y = five();

    println!("x is {}, y is {}", x, y);
}

fn plus_one(number: i32) -> i32 {
    number + 1
}

fn five() -> i32 {
    5
}

fn hello_world(msg: &str) {
    let mut count: i32 = 0;
    loop {
        if count <= 6 {
            println!("{} {}", msg, count + 1);
            count += 1;
        } else {
            break;
        }
    }
}
