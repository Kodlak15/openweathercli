use std::collections::HashMap;

use crate::options::{args::Args, environment::Environment};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct GeocodingByName {
    name: Option<String>,
    local_names: Option<HashMap<String, String>>,
    lat: Option<f64>,
    lon: Option<f64>,
    country: Option<String>,
    state: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct GeocodingByZip {}

impl GeocodingByName {
    pub async fn get(args: &Args, environment: Environment) -> Result<Self, reqwest::Error> {
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

        let response = reqwest::get(req_uri).await?;

        if !response.status().is_success() {
            println!("Response failed with status code {}", response.status());
            panic!("Failed to get response, aborting!")
        }

        let body = response.text().await?;
        let data: GeocodingByName =
            serde_json::from_str(&body).expect("Failed to deserialize response body!");

        Ok(data)
    }
}

impl GeocodingByZip {
    pub async fn get(args: &Args, environment: Environment) -> Result<Self, reqwest::Error> {
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
