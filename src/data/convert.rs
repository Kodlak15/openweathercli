pub fn to_celsius(temp: f32) -> f32 {
    temp - 273.15
}

pub fn to_fahrenheight(temp: f32) -> f32 {
    (temp * (9.0 / 5.0)) - 459.67
}

pub fn to_mph(speed: f32) -> f32 {
    speed * 2.23694
}
