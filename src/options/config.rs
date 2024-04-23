use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub key: Option<String>,
    pub units: Option<String>,
    pub lat: Option<String>,
    pub lon: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub zip: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key: None,
            units: None,
            lat: None,
            lon: None,
            city: None,
            state: None,
            country: None,
            zip: None,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let mut config_path = dirs::config_dir().unwrap_or_default();
        config_path.push("owcli/config.yaml");

        match std::fs::File::open(&config_path) {
            Ok(f) => match serde_yaml::from_reader(f) {
                Ok(config) => config,
                Err(e2) => {
                    eprintln!("Unable to deserialize the configuration file: {}", e2);
                    Self::default()
                }
            },
            Err(e1) => {
                eprintln!("Unable to open the configuration file: {}", e1);
                Self::default()
            }
        }
    }
}
