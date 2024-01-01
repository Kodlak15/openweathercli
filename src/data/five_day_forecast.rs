use serde::Deserialize;

use crate::options::{
    args::Args,
    environment::Environment,
    options::{get_city, get_country, get_key, get_lat, get_lon, get_state, get_zip},
};

use super::geocoding::Geocoding;

#[derive(Deserialize, Clone)]
pub struct FiveDayForecast {}

impl FiveDayForecast {
    pub async fn get(args: &Args, environment: &Environment) -> Result<Self, reqwest::Error> {
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
            "api.openweathermap.org/data/2.5/forecast?lat={}&lon={}&appid={}",
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

        Ok(data)
    }
}
