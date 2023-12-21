use crate::options::{args::Args, environment::Environment};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Coord {
    pub lon: Option<f32>,
    pub lat: Option<f32>,
}

#[derive(Deserialize)]
pub struct Weather {
    pub weather: WeatherData,
}

#[derive(Deserialize)]
pub struct WeatherData {
    pub id: Option<i32>,
    pub main: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Deserialize)]
pub struct Main {
    pub temp: Option<f32>,
    pub feels_like: Option<f32>,
    pub temp_min: Option<f32>,
    pub temp_max: Option<f32>,
    pub pressure: Option<i32>,
    pub humidity: Option<i32>,
    pub sea_level: Option<i32>,
    pub grnd_level: Option<i32>,
}

#[derive(Deserialize)]
pub struct Wind {
    pub speed: Option<f32>,
    pub deg: Option<i32>,
    pub gust: Option<f32>,
}

#[derive(Deserialize)]
pub struct Rain {
    pub _1h: Option<f32>,
}

#[derive(Deserialize)]
pub struct Clouds {
    pub all: Option<i32>,
}

#[derive(Deserialize)]
pub struct Sys {
    pub r#type: Option<i32>,
    pub id: Option<i32>,
    pub country: Option<String>,
    pub sunrise: Option<i32>,
    pub sunset: Option<i32>,
}

#[derive(Deserialize)]
pub struct CurrentWeather {
    pub coord: Option<Coord>,
    pub weather: Option<Weather>,
    pub base: Option<String>,
    pub main: Option<Main>,
    pub visibility: Option<i32>,
    pub wind: Option<Wind>,
    pub rain: Option<Rain>,
    pub clouds: Option<Clouds>,
    pub dt: Option<i32>,
    pub sys: Option<Sys>,
    pub timezone: Option<i32>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub cod: Option<i32>,
}

impl CurrentWeather {
    pub async fn get(args: Args) -> Result<Self, reqwest::Error> {
        let environment = Environment::load();

        let lat = match args.lat {
            Some(lat) => lat,
            None => 0,
        };

        let lon = match args.lon {
            Some(lon) => lon,
            None => 0,
        };

        let key = match args.key {
            Some(key) => key,
            None => environment.key,
        };

        let req_uri = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
            0, 0, key
        );

        let response = reqwest::get(req_uri).await?;

        if !response.status().is_success() {
            println!("Response failed with status code {}", response.status());
            panic!("Failed to get response, aborting!")
        }

        let body = response.text().await?;
        let data: CurrentWeather =
            serde_json::from_str(&body).expect("Failed to deserialize response body!");

        println!("Latitude: {}", data.coord.as_ref().unwrap().lat.unwrap());
        println!("Longitude: {}", data.coord.as_ref().unwrap().lon.unwrap());

        Ok(data)
    }
}
