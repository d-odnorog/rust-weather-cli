use clap::Parser;
use dotenv;
use serde_json::Value;

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
    let weather: Value = reqwest::blocking::get(url)?.json()?;

    println!("Place: {:?}", weather["name"].as_str().unwrap_or_default());
    println!(
        "Main: {:?}",
        weather["weather"][0]["main"].as_str().unwrap_or_default()
    );
    println!(
        "Description: {:?}",
        weather["weather"][0]["description"]
            .as_str()
            .unwrap_or_default()
    );
    println!(
        "Temperature: {:?}",
        weather["main"]["temp"].as_f64().unwrap_or_default()
    );
    println!(
        "Feels like: {:?}",
        weather["main"]["feels_like"].as_f64().unwrap_or_default()
    );

    Ok(())
}
