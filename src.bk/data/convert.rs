// Kelvin -> Celsius
pub fn to_celsius(temp: f32) -> f32 {
    temp - 273.15
}

// Kelvin -> Fahrenheight
pub fn to_fahrenheight(temp: f32) -> f32 {
    (temp * (9.0 / 5.0)) - 459.67
}

// m/s -> mph
pub fn to_mph(speed: f32) -> f32 {
    speed * 2.23694
}

// mm -> in
pub fn to_inches(precipitation: f32) -> f32 {
    precipitation / 25.4
}
