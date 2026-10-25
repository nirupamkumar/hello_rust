use colored::*;
use serde::Deserialize;
use std::io;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

fn get_weather_info(
    city: &str,
    country_code: &str,
    api_key: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );

    let response = reqwest::blocking::get(&url)?.error_for_status()?;
    let response_json = response.json::<WeatherResponse>()?;

    Ok(response_json)
}

fn get_temperature_emoji(temp: f64) -> &'static str {
    if temp < 0.0 {
        "🥶"
    } else if temp < 10.0 {
        "❄️"
    } else if temp < 20.0 {
        "🌤️"
    } else if temp < 30.0 {
        "☀️"
    } else {
        "🔥"
    }
}

fn display_weather_info(response: &WeatherResponse) {
    //let description = &response.weather[0].description;
    let description = match response.weather.first() {
        Some(weather) => &weather.description,
        None => {
            println!("Weather description is unavailable.");
            return;
        }
    };

    let colored_description = match description.as_str() {
        "clear sky" => description.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => description.bright_cyan(),
        "shower rain" | "rain" | "light rain" => description.bright_blue(),
        "thunderstrom" => description.bright_magenta(),
        "snow" => description.white(),
        "mist" | "fog" | "haze" => description.bright_black(),
        _ => description.normal(),
    };

    println!("Weather: {}", colored_description);

    let temp = response.main.temp;
    let humidity = response.main.humidity;
    let pressure = response.main.pressure;
    let wind_speed = response.wind.speed;

    println!("Temperature: {}°C {}", temp, get_temperature_emoji(temp));
    println!("Humidity: {}%", humidity);
    println!("Pressure: {} hPa", pressure);
    println!("Wind Speed: {} m/s", wind_speed);

    println!(
        "{}",
        format!("Weather in {}:", response.name).bright_green()
    );
}

fn main() {
    println!("{}", "Welcome to Weather Station".bright_yellow());

    dotenvy::dotenv().ok();

    let api_key = std::env::var("OPENWEATHER_API_KEY")
        .expect("OPENWEATHER_API_KEY must be set in your .env file");

    loop {
        println!("{}", "Please enter the name of the city:".bright_green());
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Failed to read input");
        let city = city.trim();

        println!(
            "{}",
            "Please enter the country code (eg., IN for India):".bright_green()
        );
        let mut country_code = String::new();
        io::stdin()
            .read_line(&mut country_code)
            .expect("Failed to read input");
        let country_code = country_code.trim();

        match get_weather_info(city, country_code, &api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(error) => {
                println!("Error fetching weather data: {}", error);
            }
        }

        println!("You entered: {}, {}", city, country_code);

        println!(
            "{}",
            "Do you want to search for weather in another city? (yes/no):".bright_green()
        );
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for using our software!");
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn temperature_below_zero_returns_freezing_emoji() {
        assert_eq!(get_temperature_emoji(-5.0), "🥶");
    }
}
