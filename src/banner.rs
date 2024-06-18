use colored::*;

// Crate imports
use crate::cli;
use textwrap::Options;

pub fn print_banner() {
    // Define the version of the tool
    let version = env!("CARGO_PKG_VERSION");

    // Print the banner
    println!("{}", "
    ______                __   __                 __              
    |   __ \\.--.--.-----.|  |_|  |--.--.--.-----.|  |_.-----.----.
    |      <|  |  |__ --||   _|  _  |  |  |__ --||   _|  -__|   _|
    |___|__||_____|_____||____|_____|_____|_____||____|_____|__|
    ".truecolor(255, 79, 0));

    // Print the version and GitHub link
    println!("{}{}", "rustbuster v ", version);
    println!("https://github.com/0nisec/rustbuster\n");
}

// Print a pretty block of text with the URL and wordlist
// We can just use the args struct to get the URL and wordlist
// If we get to printing the banner and everything, we can assume that the URL and wordlist are valid
pub fn print_info(args: &cli::Args) {
    // Wrap the URL and wordlist to 80 characters
    const WRAP_WIDTH: usize = 80;
    let options = Options::new(80);

    let wrapped_url = textwrap::fill(&args.url, &options);
    let wrapped_wordlist = textwrap::fill(&args.wordlist, &options);

    // Print the info obtained from the args
    // TODO: Add more information to the banner
    println!("{}", "=".repeat(WRAP_WIDTH));
    println!("{:<width$} {}", "[+] URL:", wrapped_url, width = 14);
    println!("{:<width$} {}", "[+] Wordlist:", wrapped_wordlist, width = 14);
    println!("{}", "=".repeat(WRAP_WIDTH));
}

// ? If I ever change my mind, I can always revert back to the old print_info function
// pub fn print_info(args: &cli::Args) {
//     const WIDTH: usize = 14;

//     println!("{}", "=".repeat(80));
//     println!("{:<width$} {}", "[+] URL:", args.url, width = WIDTH);
//     println!("{:<width$} {}", "[+] Wordlist:", args.wordlist, width = WIDTH);
//     println!("{}", "=".repeat(80));
// }