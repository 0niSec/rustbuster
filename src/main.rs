use clap::Parser;
use tokio::sync::mpsc;

mod banner;
mod buster;
mod cli;
mod progress;
mod utils;

#[tokio::main]
async fn main() {
    // Parse the args using the Args struct from cli.rs
    let args = cli::Args::parse();

    // If the user wants to disable colors, we set the override to false
    if args.no_color {
        colored::control::set_override(false);
    }

    // If the user doesn't want to display the banner, we don't print it
    if !args.quiet {
        banner::print_banner();
        banner::print_info(&args);
    }

    // Create a new scanner instance
    let scanner = buster::Scanner::new(args);

    // Start the scan
    banner::starting_scan();
    scanner.scan().await.unwrap();
}
