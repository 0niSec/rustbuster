use url::Url;
use colored::*;

// Function to parse the String URL into a URL struct so it can be used later for reqwest
// We return the URL struct if the URL is valid, otherwise we print an error message and exit the program
pub fn validate_url(url: &str) -> Result<Url, url::ParseError> {
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
pub fn build_url(base_url: &Url, wordlist_entry: &str) -> Result<Url, url::ParseError> {
    let mut url = Url::parse(base_url.as_str())?;

    url.set_path(wordlist_entry);

    Ok(url)
}