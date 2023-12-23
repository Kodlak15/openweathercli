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
