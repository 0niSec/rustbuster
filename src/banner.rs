use colored::*;

// Crate imports
use crate::cli;
use textwrap::Options;

pub fn print_banner() {
    // Define the version of the tool
    let version = env!("CARGO_PKG_VERSION");

    // Print the banner
    println!(
        "{}",
        "
    ______                __   __                 __              
    |   __ \\.--.--.-----.|  |_|  |--.--.--.-----.|  |_.-----.----.
    |      <|  |  |__ --||   _|  _  |  |  |__ --||   _|  -__|   _|
    |___|__||_____|_____||____|_____|_____|_____||____|_____|__|
    "
        .truecolor(255, 79, 0)
    );

    // Print the version and GitHub link
    println!("{}{}", "rustbuster v ", version);
    println!("https://github.com/0nisec/rustbuster\n");
}

// Print a pretty block of text with the URL and wordlist
// We can just use the args struct to get the URL and wordlist
pub fn print_info(args: &cli::Args) {
    // Wrap the URL and wordlist to 80 characters
    const WRAP_WIDTH: usize = 80;
    let options = Options::new(120);

    let wrapped_url = textwrap::fill(&args.url, &options);
    let wrapped_wordlist = textwrap::fill(&args.wordlist, &options);

    // Print the info obtained from the args
    // TODO: Add more information to the banner
    println!("{}", "=".repeat(WRAP_WIDTH));
    println!("{:<width$} {}", "[+] URL:", wrapped_url, width = 30);
    println!(
        "{:<width$} {}",
        "[+] Wordlist:",
        wrapped_wordlist,
        width = 30
    );
    println!("{:<width$} {}", "[+] Threads:", args.threads, width = 30);
    println!(
        "{:<width$} {}",
        "[+] Negative Status Codes:",
        args.status_codes_blacklist.join(", "),
        width = 30
    );
    println!(
        "{:<width$} {}",
        "[+] User Agent:",
        args.user_agent,
        width = 30
    );
    println!("{:<width$} {}", "[+] Timeout:", args.timeout, width = 30);
    println!("{}", "=".repeat(WRAP_WIDTH));
}

pub fn starting_scan() {
    println!("{}", "Starting scan...".blue());
    println!("{}", "=".repeat(80));
}
