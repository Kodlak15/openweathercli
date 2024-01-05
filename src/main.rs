use clap::Parser;
use openweathercli::{
    data::{current_weather::CurrentWeather, data::Data, five_day_forecast::FiveDayForecast},
    options::{
        args::Args,
        environment::{set_workdir, Environment},
    },
};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    set_workdir();

    let args = Args::parse();
    let environment = Environment::load();

    let api = if let Some(api) = &args.api {
        api.to_owned()
    } else {
        "current".to_string()
    };

    let data = match api.as_str() {
        "current" => CurrentWeather::get(&args, &environment).await?,
        "forecast" => FiveDayForecast::get(&args, &environment).await?,
        _ => CurrentWeather::get(&args, &environment).await?,
    };

    if let Some(opts) = &args.print {
        opts.split(',').for_each(|opt| match &data {
            Data::CurrentWeather(dtype) => dtype.print(opt, &args, &environment),
            Data::FiveDayForecast(dtype) => dtype.print(opt, &args, &environment),
        });
    }

    Ok(())
}
