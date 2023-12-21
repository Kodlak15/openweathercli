pub fn to_celsius(temp: f32) -> f32 {
    temp - 273.15
}

pub fn to_fahrenheight(temp: f32) -> f32 {
    (temp * (9.0 / 5.0)) - 459.67
}
