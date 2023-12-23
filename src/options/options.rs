use super::{args::Args, environment::Environment};

pub fn get_lat(args: &Args, environment: &Environment) -> Option<f64> {
    match (args.lat, environment.lat) {
        (Some(lat), _) => Some(lat),
        (_, Some(lat)) => Some(lat.parse().expect("Could not parse latitude as f64!")),
        _ => None,
    }
}

pub fn get_lon(args: &Args, environment: &Environment) -> Option<f64> {
    match (args.lat, environment.lat) {
        (Some(lon), _) => Some(lon),
        (_, Some(lon)) => Some(lon.parse().expect("Could not parse longitude as f64!")),
        _ => None,
    }
}

pub fn get_city(args: &Args, environment: &Environment) -> Option<String> {
    match (args.city, environment.city) {
        (Some(city), _) => Some(city),
        (_, Some(city)) => Some(city),
        _ => None,
    }
}

pub fn get_state(args: &Args, environment: &Environment) -> Option<String> {
    match (args.state, environment.state) {
        (Some(state), _) => Some(state),
        (_, Some(state)) => Some(state),
        _ => None,
    }
}

pub fn get_country(args: &Args, environment: &Environment) -> Option<String> {
    match (args.country, environment.country) {
        (Some(country), _) => Some(country),
        (_, Some(country)) => Some(country),
        _ => None,
    }
}

pub fn get_zip(args: &Args, environment: &Environment) -> Option<String> {
    match (args.country, environment.country) {
        (Some(zip), _) => Some(zip),
        (_, Some(zip)) => Some(zip),
        _ => None,
    }
}
