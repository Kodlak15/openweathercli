use crate::{
    data::convert::{to_celsius, to_fahrenheight, to_inches, to_mph},
    options::{
        args::Args,
        config::Config,
        options::{
            get_city, get_country, get_key, get_lat, get_lon, get_state, get_units, get_zip,
        },
    },
};
use serde::Deserialize;

use super::{data::Data, geocoding::Geocoding};

#[derive(Deserialize, Clone)]
pub struct Coord {
    pub lon: Option<f64>,
    pub lat: Option<f64>,
}

#[derive(Deserialize, Clone)]
pub struct Weather {
    pub id: Option<i32>,
    pub main: Option<String>,
    pub description: Option<String>,
    pub icon: Option<String>,
}

#[derive(Deserialize, Clone)]
pub struct Main {
    pub temp: Option<f32>,
    pub feels_like: Option<f32>,
    pub temp_min: Option<f32>,
    pub temp_max: Option<f32>,
    pub pressure: Option<i32>,
    pub humidity: Option<i32>,
    pub sea_level: Option<i32>,
    pub grnd_level: Option<i32>,
}

#[derive(Deserialize, Clone)]
pub struct Wind {
    pub speed: Option<f32>,
    pub deg: Option<i32>,
    pub gust: Option<f32>,
}

#[derive(Deserialize, Clone)]
pub struct Rain {
    pub _1h: Option<f32>,
    pub _3h: Option<f32>,
}

#[derive(Deserialize, Clone)]
pub struct Snow {
    pub _1h: Option<f32>,
    pub _3h: Option<f32>,
}

#[derive(Deserialize, Clone)]
pub struct Clouds {
    pub all: Option<i32>,
}

#[derive(Deserialize, Clone)]
pub struct Sys {
    pub r#type: Option<i32>,
    pub id: Option<i32>,
    pub country: Option<String>,
    pub sunrise: Option<i32>,
    pub sunset: Option<i32>,
}

#[derive(Deserialize, Clone)]
pub struct CurrentWeather {
    pub coord: Option<Coord>,
    pub weather: Option<Vec<Weather>>,
    pub base: Option<String>,
    pub main: Option<Main>,
    pub visibility: Option<i32>,
    pub wind: Option<Wind>,
    pub rain: Option<Rain>,
    pub snow: Option<Snow>,
    pub clouds: Option<Clouds>,
    pub dt: Option<i32>,
    pub sys: Option<Sys>,
    pub timezone: Option<i32>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub cod: Option<i32>,
}

impl CurrentWeather {
    pub async fn get(args: &Args, config: &Config) -> Result<Data, reqwest::Error> {
        let key = get_key(args, config);
        let lat = get_lat(args, config);
        let lon = get_lon(args, config);
        let city = get_city(args, config);
        let state = get_state(args, config);
        let country = get_country(args, config);
        let zip = get_zip(args, config);

        let key = match key {
            Some(key) => key,
            None => panic!("No API key found!"),
        };

        let (lat, lon) = match (lat, lon) {
            (Some(lat), Some(lon)) => (lat, lon),
            _ => {
                let geocoding = Geocoding::get(&key, city, state, country, zip).await?;

                match geocoding {
                    Some(data) => match (data.lat, data.lon) {
                        (Some(lat), Some(lon)) => (lat, lon),
                        _ => (0.0, 0.0),
                    },
                    None => (0.0, 0.0),
                }
            }
        };

        let req_uri = format!(
            "https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}",
            lat, lon, key
        );

        let response = reqwest::get(req_uri).await?;

        if !response.status().is_success() {
            println!("Response failed with status code {}", response.status());
            panic!("Failed to get response, aborting!")
        }

        let body = response.text().await?;
        let data: CurrentWeather =
            serde_json::from_str(&body).expect("Failed to deserialize response body!");

        Ok(Data::CurrentWeather(data))
    }

    pub fn print(&self, opt: &str, args: &Args, config: &Config) {
        let units = get_units(args, config);

        let units = match units {
            Some(units) => units,
            None => "M".to_string(),
        }
        .to_uppercase();

        let units = units.as_str();

        match opt {
            "lat" => println!(
                "{}",
                self.clone()
                    .coord
                    .expect("Could not unpack coordinates!")
                    .lat
                    .expect("Could not unpack latitude!")
            ),
            "lon" => println!(
                "{}",
                self.clone()
                    .coord
                    .expect("Could not unpack coordinates!")
                    .lon
                    .expect("Could not unpack longitude!")
            ),
            "id" => println!(
                "{}",
                self.clone().weather.expect("Could not unpack weather!")[0]
                    .id
                    .clone()
                    .expect("Could not unpack weather ID!")
            ),
            "weather" => println!(
                "{}",
                self.clone().weather.expect("Could not unpack weather!")[0]
                    .main
                    .clone()
                    .expect("Could not unpack main weather type!")
            ),
            "description" => println!(
                "{}",
                self.clone().weather.expect("Could not unpack weather!")[0]
                    .description
                    .clone()
                    .expect("Could not unpack weather description!")
            ),
            "icon" => println!(
                "{}",
                self.clone().weather.expect("Could not unpack weather!")[0]
                    .icon
                    .clone()
                    .expect("Could not unpack weather icon!")
            ),
            "temp" => {
                let temp = self
                    .clone()
                    .main
                    .expect("Could not unpack main!")
                    .temp
                    .expect("Could not unpack current temperature!");

                let temp = match units {
                    "M" => to_celsius(temp),
                    "I" => to_fahrenheight(temp),
                    _ => temp,
                };

                let units = match units {
                    "M" => "°C",
                    "I" => "°F",
                    _ => "°K",
                };

                println!("{:.2}{}", temp, units);
            }
            "feels_like" => {
                let feels_like = self
                    .clone()
                    .main
                    .expect("Could not unpack main!")
                    .feels_like
                    .expect("Could not unpack wind chill!");

                let feels_like = match units {
                    "M" => to_celsius(feels_like),
                    "I" => to_fahrenheight(feels_like),
                    _ => feels_like,
                };

                let units = match units {
                    "M" => "°C",
                    "I" => "°F",
                    _ => "°K",
                };

                println!("{:.2}{}", feels_like, units);
            }
            "temp_min" => {
                let temp = self
                    .clone()
                    .main
                    .expect("Could not unpack main!")
                    .temp_min
                    .expect("Could not unpack min temp!");

                let temp = match units {
                    "M" => to_celsius(temp),
                    "I" => to_fahrenheight(temp),
                    _ => temp,
                };

                let units = match units {
                    "M" => "°C",
                    "I" => "°F",
                    _ => "°K",
                };

                println!("{:.2}{}", temp, units);
            }
            "temp_max" => {
                let temp = self
                    .clone()
                    .main
                    .expect("Could not unpack main!")
                    .temp_max
                    .expect("Could not unpack max temp!");

                let temp = match units {
                    "M" => to_celsius(temp),
                    "I" => to_fahrenheight(temp),
                    _ => temp,
                };

                let units = match units {
                    "M" => "°C",
                    "I" => "°F",
                    _ => "°K",
                };

                println!("{:.2}{}", temp, units);
            }
            "pressure" => println!(
                "{}hPa",
                self.clone()
                    .main
                    .expect("Could not unpack main!")
                    .pressure
                    .expect("Could not unpack pressure!")
            ),
            "humidity" => println!(
                "{}%",
                self.clone()
                    .main
                    .expect("Could not unpack main!")
                    .humidity
                    .expect("Could not unpack humidity!")
            ),
            "visibility" => println!(
                "{}m",
                self.clone()
                    .visibility
                    .expect("Could not unpack visibility!")
            ),
            "wind_speed" => {
                let speed = self
                    .clone()
                    .wind
                    .expect("Could not unpack wind!")
                    .speed
                    .expect("Could not unpack wind speed!");

                let speed = match units {
                    "M" => speed,
                    "I" => to_mph(speed),
                    _ => speed,
                };

                let units = match units {
                    "M" => "m/s",
                    "I" => "mph",
                    _ => "m/s",
                };

                println!("{:.2}{}", speed, units);
            }
            "wind_dir" => println!(
                "{}°",
                self.clone()
                    .wind
                    .expect("Could not unpack wind!")
                    .deg
                    .expect("Could not unpack wind direction!")
            ),
            "wind_gust" => {
                let speed = self
                    .clone()
                    .wind
                    .expect("Could not unpack wind!")
                    .gust
                    .expect("Could not unpack wind gust!");

                let speed = match units {
                    "M" => speed,
                    "I" => to_mph(speed),
                    _ => speed,
                };

                let units = match units {
                    "M" => "m/s",
                    "I" => "mph",
                    _ => "m/s",
                };

                println!("{:.2}{}", speed, units);
            }
            "rain_1h" => {
                let rain = self
                    .clone()
                    .rain
                    .expect("Could not unpack rain!")
                    ._1h
                    .expect("Could not unpack 1 hour rainfall!");

                let rain = match units {
                    "M" => rain,
                    "I" => to_inches(rain),
                    _ => rain,
                };

                let units = match units {
                    "M" => "mm",
                    "I" => "in",
                    _ => "mm",
                };

                println!("{:.2}{}", rain, units);
            }
            "rain_3h" => {
                let rain = self
                    .clone()
                    .rain
                    .expect("Could not unpack rain!")
                    ._3h
                    .expect("Could not unpack 3 hour rainfall!");

                let rain = match units {
                    "M" => rain,
                    "I" => to_inches(rain),
                    _ => rain,
                };

                let units = match units {
                    "M" => "mm",
                    "I" => "in",
                    _ => "mm",
                };

                println!("{:.2}{}", rain, units);
            }
            "snow_1h" => {
                let snow = self
                    .clone()
                    .snow
                    .expect("Could not unpack snow!")
                    ._1h
                    .expect("Could not unpack 1 hour snowfall!");

                let snow = match units {
                    "M" => snow,
                    "I" => to_inches(snow),
                    _ => snow,
                };

                let units = match units {
                    "M" => "mm",
                    "I" => "in",
                    _ => "mm",
                };

                println!("{:.2}{}", snow, units);
            }
            "snow_3h" => {
                let snow = self
                    .clone()
                    .snow
                    .expect("Could not unpack snow!")
                    ._3h
                    .expect("Could not unpack 3 hour snowfall!");

                let snow = match units {
                    "M" => snow,
                    "I" => to_inches(snow),
                    _ => snow,
                };

                let units = match units {
                    "M" => "mm",
                    "I" => "in",
                    _ => "mm",
                };

                println!("{:.2}{}", snow, units);
            }
            "clouds" => println!(
                "{}%",
                self.clone()
                    .clouds
                    .expect("Could not unpack clouds!")
                    .all
                    .expect("Could not unpack all clouds!")
            ),
            _ => println!("No data to print for option {}", opt),
        };
    }
}
