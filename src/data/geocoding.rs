use crate::options::{args::Args, environment::Environment};

pub struct Geocoding {}

#[allow(dead_code)]
impl Geocoding {
    pub fn get_by_name(args: &Args, environment: Environment) -> Result<Self, reqwest::Error> {
        let city = match &args.city {
            Some(city) => city,
            None => &environment.city,
        };

        let state = match &args.state {
            Some(state) => state,
            None => &environment.state,
        };

        let country = match &args.country {
            Some(country) => country,
            None => &environment.country,
        };

        let key = match &args.key {
            Some(key) => key,
            None => &environment.key,
        };

        let req_uri = format!(
            "http://api.openweathermap.org/geo/1.0/direct?q={},{},{}&limit={}&appid={}",
            city, state, country, 1, key
        );

        Ok(Self {})
    }

    pub fn get_by_zip(args: &Args, environment: Environment) -> Result<Self, reqwest::Error> {
        let key = match &args.key {
            Some(key) => key,
            None => &environment.key,
        };

        let req_uri = format!(
            "http://api.openweathermap.org/geo/1.0/zip?zip={},{}&appid={}",
            "98075", "USA", key
        );

        Ok(Self {})
    }
}
