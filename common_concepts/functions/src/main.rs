fn main() {
    print_labeled_measurement(5, 'h');
    expression();
    let increment_result = increment(10);
    println!("The value of increment_result is: {increment_result}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn expression() {
    let y = {
        let x = 3;
        x + 1 // expression does not end with semicolon
    };

    println!("The value of y is: {y}");
}

fn increment(x: i32) -> i32 {
    x + 1 // since there's no semicolon, this is an expression. and x+1 is returned
}

// statements end in ';' and do not return a value
