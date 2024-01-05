use serde::Deserialize;

use crate::{
    data::data::Data,
    options::{
        args::Args,
        environment::Environment,
        options::{get_city, get_country, get_key, get_lat, get_lon, get_state, get_zip},
    },
};

use super::geocoding::Geocoding;

#[derive(Deserialize, Clone)]
pub struct Main {
    temp: Option<f32>,
    feels_like: Option<f32>,
    temp_min: Option<f32>,
    temp_max: Option<f32>,
    pressure: Option<i32>,
    sea_level: Option<i32>,
    grnd_level: Option<i32>,
    humidity: Option<i32>,
    temp_kf: Option<i32>,
}

#[derive(Deserialize, Clone)]
pub struct Weather {
    id: Option<i32>,
    main: Option<String>,
    description: Option<String>,
    icon: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Clouds {
    all: Option<i8>,
}

#[derive(Deserialize, Clone)]
pub struct Wind {
    speed: Option<f32>,
    deg: Option<i32>,
    gust: Option<f32>,
}

#[derive(Deserialize, Clone)]
pub struct Rain {
    _1h: Option<f32>,
    _3h: Option<f32>,
}

#[derive(Deserialize, Clone)]
pub struct Snow {
    _1h: Option<f32>,
    _3h: Option<f32>,
}

#[derive(Deserialize, Clone)]
pub struct Sys {
    pod: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Day {
    dt: Option<String>,
    main: Option<Main>,
    weather: Vec<Weather>,
    clouds: Option<Clouds>,
    wind: Option<Wind>,
    visibility: Option<i32>,
    pop: Option<f32>,
    rain: Option<Rain>,
    snow: Option<Snow>,
    sys: Option<Sys>,
    dt_txt: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct FiveDayForecast {
    cod: Option<String>,
    message: Option<i32>,
    cnt: Option<i32>,
    list: Option<Vec<Day>>,
}

impl FiveDayForecast {
    pub async fn get(args: &Args, environment: &Environment) -> Result<Data, reqwest::Error> {
        let key = get_key(args, environment);
        let lat = get_lat(args, environment);
        let lon = get_lon(args, environment);
        let city = get_city(args, environment);
        let state = get_state(args, environment);
        let country = get_country(args, environment);
        let zip = get_zip(args, environment);

        let key = match key {
            Some(key) => key,
            None => panic!("No API key found!"),
        };

        let (lat, lon) = match (lat, lon) {
            (Some(lat), Some(lon)) => (lat, lon),
            _ => {
                let geocoding = Geocoding::get(&key, &city, &state, &country, &zip).await?;

                match geocoding {
                    Some(data) => match (data.lat, data.lon) {
                        (Some(lat), Some(lon)) => (lat, lon),
                        _ => (0.0, 0.0),
                    },
                    None => (0.0, 0.0),
                }
            }
        };

        let req_uri = format!(
            "https://api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&appid={}",
            lat, lon, key
        );

        let response = reqwest::get(req_uri).await?;

        if !response.status().is_success() {
            println!("Response failed with status code {}", response.status());
            panic!("Failed to get response, aborting!")
        }

        let body = response.text().await?;
        let data: FiveDayForecast =
            serde_json::from_str(&body).expect("Failed to deserialize response body!");

        println!("Body: {:?}", body);

        Ok(Data::FiveDayForecast(data))
    }

    pub fn print(&self, _opt: &str, _args: &Args, _environment: &Environment) {}
}
