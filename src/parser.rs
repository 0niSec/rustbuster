use crate::cli::Args;
use clap::Parser;

pub fn parse_url() -> String {
    let args = Args::parse();
    return args.url;
}