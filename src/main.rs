use clap::Parser;
use openweathercli::{
    data::{current_weather::CurrentWeather, data::Data},
    options::{args::Args, config::Config},
};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();
    let config = Config::load();

    let api = match &args.api {
        Some(api) => api.to_owned(),
        None => "current".to_string(),
    };

    // Set up this way to make it easier to add other options later
    let data = match api.as_str() {
        "current" => CurrentWeather::get(&args, &config).await?,
        _ => CurrentWeather::get(&args, &config).await?,
    };

    if let Some(opts) = &args.print {
        opts.split(',').for_each(|opt| match &data {
            Data::CurrentWeather(dtype) => dtype.print(opt, &args, &config),
        });
    }

    Ok(())
}
