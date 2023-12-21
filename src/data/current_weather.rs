use crate::{
    data::convert::{to_celsius, to_fahrenheight, to_inches, to_mph},
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
    pub weather: Option<Weather>,
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
            _ => println!("No data to print for option {}", opt),
        };
    }
}
