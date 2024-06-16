use std::fs::File;
use std::path::Path;
use colored::*;

// Function to check that the wordlist is a valid path and return the PathBuf
pub fn parse_wordlist(wordlist: &str) -> std::path::PathBuf {
    let path: &Path = Path::new(wordlist);

    // If the path is valid, we try to open the file
    // If the file cannot be opened, we print an error message and exit the program
    if path.exists() {
        match File::open(&path) {
            Ok(_) => path.to_path_buf(),
            Err(e) => {
                eprintln!("{}", format!("Error: The wordlist file cannot be read: {}", e).red());
                std::process::exit(1);
            }
        }
    } else {
        eprintln!("{}", format!("Error: The wordlist path is invalid.").red());
        std::process::exit(1);
    }

    
}