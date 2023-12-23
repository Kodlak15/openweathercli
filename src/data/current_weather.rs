use crate::{
    data::convert::{to_celsius, to_fahrenheight, to_inches, to_mph},
    options::{
        args::Args,
        environment::Environment,
        options::{
            get_city, get_country, get_key, get_lat, get_lon, get_state, get_units, get_zip,
        },
    },
};
use serde::Deserialize;

// use super::geocoding::{self, Geocoding, GeocodingByName, GeocodingByZip};
use super::geocoding::Geocoding;

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
    pub async fn get(args: &Args, environment: &Environment) -> Result<Self, reqwest::Error> {
        let key = get_key(args, environment);
        let lat = get_lat(args, environment);
        let lon = get_lon(args, environment);
        let city = get_city(args, environment);
        let state = get_state(args, environment);
        let country = get_country(args, environment);
        let zip = get_zip(args, environment);

        let key = match key {
            Some(key) => key,
            None => panic!("No API key found!"),
        };

        // Order of precedence: latitude/longitude > geolocation by name > geolocation by zip
        // If coordinates cannot be retrieved, default to (0.0, 0.0)
        let (lat, lon) = match (lat, lon) {
            (Some(lat), Some(lon)) => (lat, lon),
            _ => {
                let geocoding = Geocoding::get(&key, &city, &state, &country, &zip).await?;

                match geocoding {
                    Some(geocoding) => match (geocoding.by_name, geocoding.by_zip) {
                        (Some(by_name), _) => match (by_name[0].lat, by_name[0].lon) {
                            (Some(lat), Some(lon)) => (lat, lon),
                            _ => (0.0, 0.0),
                        },
                        (_, Some(by_zip)) => match (by_zip[0].lat, by_zip[0].lon) {
                            (Some(lat), Some(lon)) => (lat, lon),
                            _ => (0.0, 0.0),
                        },
                        _ => (0.0, 0.0),
                    },
                    _ => (0.0, 0.0),
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

        Ok(data)
    }

    pub fn print(&self, opt: &str, args: &Args, environment: &Environment) {
        let units = get_units(args, environment);

        let units = match units {
            Some(units) => units,
            None => "M".to_string(),
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
            "id" => match args.verbose {
                true => println!(
                    "Weather ID: {}",
                    self.clone().weather.expect("Could not unpack weather!")[0]
                        .id
                        .clone()
                        .expect("Could not unpack weather ID!")
                ),
                false => println!(
                    "{}",
                    self.clone().weather.expect("Could not unpack weather!")[0]
                        .id
                        .clone()
                        .expect("Could not unpack weather ID!")
                ),
            },
            "weather" => match args.verbose {
                true => println!(
                    "Current weather: {}",
                    self.clone().weather.expect("Could not unpack weather!")[0]
                        .main
                        .clone()
                        .expect("Could not unpack main weather type!")
                ),
                false => println!(
                    "{}",
                    self.clone().weather.expect("Could not unpack weather!")[0]
                        .main
                        .clone()
                        .expect("Could not unpack main weather type!")
                ),
            },
            "description" => match args.verbose {
                true => println!(
                    "Weather description: {}",
                    self.clone().weather.expect("Could not unpack weather!")[0]
                        .description
                        .clone()
                        .expect("Could not unpack weather description!")
                ),
                false => println!(
                    "{}",
                    self.clone().weather.expect("Could not unpack weather!")[0]
                        .description
                        .clone()
                        .expect("Could not unpack weather description!")
                ),
            },
            "icon" => match args.verbose {
                true => println!(
                    "Weather Icon: {}",
                    self.clone().weather.expect("Could not unpack weather!")[0]
                        .icon
                        .clone()
                        .expect("Could not unpack weather icon!")
                ),
                false => println!(
                    "{}",
                    self.clone().weather.expect("Could not unpack weather!")[0]
                        .icon
                        .clone()
                        .expect("Could not unpack weather icon!")
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

                    let units = match units {
                        "M" => "°C",
                        "I" => "°F",
                        _ => "°K",
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

                    let units = match units {
                        "M" => "°C",
                        "I" => "°F",
                        _ => "°K",
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

                    let units = match units {
                        "M" => "°C",
                        "I" => "°F",
                        _ => "°K",
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

                    let units = match units {
                        "M" => "°C",
                        "I" => "°F",
                        _ => "°K",
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

                    let units = match units {
                        "M" => "°C",
                        "I" => "°F",
                        _ => "°K",
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

                    let units = match units {
                        "M" => "°C",
                        "I" => "°F",
                        _ => "°K",
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

                    let units = match units {
                        "M" => "°C",
                        "I" => "°F",
                        _ => "°K",
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

                    let units = match units {
                        "M" => "°C",
                        "I" => "°F",
                        _ => "°K",
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
            "visibility" => match args.verbose {
                true => println!(
                    "Visibility: {}m",
                    self.clone()
                        .visibility
                        .expect("Could not unpack visibility!")
                ),
                false => println!(
                    "{}m",
                    self.clone()
                        .visibility
                        .expect("Could not unpack visibility!")
                ),
            },
            "wind_speed" => match args.verbose {
                true => {
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

                    println!("Wind Speed: {:.2}{}", speed, units);
                }
                false => {
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
            },
            "wind_dir" => match args.verbose {
                true => println!(
                    "Wind Direction: {}°",
                    self.clone()
                        .wind
                        .expect("Could not unpack wind!")
                        .deg
                        .expect("Could not unpack wind direction!")
                ),
                false => println!(
                    "{}°",
                    self.clone()
                        .wind
                        .expect("Could not unpack wind!")
                        .deg
                        .expect("Could not unpack wind direction!")
                ),
            },
            "wind_gust" => match args.verbose {
                true => {
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

                    println!("Wind Gust: {:.2}{}", speed, units);
                }
                false => {
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
            },
            "rain_1h" => match args.verbose {
                true => {
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

                    println!("Rainfall 1hr: {:.2}{}", rain, units);
                }
                false => {
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
            },
            "rain_3h" => match args.verbose {
                true => {
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

                    println!("Rainfall 3hr: {:.2}{}", rain, units);
                }
                false => {
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
            },
            "snow_1h" => match args.verbose {
                true => {
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

                    println!("Snowfall 1hr: {:.2}{}", snow, units);
                }
                false => {
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
            },
            "snow_3h" => match args.verbose {
                true => {
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

                    println!("Snowfall 3hr: {:.2}{}", snow, units);
                }
                false => {
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
            },
            "clouds" => match args.verbose {
                true => println!(
                    "Clouds: {}%",
                    self.clone()
                        .clouds
                        .expect("Could not unpack clouds!")
                        .all
                        .expect("Could not unpack all clouds!")
                ),
                false => println!(
                    "{}%",
                    self.clone()
                        .clouds
                        .expect("Could not unpack clouds!")
                        .all
                        .expect("Could not unpack all clouds!")
                ),
            },
            _ => println!("No data to print for option {}", opt),
        };
    }
}
