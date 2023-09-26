#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum NewIpAddr {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method defined here
    }
}

//IMPLEMETED IN STD LIB AND INCLUDED IN PRECLUDE
// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: four,
        address: String::from("192.168.1.1"),
    };

    dbg!(&home);

    let loopback = IpAddr {
        kind: six,
        address: String::from("168.61.23.1"),
    };

    dbg!(&loopback);

    let new_home = NewIpAddr::V4(String::from("123.456.789"));
    dbg!(&new_home);

    let new_loopback = NewIpAddr::V6(String::from("::1"));
    dbg!(&new_loopback);

    let a = IpAddr2::V4(127, 0, 0, 1);
    let b = IpAddr2::V6(String::from("::1"));

    println!("here's a = {:?}", a);
    println!("here's b = {:?}", b);

    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
