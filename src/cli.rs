/// This file contains the command line arguments parser

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, name = "rustbuster", about = "A simple directory brute force tool written in Rust.", author="0niSec")]
pub struct Args {
    /// The target URL
    #[arg(short, long)]
    pub url: String,

    /// The path to the wordlist to use
    #[arg(short, long)]
    pub wordlist: String,

    /// The number of concurrent threads
    #[arg(short, long, default_value_t=10)]
    pub threads: u8,

    /// Don't display the banner and other info
    #[arg(short, long)]
    pub quiet: bool,
}