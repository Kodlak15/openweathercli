use dotenv::dotenv;
use std::{collections::HashMap, env};

pub struct Environment {
    pub key: String,
    pub units: String,
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

        Self { key, units }
    }
}
