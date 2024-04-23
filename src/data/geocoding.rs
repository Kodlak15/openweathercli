use std::collections::HashMap;

use serde::Deserialize;

async fn geocoding_by_name(
    key: &str,
    city: &str,
    state: &str,
    country: &str,
) -> Result<Geocoding, reqwest::Error> {
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
    let geocoding: Geocoding =
        serde_json::from_str(&body).expect("Failed to deserialize response body!");

    Ok(geocoding)
}

async fn geocoding_by_zip(
    key: &str,
    country: &str,
    zip: &str,
) -> Result<Geocoding, reqwest::Error> {
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
    let value: serde_json::Value = serde_json::from_str(&body).unwrap();
    let geocoding_data: GeocodingData = serde_json::from_value(value).unwrap();
    let geocoding = Geocoding {
        data: Some(geocoding_data),
    };

    Ok(geocoding)
}

#[derive(Deserialize, Clone, Debug)]
pub struct GeocodingData {
    pub name: Option<String>,
    pub local_names: Option<HashMap<String, String>>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Geocoding {
    pub data: Option<GeocodingData>,
}

impl Geocoding {
    pub async fn get(
        key: &str,
        city: Option<String>,
        state: Option<String>,
        country: Option<String>,
        zip: Option<String>,
    ) -> Result<Option<GeocodingData>, reqwest::Error> {
        match (&city, &state, &country, &zip) {
            (Some(city), Some(state), Some(country), Some(zip)) => {
                let geocoding = geocoding_by_name(key, city, state, country).await?;

                match geocoding.data {
                    Some(data) => Ok(Some(data.to_owned())),
                    _ => {
                        let geocoding = geocoding_by_zip(key, country, zip).await?;

                        match geocoding.data {
                            Some(data) => Ok(Some(data.to_owned())),
                            _ => Ok(None),
                        }
                    }
                }
            }
            (Some(city), Some(state), Some(country), None) => {
                let geocoding = geocoding_by_name(key, city, state, country).await?;

                match geocoding.data {
                    Some(data) => Ok(Some(data.to_owned())),
                    _ => Ok(None),
                }
            }
            (_, _, Some(country), Some(zip)) => {
                let geocoding = geocoding_by_zip(key, country, zip).await?;

                match geocoding.data {
                    Some(data) => Ok(Some(data.to_owned())),
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    }
}
