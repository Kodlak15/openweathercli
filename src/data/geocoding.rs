use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct GeocodingByName {
    pub name: Option<String>,
    pub local_names: Option<HashMap<String, String>>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub country: Option<String>,
    pub state: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct GeocodingByZip {
    pub zip: Option<String>,
    pub name: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub country: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Geocoding {
    by_name: Option<Vec<GeocodingByName>>,
    by_zip: Option<Vec<GeocodingByZip>>,
}

impl GeocodingByName {
    pub async fn get(
        key: &String,
        city: &String,
        state: &String,
        country: &String,
    ) -> Result<Self, reqwest::Error> {
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
    pub async fn get(key: &String, country: &String, zip: &String) -> Result<Self, reqwest::Error> {
        let req_uri = format!(
            "http://api.openweathermap.org/geo/1.0/zip?zip={},{}&appid={}",
            zip, country, key
        );

        let response = reqwest::get(req_uri).await?;

        if !response.status().is_success() {
            println!("Response failed with status code {}", response.status());
            panic!("Failed to get response, aborting!")
        }

        let body = response.text().await?;
        let data: GeocodingByZip =
            serde_json::from_str(&body).expect("Failed to deserialize response body!");

        Ok(data)
    }
}

impl Geocoding {
    pub async fn get(
        key: &String,
        city: &Option<String>,
        state: &Option<String>,
        country: &Option<String>,
        zip: &Option<String>,
    ) -> Result<Self, reqwest::Error> {
        let by_name = match (&city, &state, &country) {
            (Some(city), Some(state), Some(country)) => {
                Some(GeocodingByName::get(key, city, state, country).await?)
            }
            _ => None,
        };

        let by_zip = match (&country, &zip) {
            (Some(country), Some(zip)) => Some(GeocodingByZip::get(key, country, zip).await?),
            _ => None,
        };

        let by_name = vec![by_name];
        let by_zip = vec![by_zip];

        Ok(Self { by_name, by_zip })
    }
}
