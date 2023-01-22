#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    //these are enums, they let you build multiple types all contained in one enum blueprint kinda thing
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let four = IpAddrKind::V4(127, 0, 0, 1); //this is how you write that data to a particular version of the enum
    let six = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("cool neat"));
    m.call();

    dbg!(&four, &six);
}

// fn route(ip_kind: IpAddrKind) {}
