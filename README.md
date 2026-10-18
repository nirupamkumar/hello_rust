# 🌤️ hello_rust: Terminal Weather App

A lightweight terminal weather app written in Rust. Look up the current weather for any city using the OpenWeatherMap API, without leaving your command line.

> Developed on Windows 10 with VS Code. Should also work on macOS and Linux.

## Features

- Current weather for any city worldwide
- Temperature, feels-like, humidity, wind, and conditions
- Colored, readable terminal output
- Clear error messages (city not found, no internet, invalid API key)
- Powered by the [OpenWeatherMap Current Weather API](https://openweathermap.org/current)

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) **1.85 or newer** (the project uses Rust edition 2024), installed via `rustup`
- On Windows 10: the **Visual Studio C++ Build Tools** (the `rustup` installer will prompt you for them)
- [VS Code](https://code.visualstudio.com/) with the **rust-analyzer** extension (recommended)
- A free [OpenWeatherMap](https://openweathermap.org/api) API key (see [Configuration](#configuration))

Check your install:

```powershell
rustc --version
cargo --version
```

If your Rust is older than 1.85, update it:

```powershell
rustup update
```

## Installation

```powershell
git clone https://github.com/<your-username>/hello_rust.git
cd hello_rust
cargo build --release
```

The compiled binary will be at `target\release\hello_rust.exe`.

## Configuration

This app uses the [OpenWeatherMap API](https://openweathermap.org/api). The free plan is enough.

1. Create a free account at [openweathermap.org](https://home.openweathermap.org/users/sign_up).
2. Open the **API keys** tab in your account and copy your key.
3. Wait for the key to activate (this can take from a few minutes up to a couple of hours).
4. Set it as an environment variable:

**PowerShell (current session only):**

```powershell
$env:WEATHER_API_KEY = "your_api_key_here"
```

**PowerShell / CMD (permanent):**

```powershell
setx WEATHER_API_KEY "your_api_key_here"
```

Restart your terminal (and VS Code) after using `setx`.

> ⚠️ Never commit your API key to Git.

## Usage

Run the app with Cargo:

```powershell
cargo run
```

Or run the release build directly:

```powershell
.\target\release\hello_rust.exe
```

Then enter a city name when prompted, for example `London` or `Hyderabad`.

## Example Output

```
Weather in London, GB
-----------------------------
Condition : Partly cloudy
Temp      : 14°C (feels like 12°C)
Humidity  : 72%
Wind      : 5 m/s
```

## Development in VS Code

1. Open the project folder in VS Code (`File > Open Folder`).
2. Install the **rust-analyzer** extension.
3. Open the integrated terminal (`` Ctrl+` ``) and run `cargo run`.

Useful commands:

```powershell
cargo check          # fast compile check
cargo fmt            # format code
cargo clippy         # lint
cargo test           # run tests
```

## Project Structure

```
hello_rust/
├── Cargo.toml
├── src/
│   └── main.rs
└── README.md
```

## Dependencies

| Crate | Purpose |
|-------|---------|
| [`reqwest`](https://crates.io/crates/reqwest) 0.11 (`blocking`, `json`) | HTTP requests to the OpenWeatherMap API |
| [`serde`](https://crates.io/crates/serde) 1.0 (`derive`) | Deserializing API responses into Rust structs |
| [`serde_json`](https://crates.io/crates/serde_json) 1.0 | JSON parsing |
| [`colored`](https://crates.io/crates/colored) 2.0 | Colored terminal output |

## Troubleshooting

- **`link.exe not found` on Windows:** install the Visual Studio C++ Build Tools and restart VS Code.
- **`edition 2024 is unstable` or similar error:** run `rustup update` to get Rust 1.85 or newer.
- **`401 Unauthorized`:** your API key is missing or invalid. New OpenWeatherMap keys can take up to a couple of hours to activate, so try again later.
- **`404 city not found`:** check the spelling or add the country code, e.g. `London,GB`.
- **`429 Too Many Requests`:** you exceeded the free plan limit (60 calls per minute). Wait a minute and retry.
- **Garbled symbols (like °) in the terminal:** use Windows Terminal, or run `chcp 65001` first.
- **Environment variable not found after `setx`:** close and reopen your terminal and VS Code.

## Credits

Weather data provided by [OpenWeatherMap](https://openweathermap.org/).