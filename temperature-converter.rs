/* Catană Ioan-Alexandru           */ 
/* Fahrenheit to Celsius converter */ 
/* 14.03.2023                      */ 

use std::io;

fn main() {
    let mut temperature = String::new();
    let mut preference = String::new();

    print_preferences();
    
    println!("Input your preference: ");
    io::stdin().read_line(&mut preference).expect("");

    let preference: i32 = preference.trim().parse().unwrap();

    match preference {
        1 => {
            println!("Input the temperature (°F): ");
            io::stdin().read_line(&mut temperature).expect("");

            let temperature: f64 = temperature.trim().parse().unwrap();

            let temperature = convert_to_c(temperature);

            println!("The temperature is {}°C", temperature);
        },
        2 => {
            println!("Input the temperature (°C): ");
            io::stdin().read_line(&mut temperature).expect("");

            let temperature: f64 = temperature.trim().parse().unwrap();

            let temperature = convert_to_f(temperature);

            println!("The temperature is {}°F", temperature);
        },
        _ => println!("Option is invalid"),
    }
}

fn convert_to_c(ftemp: f64) -> f64 {
    (ftemp - 32.0) * 0.5556
}

fn convert_to_f(ctemp: f64) -> f64 {
    (ctemp * 1.8) + 32.0
}

fn print_preferences() {
    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");
}