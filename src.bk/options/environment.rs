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
        dotenv().ok();

        let environment: HashMap<String, String> = env::vars().collect();

        let key = environment.get("API_KEY");
        let units = environment.get("UNITS");
        let lat = environment.get("LATITUDE");
        let lon = environment.get("LONGITUDE");
        let city = environment.get("CITY");
        let state = environment.get("STATE");
        let country = environment.get("COUNTRY");
        let zip = environment.get("ZIPCODE");

        Self {
            key: key.cloned(),
            units: units.cloned(),
            lat: lat.cloned(),
            lon: lon.cloned(),
            city: city.cloned(),
            state: state.cloned(),
            country: country.cloned(),
            zip: zip.cloned(),
        }
    }
}
