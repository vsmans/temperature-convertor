// App to confert temperature to fahrenheit or celcius

use std::io;

// declare constants
const CONVERSION_TO_C_RATIO: f64 = 5.0 / 9.0;
const CONVERSION_TO_F_RATIO: f64 = 9.0 / 5.0; 
const CONVERSION_OFFSET: f64 = 32.0;

struct Temperature {
    option: i32,
    value: f64,
}

// function to convert temperature
impl Temperature {
    fn convert(&self) -> f64 {
        if self.option == 1 {
            (self.value - CONVERSION_OFFSET) * CONVERSION_TO_C_RATIO  
        } else {
            (self.value * CONVERSION_TO_F_RATIO) + CONVERSION_OFFSET 
        }
    }
}

// function to get user input
fn get_user_input() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input;
} 

// function to get users conversion option
fn user_option() -> i32 {
    println!("Please select the unit of measure to convert to:");
    println!("1. Celsius\n2. Fahrenheit\nPlease select option '1' or '2':");
    loop {
        let user_option: String = get_user_input();
        let user_option: i32 = match user_option.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid. Please enter a number:"); 
                continue;
            },
        };

        if user_option == 1 || user_option == 2 {
            return user_option;
        } else {
            println!("Invalid. Please enter a value of '1' or '2':");
        }
    }
}

// function to get user temperature value
fn user_temperature() -> f64 {
    println!("Please provide the temperature value:");
    loop {
        let user_temperature: String = get_user_input();
        let user_temperature: f64 = match user_temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid. Please enter a number:");
                continue;
            },
        };
        return user_temperature;
    }
}

fn main() {
    println!("Temperature conversion");

    let temperature: Temperature = Temperature {
        option: user_option(),
        value:  user_temperature(),
    };
    
    if temperature.option == 1 {
        println!("{} degrees Fahrenheit is {} Celcius", temperature.option, temperature.convert());
    } else {
        println!("{} degrees Celcius is {} Fahrenheit", temperature.option, temperature.convert());
    }
}
