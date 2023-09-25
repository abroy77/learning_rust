fn main() {
    eg1();
    trying_else_if();
    if_as_expression();
}


fn eg1() {
    let number = 20;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn trying_else_if() {
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
fn if_as_expression() {
    let condition = false;
    let number = if condition {5} else {6}; // if and else must return the same type. else compile error
    // remember blocks evaluate to the last expression in them
    // numbers alone are expressions
    println!("The value of number is: {number}");
}
