use crate::options::{args::Args, environment::Environment};

pub struct Geocoding {}

#[allow(dead_code)]
impl Geocoding {
    pub fn get_by_name(args: &Args, environment: Environment) -> Result<Self, reqwest::Error> {
        let city = match &args.city {
            Some(city) => city,
            None => environment.city,
        }

        let key = match &args.key {
            Some(key) => key,
            None => &environment.key,
        };

        let req_uri = format!(
            "http://api.openweathermap.org/geo/1.0/direct?q={},{},{}&limit={}&appid={}",
            "Seattle", "WA", "USA", 1, key
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
