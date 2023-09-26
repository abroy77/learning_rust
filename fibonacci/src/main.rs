use std::io;

fn main() {
    println!("Welcome to the fibonacci printer!");
    let n = get_n();
    fibonacci(n);
}

fn get_n() -> u32 {
    println!(" How many numbers of the Fibonacci sequence do you want to print?");
    let mut n_str = String::new();

    let n: u32 = loop {
        io::stdin()
            .read_line(&mut n_str)
            .expect("Failed to read line");

        // convert to integer
        let n: u32 = match n_str.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        // check that n is greater than 0
        if n < 1 {
            println!("Please enter a number greater than 0");
            continue;
        }
        break n;
    };
    n
}

fn fibonacci(n: u32) {
    let mut a = 0;
    let mut b = 1;
    let mut _c = 0;
    println!("{}", "=".repeat(40));
    for _ in 0..n {
        println!("{a}");
        _c = a + b;
        a = b;
        b = _c;
    }
}
