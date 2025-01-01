use clap::Parser;
use std::path::PathBuf;

use super::parse_path::parse_path;
#[derive(Parser, Debug)]
#[command(term_width = 0)]
pub struct Args {
    /// The path to the directory to process
    #[arg(short = 's', long = "source", value_name = "DIR", value_parser = parse_path, default_value = "./src")]
    pub source: PathBuf,
}
