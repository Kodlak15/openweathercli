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

struct Coord {
    lon: Option<String>,
    lat: Option<String>,
}

struct Weather {
    id: Option<String>,
    main: Option<String>,
    description: Option<String>,
    icon: Option<String>,
}

struct Main {}

struct CurrentWeather {
    coord: Option<Coord>,
    weather: Option<Weather>,
    base: Option<String>,
    main: Option<Main>,
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
async fn main() {
    let environment = Environment::load();

    let req_uri = format!(
        "https://api.openweathermap.org/data/3.0/onecall?lat={}&lon={}&exclude={}&appid={}",
        0, 0, "", environment.key
    );

    let res = get_response(&environment).await;

    println!("Key: {}", environment.key);
    println!("Response: {}", res.text().await.expect("???"));
}
