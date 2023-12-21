use clap::Parser;
use dotenv::dotenv;
use reqwest::Response;
use std::{collections::HashMap, env};

#[derive(Parser, Debug)]
struct Args {}

struct Environment {
    key: String,
}

impl Environment {
    fn load() -> Self {
        dotenv().ok();

        let environment: HashMap<String, String> = env::vars().collect();

        let key = if let Some(key) = environment.get("API_KEY") {
            key.to_string()
        } else {
            panic!("Failed to load API key!");
        };

        Self { key }
    }
}

#[derive(serde::Deserialize)]
struct Coord {
    lon: Option<f32>,
    lat: Option<f32>,
}

#[derive(serde::Deserialize)]
struct Weather {
    id: Option<i32>,
    main: Option<String>,
    description: Option<String>,
    icon: Option<String>,
}

#[derive(serde::Deserialize)]
struct Main {
    temp: Option<f32>,
    feels_like: Option<f32>,
    temp_min: Option<f32>,
    temp_max: Option<f32>,
    pressure: Option<i32>,
    humidity: Option<i32>,
    sea_level: Option<i32>,
    grnd_level: Option<i32>,
}

#[derive(serde::Deserialize)]
struct Wind {
    speed: Option<f32>,
    deg: Option<i32>,
    gust: Option<f32>,
}

#[derive(serde::Deserialize)]
struct Rain {
    _1h: Option<f32>,
}

#[derive(serde::Deserialize)]
struct Clouds {
    all: Option<i32>,
}

#[derive(serde::Deserialize)]
struct Sys {
    r#type: Option<i32>,
    id: Option<i32>,
    country: Option<String>,
    sunrise: Option<i32>,
    sunset: Option<i32>,
}

#[derive(serde::Deserialize)]
struct CurrentWeather {
    coord: Option<Coord>,
    weather: Option<Weather>,
    base: Option<String>,
    main: Option<Main>,
    visibility: Option<i32>,
    wind: Option<Wind>,
    rain: Option<Rain>,
    clouds: Option<Clouds>,
    dt: Option<i32>,
    sys: Option<Sys>,
    timezone: Option<i32>,
    id: Option<i32>,
    name: Option<String>,
    cod: Option<i32>,
}

impl CurrentWeather {
    async fn get(key: String) -> Result<Self, reqwest::Error> {
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
        let weather: CurrentWeather =
            serde_json::from_str(&body).expect("Failed to deserialize response body!");

        Ok(weather)
    }
}

// this is only for current weather!
async fn get_response(environment: &Environment) -> Response {
    let req_uri = format!(
        "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
        0, 0, environment.key
    );

    let mut response_json: HashMap<&str, &str> = HashMap::new();

    reqwest::get(req_uri)
        .await
        .expect("Failed to unpack API response!")
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let environment = Environment::load();

    let weather = CurrentWeather::get(environment.key).await?;

    println!("Lat: {}", weather.coord.unwrap().lat.unwrap());

    Ok(())

    // let req_uri = format!(
    //     "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&exclude={}&appid={}",
    //     0, 0, "", environment.key
    // );
    //
    // let res = get_response(&environment).await;
    //
    // println!("Key: {}", environment.key);
    // println!("Response: {}", res.text().await.expect("???"));
}
