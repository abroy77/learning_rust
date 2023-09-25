// loops like loop, while, for
fn main() {
    loopy_loop();
    loop_return();
    loop_labels();
    while_looping();
    for_looping();
    countdown_for();
}    
fn loopy_loop() {
    loop {
        println!("looping!");
        break; // break out of loop
    }
}

fn loop_return() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // this is the value that's returned
        }
    };

    println!("The result is {result}");
}


fn loop_labels() {
    println!("running loop_labels");
    let mut count = 0;
    'counting_up: loop { //  single quote to label a loop. only opening quote, not closing quote. wierd 
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // v cool. can break out of specified loop. not just innermost loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// while loops
fn while_looping() {
    println!("running while_looping");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

// for loops. can be used over collections like arrays woo
// when iterating over a collection, it's safer to use a for loop than a while loop
// also faster because array length is fixed so for loop already knows how many iterations
// and does not need to test against a condition on each iteration

fn for_looping() {
    println!("running for_looping");

    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
fn countdown_for() {
    println!("running countdown_for");
    for number in (1..4).rev() { // rev() reverses the range
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}