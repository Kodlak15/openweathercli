use super::{args::Args, environment::Environment};

pub fn get_key(args: &Args, environment: &Environment) -> Option<String> {
    match (&args.key, &environment.key) {
        (Some(key), _) => Some(key.to_string()),
        (_, Some(key)) => Some(key.to_string()),
        _ => None,
    }
}

pub fn get_lat(args: &Args, environment: &Environment) -> Option<f64> {
    match (&args.lat, &environment.lat) {
        (Some(lat), _) => Some(*lat),
        (_, Some(lat)) => Some(lat.parse().expect("Could not parse latitude as f64!")),
        _ => None,
    }
}

pub fn get_lon(args: &Args, environment: &Environment) -> Option<f64> {
    match (&args.lat, &environment.lat) {
        (Some(lon), _) => Some(*lon),
        (_, Some(lon)) => Some(lon.parse().expect("Could not parse longitude as f64!")),
        _ => None,
    }
}

pub fn get_city(args: &Args, environment: &Environment) -> Option<String> {
    match (&args.city, &environment.city) {
        (Some(city), _) => Some(city.to_string()),
        (_, Some(city)) => Some(city.to_string()),
        _ => None,
    }
}

pub fn get_state(args: &Args, environment: &Environment) -> Option<String> {
    match (&args.state, &environment.state) {
        (Some(state), _) => Some(state.to_string()),
        (_, Some(state)) => Some(state.to_string()),
        _ => None,
    }
}

pub fn get_country(args: &Args, environment: &Environment) -> Option<String> {
    match (&args.country, &environment.country) {
        (Some(country), _) => Some(country.to_string()),
        (_, Some(country)) => Some(country.to_string()),
        _ => None,
    }
}

pub fn get_zip(args: &Args, environment: &Environment) -> Option<String> {
    match (&args.zip, &environment.zip) {
        (Some(zip), _) => Some(zip.to_string()),
        (_, Some(zip)) => Some(zip.to_string()),
        _ => None,
    }
}

pub fn get_units(args: &Args, environment: &Environment) -> Option<String> {
    match (&args.units, &environment.units) {
        (Some(units), _) => Some(units.to_string()),
        (_, Some(units)) => Some(units.to_string()),
        _ => None,
    }
}
