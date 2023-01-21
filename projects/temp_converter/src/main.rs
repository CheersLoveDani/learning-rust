use std::io;

fn main() {
    loop {
        let mut temperature: String = String::new();
        println!("Input your temp in C as a number");

        io::stdin()
            .read_line(&mut temperature)
            .expect("Failed to read line");

        let temperature: u32 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let temperature: f64 = convert_celsius_to_fahrenheit(temperature);

        println!("In fahrenheit this is: {}", temperature);

        break;
    }
}

fn convert_celsius_to_fahrenheit(temperature: u32) -> f64 {
    let tempf = temperature as f64;
    ((tempf * 9.0) / 5.0) + 32.0
}
