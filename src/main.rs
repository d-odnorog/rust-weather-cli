use clap::Parser;
use dotenv;
use serde::Deserialize;

const LAT: f32 = 55.75;
const LON: f32 = 37.77;

#[derive(Parser)]
#[command(name = "forecast")]
#[command(about = "Weather in your terminal", long_about = None)]
struct Args {
    /// Number of days for the forecast
    #[arg(short, default_value_t = 0)]
    days: u8,
}

#[derive(Deserialize, Debug)]
struct Coord {
    lat: f32,
    lon: f32,
}

#[derive(Deserialize, Debug)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Deserialize, Debug)]
struct CurrentWeatherMain {
    temp: f32,
    feels_like: f32,
}

#[derive(Deserialize, Debug)]
struct CurrentWeather {
    coord: Coord,
    weather: Vec<Weather>,
    base: String,
    main: CurrentWeatherMain,
}

fn main() -> Result<(), reqwest::Error> {
    dotenv::dotenv().unwrap(); // !!

    let mut api_key: Option<String> = None;
    for (key, value) in std::env::vars() {
        if key != "APIKEY" {
            continue;
        }
        api_key = Some(value);
    }
    if api_key.is_none() {
        panic!("Need API key");
    }
    let api_key: String = api_key.unwrap();

    let args = Args::parse();

    let method: &str = match args.days {
        0 => "weather",
        _ => "forecast",
    };
    let cnt: u8 = args.days * 8;

    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/{method}?lat={LAT}&lon={LON}&appid={api_key}&units=metric&cnt={cnt}"
    );
    let weather: CurrentWeather = reqwest::blocking::get(url)?.json()?;

    println!("Main: {:?}", weather.weather[0].main);
    println!("Description: {:?}", weather.weather[0].description);
    println!("Temperature: {:?}", weather.main.temp);
    println!("Feels like: {:?}", weather.main.feels_like);

    Ok(())
}
