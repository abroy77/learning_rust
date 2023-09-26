#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, r2: &Rectangle) -> bool {
        (self.width > r2.width) && (self.height > r2.height)
    }
    fn new_square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let r1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {} sq px.", r1.area());
    if r1.width() {
        println!(" Non-zero width is {}", r1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(" Can r1 hold r2 {}", r1.can_hold(&rect2));
    println!(" Can r1 hold r3 {}", r1.can_hold(&rect3));

    let square = Rectangle::new_square(10);
    println!("square is\n {:#?}", square);
}
