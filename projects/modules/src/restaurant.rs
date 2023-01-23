pub mod food;

pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("added to waitlist");
        }

        pub fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {
            println!("took order");
        }

        pub fn serve_order() {}

        pub fn take_payment() {}
    }
    pub fn eat_pizza() {
        println!("I ate some pizza!")
    }
}
