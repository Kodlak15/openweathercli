use crate::{
    data::convert::{to_celsius, to_fahrenheight},
    options::{args::Args, environment::Environment},
};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Coord {
    pub lon: Option<f32>,
    pub lat: Option<f32>,
}

#[derive(Deserialize, Clone)]
pub struct Weather {
    pub weather: Option<WeatherData>,
}

#[derive(Deserialize, Clone)]
pub struct WeatherData {
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
    pub weather: Option<Weather>,
    pub base: Option<String>,
    pub main: Option<Main>,
    pub visibility: Option<i32>,
    pub wind: Option<Wind>,
    pub rain: Option<Rain>,
    pub clouds: Option<Clouds>,
    pub dt: Option<i32>,
    pub sys: Option<Sys>,
    pub timezone: Option<i32>,
    pub id: Option<i32>,
    pub name: Option<String>,
    pub cod: Option<i32>,
}

impl CurrentWeather {
    pub async fn get(args: &Args, environment: &Environment) -> Result<Self, reqwest::Error> {
        let lat = match args.lat {
            Some(lat) => lat,
            None => 0,
        };

        let lon = match args.lon {
            Some(lon) => lon,
            None => 0,
        };

        let key = match &args.key {
            Some(key) => key,
            None => &environment.key,
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

        Ok(data)
    }

    pub fn print(&self, opt: &str, args: &Args, environment: &Environment) {
        let units = match &args.units {
            Some(units) => units,
            None => &environment.units,
        }
        .to_uppercase();

        let units = units.as_str();

        match opt {
            "lat" => match args.verbose {
                true => println!(
                    "Latitude: {}",
                    self.clone()
                        .coord
                        .expect("Could not unpack coordinates!")
                        .lat
                        .expect("Could not unpack latitude!")
                ),
                false => println!(
                    "{}",
                    self.clone()
                        .coord
                        .expect("Could not unpack coordinates!")
                        .lat
                        .expect("Could not unpack latitude!")
                ),
            },
            "lon" => match args.verbose {
                true => println!(
                    "Longitude: {}",
                    self.clone()
                        .coord
                        .expect("Could not unpack coordinates!")
                        .lon
                        .expect("Could not unpack longitude!")
                ),
                false => println!(
                    "{}",
                    self.clone()
                        .coord
                        .expect("Could not unpack coordinates!")
                        .lon
                        .expect("Could not unpack longitude!")
                ),
            },
            "weather" => match args.verbose {
                true => println!(
                    "Current weather: {}",
                    self.clone()
                        .weather
                        .expect("Could not unpack weather!")
                        .weather
                        .expect("Could not unpack weather data!")
                        .main
                        .expect("Could not unpack main weather type!")
                ),
                false => println!(
                    "{}",
                    self.clone()
                        .weather
                        .expect("Could not unpack weather!")
                        .weather
                        .expect("Could not unpack weather data!")
                        .main
                        .expect("Could not unpack main weather type!")
                ),
            },
            "description" => match args.verbose {
                true => println!(
                    "Weather description: {}",
                    self.clone()
                        .weather
                        .expect("Could not unpack weather!")
                        .weather
                        .expect("Could not unpack weather data!")
                        .description
                        .expect("Could not unpack weather description!")
                ),
                false => println!(
                    "{}",
                    self.clone()
                        .weather
                        .expect("Could not unpack weather!")
                        .weather
                        .expect("Could not unpack weather data!")
                        .description
                        .expect("Could not unpack weather description!")
                ),
            },
            "temp" => match args.verbose {
                true => {
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

                    println!("Current Temperature: {:.2}{}", temp, units);
                }
                false => {
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

                    println!("{:.2}{}", temp, units);
                }
            },
            "feels_like" => match args.verbose {
                true => {
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

                    println!("Wind Chill: {:.2}{}", feels_like, units);
                }
                false => {
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

                    println!("{:.2}{}", feels_like, units);
                }
            },
            "temp_min" => match args.verbose {
                true => {
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

                    println!("Low Temperature: {:.2}{}", temp, units);
                }
                false => {
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

                    println!("{:.2}{}", temp, units);
                }
            },
            "temp_max" => match args.verbose {
                true => {
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

                    println!("High Temperature: {:.2}{}", temp, units);
                }
                false => {
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

                    println!("{:.2}{}", temp, units);
                }
            },
            "pressure" => match args.verbose {
                true => println!(
                    "Pressure: {}hPa",
                    self.clone()
                        .main
                        .expect("Could not unpack main!")
                        .pressure
                        .expect("Could not unpack pressure!")
                ),
                false => println!(
                    "{}hPa",
                    self.clone()
                        .main
                        .expect("Could not unpack main!")
                        .pressure
                        .expect("Could not unpack pressure!")
                ),
            },
            "humidity" => match args.verbose {
                true => println!(
                    "Humidity: {}%",
                    self.clone()
                        .main
                        .expect("Could not unpack main!")
                        .humidity
                        .expect("Could not unpack humidity!")
                ),
                false => println!(
                    "{}%",
                    self.clone()
                        .main
                        .expect("Could not unpack main!")
                        .humidity
                        .expect("Could not unpack humidity!")
                ),
            },
            _ => println!("No data to print for option {}", opt),
        };
    }
}
