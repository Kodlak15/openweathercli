use dotenv::dotenv;
use std::{collections::HashMap, env};

pub struct Environment {
    pub key: String,
}

impl Environment {
    pub fn load() -> Self {
        dotenv().ok();

        let environment: HashMap<String, String> = env::vars().collect();

        let key = if let Some(key) = environment.get("API_KEY") {
            key.to_string()
        } else {
            // Not entirely sure that I want to panic here
            panic!("Failed to load API key!");
        };

        Self { key }
    }
}
