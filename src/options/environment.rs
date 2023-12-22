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
}

impl Environment {
    pub fn load() -> Self {
        dotenv().ok();

        let environment: HashMap<String, String> = env::vars().collect();

        let key = if let Some(key) = environment.get("API_KEY") {
            key.to_string()
        } else {
            "".to_string()
        };

        let units = if let Some(units) = environment.get("UNITS") {
            units.to_string()
        } else {
            "".to_string()
        };

        let lat = if let Some(lat) = environment.get("LATITUDE") {
            lat.parse().expect("Could not parse latitude as f32!")
        } else {
            0.0
        };

        let lon = if let Some(lon) = environment.get("LONGITUDE") {
            lon.parse().expect("Could not parse longitude as f32!")
        } else {
            0.0
        };

        Self {
            key,
            units,
            lat,
            lon,
        }
    }
}
