#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let r1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    println!("The area of the rectangle is {}", area(&r1));
    println!("the rectangle is: {:?}", r1);
    println!("now pretty print");
    println!("pretty print rectangle\n{:#?}", r1);
    println!(" time to debug");
    dbg!(&r1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
