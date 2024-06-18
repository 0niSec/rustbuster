use clap::Parser;

mod cli;
mod parser;
mod banner;
mod scanner;

fn main() {
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

    let url = parser::url_parser::parse_url(&args.url);
    let wordlist = parser::wordlist_parser::parse_wordlist(&args.wordlist);
    println!("URL: {:?}", url);
    println!("Wordlist: {:?}", wordlist);
}
