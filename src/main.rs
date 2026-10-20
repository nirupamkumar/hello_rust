use std::io;
use colored::*;

fn main() {
    println!("{}", "Welcome to Weather Station".bright_yellow());

    loop {
        println!("{}", "Please enter the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input");
        let city = city.trim();

        println!("{}", "Please enter the country code (eg., IN for India):".bright_green());
        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input");
        let country_code = country_code.trim();

        println!("You entered: {}, {}", city, country_code);

        println!("{}", "Do you want to search for weather in another city? (yes/no):".bright_green());
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for using our software!");
            break;
        }
    }
}