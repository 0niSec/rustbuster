use clap::Parser;

mod cli;
mod parser;


fn main() {
    // Parse the args using the Args struct from cli.rs
    let args = cli::Args::parse();

    if args.no_color {
        colored::control::set_override(false);
    }

    let url = parser::url_parser::parse_url(&args.url);
    let wordlist = parser::wordlist_parser::parse_wordlist(&args.wordlist);
    println!("URL: {:?}", url);
    println!("Wordlist: {:?}", wordlist);
}
