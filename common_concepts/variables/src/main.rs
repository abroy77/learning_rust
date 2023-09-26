fn main() {
    let mut x: i32 = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");

    // shadowing
    let y: i32 = 5;
    println!("The value of y is: {y}");
    {
        let y = 2 * y;
        println!("The value of y in the inner scope is: {y}");
    }
    let y = y + 1;
    println!("The value of y is shadowed to: {y}");

    let spaces = "     "; // shadowing to change type from str
    let spaces = spaces.len(); // to int

    println!("The value of spaces is: {spaces}");

    // with shadowing you can change types.
    // but with mutable variables you cannot change the type of the variable once it is assigned.
    // you can't do this:
    // let mut spaces = "     ";
    // spaces = spaces.len(); // error: mismatched types

    // Data types

    // Scalar types
    // Integers: signed and unsigned
    let x: u32 = 5; // unsigned 32 bit integer
    println!("The value of x unsigned int is: {x}");
    let x: i32 = 5; // signed 32 bit integer
    println!("The value of x signed int is: {x}");

    // Floating point numbers
    let x = 2.0; // f64
    println!("The value of x f64 is: {x}");
    let y: f32 = 3.0; // f32
    println!("The value of y f32 is: {y}");

    // math operations

    // addition
    let sum = 5 + 10;
    println!("The value of sum is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The value of difference is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The value of product is: {product}");
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    println!("The value of quotient is: {quotient}");
    println!("The value of truncated is: {truncated}");
    // remainder
    let remainder = 43 % 5;
    println!("The value of remainder is: {remainder}");

    // Boolean type
    let t = true;
    println!("The value of t is: {t}");
    let f: bool = false; // with explicit type annotation
    println!("The value of f is: {f}");

    // Character type
    let c = 'z';
    println!("The value of c is: {c}");
    let z: char = 'ℤ';
    println!("The value of z is: {z}");
    let heart_eyed_cat = '😻';
    println!("The value of heart_eyed_cat is: {heart_eyed_cat}");

    // Compound types
    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x, y, z is: {x}, {y}, {z}");
    // index access
    let five_hundred = tup.0;
    println!("The value of five_hundred is: {five_hundred}");
    let six_point_four = tup.1;
    println!("The value of six_point_four is: {six_point_four}");
    let one = tup.2;
    println!("The value of one is: {one}");

    // some more shadowing
    let spaces = "   ";
    println!("spaces in the start is {spaces}.");
    let spaces = spaces.len();
    println!("spaces later is {spaces}.");

    // Arrays
    // Rust arrays have fixed length
    // unlike tuple each element must have same size
    // unlike tuple, array can be mutable (not size though)

    let a = [1, 2, 3, 4, 5];

    // declaration

    let a: [i32; 5] = [1, 2, 3, 4, 5]; // the format is [type; size]

    // broadcast
    let a = [3; 5]; // the format is [value; size]

    // array indexing
    let first_element = a[0];
    println!(" first element of a is: {first_element}"); // note rust is zero indexed

    // functions
    another_function();
}

fn another_function() {
    println!("Another function.");
}
