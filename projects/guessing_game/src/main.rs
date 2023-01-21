use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // println! is essentially console.log()
    println!("Guess the number!");

    // randomness isnt a core library we imported it in the Cargo.toml file, we lay out using imported crates (like packages) functions with the :: operator a bit like using React.useEffect and such in react and js
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    //This is just to show us the secret number for testing, notice also we can pass the name of a function into a println! inbetween the curlys like this {variable} but wouldn't be able to do something more complex like {variable * 2},
    //this would need to be done like so ("text with {}", variable * 2)
    println!("Not so secret number is {secret_number}");

    //loop runs over and over until it hits a break; spooky infinite loops beware~
    loop {
        println!("Please input your guess.");

        // let doesn't mean its changeable or "Mutable", we need to write the mut keyword for that
        let mut guess: String = String::new();

        //another imported library this time for getting user input in the console.
        io::stdin()
            // Notice we use the & indicator to show this is a reference (and not a copy) to the guess variable, it is &mut because guess it Mutable otherwise it would be &guess (I think)
            .read_line(&mut guess)
            //expect either returns the resulting Ok value or errors the program with the msg: you pass it
            .expect("Failed to read line");

        //This is called shadowing by naming the same variable again (freaks me out atm ngl, but more will be explained later), this is used mostly for changing the variables type, lets break it down:
        //guess: u32 is saying that we want to change it to a type u32 (unsigned 32) which is a number type.
        //.trim() simply removes the whitespace around the String
        //.parse() is the bit that actually changes the type from string to u32 (not currently sure how it knows to be u32? maybe its a clever compiling thing?)
        //match is error handling which we all know and love, it works when something returns a Result enum (more on this later) and checks if its Ok(it succeeds) or it Err(it errors) and runs code depending on which
        //note Err(_) is a catch all errors, which is helpful.
        //finally continue; skips the rest of the loop and runs the next one.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        //cmp is another core library which helps with sorting numbers, in this case checking if something is Less than, Greater than or Equal to. fairly self explanatory.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
