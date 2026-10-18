use std::io; // NOTE: unused import warning until main uses it
use reqwest::Response; // CHANGE: remove this import; blocking::get returns reqwest::blocking::Response, so it's not needed
use serde::Deserialize;
use colored::*; // NOTE: unused import warning until main uses it

#[derive(Deserialize, Debug)]
struct WeatherResponse{
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

fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!("https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}", city, country_code, api_key);
    let response = reqwest::blocking::get(&url)?;
    // CHANGE: `Response` is a type, use the local variable `response` (lowercase)
    let response_json: WeatherResponse = Response.json::<WeatherResponse>()?;
    // CHANGE: remove the trailing semicolon so the Result is returned instead of ()
    Ok(response_json);
}

// NOTE: main never calls get_weather_info yet
// NOTE: Cargo.toml needs reqwest features ["blocking", "json"] for .json() on a blocking response
fn main() {
    println!("Hello, World!");
}