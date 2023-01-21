use std::io;

fn main() {
    let mut temperature: String = String::new();

    println!("Input your temp in C");

    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    println!("{}", temperature);
}
