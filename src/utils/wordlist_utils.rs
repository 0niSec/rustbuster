use std::fs::File;
use std::path::Path;
use colored::*;

// Function to check that the wordlist is a valid path and return the PathBuf
pub fn validate_wordlist_path(wordlist: &str) -> std::path::PathBuf {
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

// Function to check if the wordlist file is empty
pub fn is_empty(wordlist: &str) -> bool {
    let path: &Path = Path::new(wordlist);

    match File::open(&path) {
        Ok(file) => {
            // Check if the file is empty
            match file.metadata() {
                Ok(metadata) if metadata.len() > 0 => false,
                Ok(_) => {
                    eprintln!("{}", format!("Error: The wordlist file is empty.").red());
                    std::process::exit(1);
                },
                Err(e) => {
                    eprintln!("{}", format!("Error: Unable to read the wordlist file metadata: {}", e).red());
                    std::process::exit(1);
                }
            }
        },
        Err(e) => {
            eprintln!("{}", format!("Error: The wordlist file cannot be read: {}", e).red());
            std::process::exit(1);
        }
    }
}

// Function to read the wordlist file and split it into a vector of strings
pub fn read_wordlist(wordlist: &std::path::PathBuf) -> Vec<String> {
    let contents = std::fs::read_to_string(wordlist);
    match contents {
        Ok(contents) => {
            let lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();
            lines
        },
        Err(e) => {
            eprintln!("{}", format!("Error: {}", e).red());
            std::process::exit(1);
        }
    }
}