fn main() {
    let mut v1: Vec<i32> = Vec::new(); //if the vector we are creating is empty we need to specify what data type it will hold with <i23>
    let mut v2 = vec![4, 5, 6]; //if we have data already we can just put it in with the vec! macro and rust will figure it out

    v2.push(7); //pushing values! Just like arrays in js!
    v2.push(8);
    v2.push(9);

    let mut i = 1; //while loop i value!
    while i < 16 {
        //while loop!
        v1.push(i);
        i = i + 1;
    }

    modify_vector(&mut v1); //Passing in a mutable reference so we dont have to pass ownership back

    print_all_vector_values(&v1); //Just passing in a standard reference because we arent changing the Vector

    print_all_vector_values(&v2);

    println!("{}", find_third_in_vector(&v1)); //Using function return values is values is standard

    #[derive(Debug)]
    enum SpreadsheetCell {
        //If we want a Vector with a bunch of different types then enumerators are very helpful!
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        //Just state we are going to use an enum as the type in Vec<>, or leave it and rust will figure it out
        SpreadsheetCell::Int(4),
        SpreadsheetCell::Float(6.3),
        SpreadsheetCell::Text(String::from("happy luck goldfish")),
    ];

    dbg!(&row[0]);
}

fn find_third_in_vector(vector: &Vec<i32>) -> &i32 {
    let third: Option<&i32> = vector.get(2);
    match third {
        Some(third) => third,
        None => {
            println!(
                "no third value in {:?}, last index is {:?}, returning 0",
                vector,
                vector.len() - 1
            );
            &0
        }
    }
}

fn modify_vector(vector: &mut Vec<i32>) {
    for value in vector {
        *value *= 32;
    }
}

fn print_all_vector_values(vector: &Vec<i32>) {
    for (i, value) in vector.iter().enumerate() {
        //                          This ^-----------------^ is a useful way of getting an index value for a for loop!
        println!("value at index {} of vector is: {}", i, value)
    }
}
