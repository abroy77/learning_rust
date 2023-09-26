#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Coin {
    fn get_value(&self) -> u8 {
        match self {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
#[derive(Debug)]
enum Coin2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin2 {
    fn get_value(&self) -> u8 {
        match self {
            Coin2::Penny => 1,
            Coin2::Nickel => 5,
            Coin2::Dime => 10,
            Coin2::Quarter(state) => {
                println!("State quarter from: {:?}", state);
                25
            }
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let c1 = Coin::Quarter;
    println!("value of {:?} is {}", c1, c1.get_value());

    let c2 = Coin2::Quarter(UsState::Alabama);
    println!("value of {:?} is {}", c2, c2.get_value());

    let x: i32 = 5;
    let result = plus_one(Some(x));
    let result = match result {
        None => 0,
        Some(i) => i,
    };

    println!("adding one to {} gives us {}", x, result);

    // if let construction for control flow

    let config_max = Some(3u8);
    match config_max {
        Some(max) => {
            println!(" the max is configured to {}", max);
        }
        _ => (),
    }

    // that was long. we can shorten using if let mechanism

    if let Some(max) = config_max {
        println!(" the max is configured to {}", max);
    }

    let c3 = Coin2::Penny;
    let mut count = 0;
    if let Coin2::Quarter(state) = c3 {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("{count}");
}
