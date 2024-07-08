use colored::*;
use url::Url;

// Function to parse the String URL into a URL struct so it can be used later for reqwest
pub fn validate_url(url: &str) -> Result<Url, url::ParseError> {
    // This supports IPv4 addresses as well out of the box
    let parsed_url: Result<Url, url::ParseError> = Url::parse(&url);
    match parsed_url {
        Ok(url) => Ok(url),
        Err(e) => {
            eprintln!("{}", format!("Error: {}", e).red());
            std::process::exit(1);
        }
    }
}

// Function to concatenate the base URL with the wordlist entries
pub fn build_url(base_url: &Url, wordlist_entry: &String) -> Result<Url, url::ParseError> {
    let built_url = base_url.join(wordlist_entry);

    match built_url {
        Ok(url) => Ok(url),
        Err(e) => {
            eprintln!("{}", format!("Error: {}", e).red());
            std::process::exit(1);
        }
    }
}
