//main as usual runs through everything
fn main() {
    pushing_to_strings();
    variables_interacting();
    strings_interacting();
    giving_and_taking_ownership();
    mutating_references();
}

//---------------------------------------------------------------------------------------------------------------------------------------------

fn pushing_to_strings() {
    let mut new_string = String::from("Hello"); //This makes a String from a str
    println!("{}", new_string);
    new_string.push_str(", world!"); //.push_str is like .append() from js, basically adds to the end of the string
    println!("{}", new_string);
}

//---------------------------------------------------------------------------------------------------------------------------------------------

fn variables_interacting() {
    let x = 5; //copys the data because this is a simple data type and goes on the stack
    let y = x; //both y and x are valid
    println!("Both x and y are valid: {} {}", x, y);
}

//---------------------------------------------------------------------------------------------------------------------------------------------

fn strings_interacting() {
    let s1 = String::from("hello"); //this is a complex data type and therefor is a pointer and goes on the heap.
    let s2 = s1; //s2 copys the pointer and makes s1 invalid to avoid double dropping errors
    println!("Only s2 is valid: {}", s2);
    let s3 = s2.clone(); //.clone() does what it says in the tin, it clones all the data making s3 a completely separate value
    println!("Both s2 and s3 are valid: {}, {}", s2, s3);
}

fn giving_and_taking_ownership() {
    let string1 = String::from("string1");
    takes_ownership(string1); //string1 can no longer be used in this scope since it was moved to takes_ownership()

    let string2 = String::from("string2");
    takes_reference(&string2); //string2 can still be used in this scope as it keeps ownership
    println!("main keeps ownership: {}", string2);
}

fn takes_ownership(string: String) {
    //because this is a String when it recieves this variable it takes ownership and it can no longer be used outside of the function
    println!("main loses ownership of: {}", string);
} //string is dropped and taken out of memory

//returning will transfer ownership also

//---------------------------------------------------------------------------------------------------------------------------------------------

fn takes_reference(string: &String) {
    //because this takes a reference the function does not take ownership of the value
    println!("the function takes a reference: {}", string); //because this is a reference and not ownership, string cannot be modified
} //The value isnt dropped here because the function doesnt have ownership

fn mutating_references() {
    let mut string3 = String::from("string3"); //mutatable value
    mutate_reference(&mut string3); //Mutatable references let functions edit values without owning them, letting the owner still use the value
    println!("and is in the owner as: '{}'", string3) //only one mutable reference can exist at one time but unlimited immutable references
}

fn mutate_reference(string: &mut String) {
    println!("reference starts as '{}'", string);
    string.push_str(" mutated"); //this string is mutated for the owner and the reference can still see the updated value
    println!("is mutated into '{}'", string);
} //reminds me of passing props in react useState, they update on the parent component but can be used by the child

//---------------------------------------------------------------------------------------------------------------------------------------------

// you cannot return a &reference from a function because the original value will be dropped so it will be referencing nothing
