use crate::arguments::parse_path::parse_path;
use clap::Parser;
use std::path::PathBuf;
#[derive(Parser, Debug)]
#[command(term_width = 0)]
pub struct Args {
    #[arg(short = 's', long = "source", value_name = "DIR", value_parser = parse_path, default_value = "./src")]
    pub source: PathBuf,
}
