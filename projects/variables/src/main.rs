fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    //tuples
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("{:#?}", tup);

    //extracting data from tuples
    let (x, y, z) = tup;

    println!("x = {}, y = {}, z = {}", x, y, z);

    //accessing individual data from tuple
    println!("The third value of this tuple is: {}", tup.2);

    //Arrays: which have a static length in this language
    let array: [i32; 5] = [1, 2, 3, 4, 5];

    println!("This array is: {:?}", array);

    //lil shortcut for making an array of 5, 3s
    let array_of_3 = [3; 5];

    println!("Array of 5 3's: {:?}", array_of_3);

    //standard array value accessing
    println!("First of array: {}", array[0])
}
