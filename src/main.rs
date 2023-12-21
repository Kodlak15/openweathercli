use clap::Parser;
use openweathercli::{data::current_weather::CurrentWeather, options::args::Args};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    let api = if let Some(api) = &args.api {
        api.to_owned()
    } else {
        "current".to_string()
    };

    let data = match api.as_str() {
        "current" => CurrentWeather::get(&args),
        "forecast" => todo!(),
        _ => CurrentWeather::get(&args),
    }
    .await?;

    if let Some(opts) = args.print {
        opts.split(',').for_each(|opt| {
            data.print(opt, args.verbose);
        });
    }

    Ok(())
}
