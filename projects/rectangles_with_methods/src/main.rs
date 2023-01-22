#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        //We are passing the Rectangle instances own values as self
        self.width * self.height
    }
    fn width(&self) -> bool {
        //we can name methods the same as one of the structs fields
        self.width > 0
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        //methods can take arguments and those arguments can be of the same type of struct
        self.width > rect.width && self.height > rect.height
    }
}
impl Rectangle {
    //you can have multiple impl blocks, not that its useful here
    fn square(size: u32) -> Self {
        //associated functions are not methods as you dont call them on an instance, they are called like this Rectangle::square()
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = dbg!(Rectangle::square(10)); //this is how you would call an associated function
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle {:?} is {} square pixels",
        rect1,
        rect1.area() //now we can just call the method on the instance which we want it to run on. (kinda like object methods in js)
    );

    dbg!(&rect1);

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    dbg!(rect1.can_hold(&rect2)); //This is how we pass in other arguments to methods, note we still don't pass in &self
    dbg!(rect1.can_hold(&rect3));
}

//if this ^ looks like its missing some comments then check out ../rectangles, which explains a few other things
