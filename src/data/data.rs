use super::{current_weather::CurrentWeather, five_day_forecast::FiveDayForecast};

pub enum Data {
    CurrentWeather(CurrentWeather),
    FiveDayForecast(FiveDayForecast),
}
