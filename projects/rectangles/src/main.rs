#[derive(Debug)] //this lets us use some nice println! formatting, see ln 17
struct Rectangle {
    //creating the struct Rectangle, pretty standard
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2; //standard defining values
    let rect1 = Rectangle {
        //creating an instance of rectangle with our own values
        width: dbg!(30 * scale), //see below for more of the magic of dbg! (debug)
        height: 50,
    };

    println!(
        "The area of the rectangle {:?} is {} square pixels", //not sure if I mentioned this before but :? can be used to print {} stuff with [derive(debug)] outer attribute
        rect1,
        area(&rect1) //passing a reference so it doesn't kill the data when the function is over
    );

    dbg!(&rect1); //Note: although dbg! does return ownership, if it doesn't have anything to return ownership to then it still gets killed. References should be used
}

fn area(rectangle: &Rectangle) -> u32 {
    //We are passing in rect1 AS a Rectangle instance, neat coding if you ask me
    rectangle.width * rectangle.height
}
