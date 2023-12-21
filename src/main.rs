use clap::Parser;
use openweathercli::{data::current_weather::CurrentWeather, options::args::Args};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    // Get the api option (current/forecast)
    let api = if let Some(api) = &args.api {
        api.to_owned()
    } else {
        "current".to_string()
    };

    // Pass to api handler
    match api.as_str() {
        "current" => CurrentWeather::get(args),
        "forecast" => todo!(),
        _ => CurrentWeather::get(args),
    }
    .await?;

    Ok(())
}
