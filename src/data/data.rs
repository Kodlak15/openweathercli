use super::current_weather::CurrentWeather;

pub enum Data {
    CurrentWeather(CurrentWeather),
}
