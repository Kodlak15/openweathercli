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
