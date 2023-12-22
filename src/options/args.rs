use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub api: Option<String>,
    #[arg(long, allow_hyphen_values(true))]
    pub lat: Option<f64>,
    #[arg(long, allow_hyphen_values(true))]
    pub lon: Option<f64>,
    #[arg(long)]
    pub units: Option<String>,
    #[arg(short, long)]
    pub key: Option<String>,
    #[arg(short, long)]
    pub print: Option<String>, // Print user specified information
    #[arg(short, long)]
    pub summary: Option<String>, // Print general summary of data
    #[arg(short, long, action)]
    pub verbose: bool,
}
