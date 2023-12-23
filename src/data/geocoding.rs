use std::collections::HashMap;

use serde::Deserialize;

async fn geocoding_by_name(
    key: &String,
    city: &String,
    state: &String,
    country: &String,
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
    println!("Body: {}", body);
    let data: Geocoding =
        serde_json::from_str(&body).expect("Failed to deserialize response body!");

    Ok(data)
}

async fn geocoding_by_zip(
    key: &String,
    country: &String,
    zip: &String,
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
    let data: Geocoding =
        serde_json::from_str(&body).expect("Failed to deserialize response body!");

    Ok(data)
}

#[derive(Deserialize, Clone)]
pub struct GeocodingData {
    pub name: Option<String>,
    pub local_names: Option<HashMap<String, String>>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Geocoding {
    pub data: Option<Vec<GeocodingData>>,
}

impl Geocoding {
    pub async fn get(
        key: &String,
        city: &Option<String>,
        state: &Option<String>,
        country: &Option<String>,
        zip: &Option<String>,
    ) -> Result<Option<GeocodingData>, reqwest::Error> {
        match (&city, &state, &country, &zip) {
            (Some(city), Some(state), Some(country), Some(zip)) => {
                let geocoding = geocoding_by_name(key, city, state, country).await?;

                match geocoding.data {
                    Some(data) => Ok(Some(data[0].to_owned())),
                    _ => {
                        let geocoding = geocoding_by_zip(key, country, zip).await?;

                        match geocoding.data {
                            Some(data) => Ok(Some(data[0].to_owned())),
                            _ => Ok(None),
                        }
                    }
                }
            }
            _ => todo!(),
        }
    }
}

// impl GeocodingByName {
//     pub async fn get(
//         key: &String,
//         city: &String,
//         state: &String,
//         country: &String,
//     ) -> Result<Geocoding, reqwest::Error> {
//         let req_uri = format!(
//             "http://api.openweathermap.org/geo/1.0/direct?q={},{},{}&limit={}&appid={}",
//             city, state, country, 1, key
//         );
//
//         let response = reqwest::get(req_uri).await?;
//
//         if !response.status().is_success() {
//             println!("Response failed with status code {}", response.status());
//             panic!("Failed to get response, aborting!")
//         }
//
//         let body = response.text().await?;
//         println!("Body: {}", body);
//         let data: Geocoding =
//             serde_json::from_str(&body).expect("Failed to deserialize response body!");
//
//         Ok(data)
//     }
// }
//
// impl GeocodingByZip {
//     pub async fn get(
//         key: &String,
//         country: &String,
//         zip: &String,
//     ) -> Result<Geocoding, reqwest::Error> {
//         let req_uri = format!(
//             "http://api.openweathermap.org/geo/1.0/zip?zip={},{}&appid={}",
//             zip, country, key
//         );
//
//         let response = reqwest::get(req_uri).await?;
//
//         if !response.status().is_success() {
//             println!("Response failed with status code {}", response.status());
//             panic!("Failed to get response, aborting!")
//         }
//
//         let body = response.text().await?;
//         let data: Geocoding =
//             serde_json::from_str(&body).expect("Failed to deserialize response body!");
//
//         Ok(data)
//     }
// }
//
// impl Geocoding {
//     pub async fn get(
//         key: &String,
//         city: &Option<String>,
//         state: &Option<String>,
//         country: &Option<String>,
//         zip: &Option<String>,
//     ) -> Result<Option<Self>, reqwest::Error> {
//         let geocoding = match (&city, &state, &country, &zip) {
//             (Some(city), Some(state), Some(country), _) => {
//                 Some(GeocodingByName::get(key, city, state, country).await?)
//             }
//             (_, _, Some(country), Some(zip)) => Some(GeocodingByZip::get(key, country, zip).await?),
//             _ => None,
//         };
//
//         Ok(geocoding)
//     }
// }
