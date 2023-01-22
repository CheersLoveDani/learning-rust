fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("config_max is: {}", max);
    }; // can use else{} here as a catch all, if you are checking more than one type though best to use match
}
