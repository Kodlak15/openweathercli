use dotenv::dotenv;
use std::{collections::HashMap, env};

pub fn set_workdir() {
    let mut exe_path = env::current_exe().expect("Could not unpack path to executable!");
    exe_path.pop();
    exe_path.pop();
    exe_path.pop();

    if let Err(e) = env::set_current_dir(&exe_path) {
        eprintln!("Error setting working directory: {}", e);
    }
}

pub struct Environment {
    pub key: String,
    pub units: String,
    pub lat: f64,
    pub lon: f64,
    pub city: String,
    pub state: String,
    pub country: String,
    pub zip: String,
}

impl Environment {
    pub fn load() -> Self {
        dotenv().ok();

        let environment: HashMap<String, String> = env::vars().collect();

        let key = match environment.get("API_KEY") {
            Some(key) => key.to_string(),
            None => "".to_string(),
        };

        let units = match environment.get("UNITS") {
            Some(units) => units.to_string(),
            None => "M".to_string(),
        };

        let lat = match environment.get("LATITUDE") {
            Some(lat) => lat.parse().expect("Could not parse latitude as f64!"),
            None => 0.0,
        };

        let lon = match environment.get("LONGITUDE") {
            Some(lon) => lon.parse().expect("Could not parse longitude as f64!"),
            None => 0.0,
        };

        let city = match environment.get("CITY") {
            Some(city) => city.to_string(),
            None => "".to_string(),
        };

        let state = match environment.get("STATE") {
            Some(state) => state.to_string(),
            None => "".to_string(),
        };

        let country = match environment.get("COUNTRY") {
            Some(country) => country.to_string(),
            None => "".to_string(),
        };

        let zip = match environment.get("ZIPCODE") {
            Some(zip) => zip.to_string(),
            None => "".to_string(),
        };

        Self {
            key,
            units,
            lat,
            lon,
            city,
            state,
            country,
            zip,
        }
    }
}
