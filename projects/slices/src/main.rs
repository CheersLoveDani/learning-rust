fn main() {
    let sentence = String::from("This is a cool big sentence to slice up");
    println!("{}.", sentence);

    let first_word_in_sentence = first_word(&sentence);
    println!("{}", first_word_in_sentence);
}

fn first_word(string: &str) -> &str {
    let bytes = string.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &string[..i];
        }
    }

    &string[..]
}
