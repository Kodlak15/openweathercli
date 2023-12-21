use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    pub api: Option<String>,
    #[arg(long)]
    pub lat: Option<i32>,
    #[arg(long)]
    pub lon: Option<i32>,
    #[arg(long)]
    pub units: Option<String>,
    #[arg(short, long)]
    pub key: Option<String>,
    #[arg(short, long)]
    pub print: Option<String>,
    #[arg(short, long, action)]
    pub verbose: bool,
}