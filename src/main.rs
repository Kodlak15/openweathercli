use clap::Parser;
use dotenv::dotenv;
use std::{collections::HashMap, env};

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub api: Option<String>,
    #[arg(long, allow_hyphen_values(true))]
    pub lat: Option<f64>,
    #[arg(long, allow_hyphen_values(true))]
    pub lon: Option<f64>,
    #[arg(long)]
    pub city: Option<String>,
    #[arg(long)]
    pub state: Option<String>,
    #[arg(long)]
    pub country: Option<String>,
    #[arg(long)]
    pub zip: Option<String>,
    #[arg(long)]
    pub units: Option<String>,
    #[arg(short, long)]
    pub key: Option<String>,
    #[arg(short, long)]
    pub print: Option<String>, // Print user specified information
    #[arg(short, long)]
    pub summary: Option<String>, // Print general summary of data
    #[arg(short, long, action)]
    pub verbose: bool,
}

pub struct Environment {
    pub key: Option<String>,
    pub units: Option<String>,
    pub lat: Option<String>,
    pub lon: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub zip: Option<String>,
}

impl Environment {
    pub fn load() -> Self {
        // Load environment variables from .env
        dotenv().ok();
        let environment: HashMap<String, String> = env::vars().collect();

        Self {
            key: environment.get("API_KEY").cloned(),
            units: environment.get("UNITS").cloned(),
            lat: environment.get("LATITUDE").cloned(),
            lon: environment.get("LONGITUDE").cloned(),
            city: environment.get("CITY").cloned(),
            state: environment.get("STATE").cloned(),
            country: environment.get("COUNTRY").cloned(),
            zip: environment.get("ZIPCODE").cloned(),
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    println!("{:?}", env::current_dir().unwrap())
}
