use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, name = "rustbuster", about = "A simple directory brute force tool written in Rust.", author="0niSec")]
pub struct Args {
    /// The target URL
    #[arg(short, long)]
    url: String,

    /// The path to the wordlist to use
    #[arg(short, long)]
    wordlist: String,

    /// The number of concurrent threads
    #[arg(short, long, default_value_t=10)]
    threads: u8,

    /// Don't display the banner and other info
    #[arg(short, long)]
    quiet: bool,
}