/// This file contains the command line arguments parser

use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version=env!("CARGO_PKG_VERSION"), 
    name = "rustbuster", 
    about = "A simple directory brute force tool written in Rust.", 
    author="0niSec",
)]
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

    /// Disable colors in the output
    #[arg(long)]
    pub no_color: bool,

    /// Status Codes allow list (these will be returned)
    #[arg(short, long)]
    pub status_codes: Vec<String>,

    /// Status codes blacklist (these will not be returned)
    #[arg(short='b', long, default_values=vec!["404"])]
    pub status_codes_blacklist: Vec<String>,

    /// Set the User-Agent string
    #[arg(short='a', long, default_value=concat!("rustbuster/", env!("CARGO_PKG_VERSION")))]
    pub user_agent: String,

    /// Follow redirects
    #[arg(short='r', long, default_value="none", value_parser=["none", "follow"])]
    pub redirect_policy: String,

    /// HTTP Timeout
    #[arg(short, long, default_value_t=10)]
    pub timeout: u64,
}