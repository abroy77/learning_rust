use std::io;

fn main() {
    println!("Welcome to the temperature conversion program!");
    println!("What scale are you converting from? (C or F)");
    let mut scale: String = String::new();

    io::stdin()
        .read_line(&mut scale)
        .expect("Failed to read line");

    let scale = scale.trim();
    println!("You entered: {scale}");


    if scale == "F" {
        println!("What is the temperature in Fahrenheit?");
        let input_temperature = get_input_temperature();
        let output_temperature = (input_temperature - 32.0) * 5.0 / 9.0;
        println!("The temperature in Celsius is: {output_temperature}");
    }
    else if scale == "C" {
        println!("What is the temperature in Celsius?");
        let input_temperature = get_input_temperature();
        let output_temperature = (input_temperature * 9.0 / 5.0) + 32.0;
        println!("The temperature in Fahrenheit is: {output_temperature}");
    }
    else {
        println!("Please enter either C or F");
        main();
    }
    }


fn get_input_temperature() -> f64 {
    let mut input_temperature: String = String::new();
    io::stdin()
        .read_line(&mut input_temperature)
        .expect("Failed to read line");

    let input_temperature: f64 = match input_temperature.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a number");
            get_input_temperature()
        },
    };
    input_temperature
}
