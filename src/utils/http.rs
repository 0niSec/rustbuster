use url::Url;

use crate::buster::Scanner;
use crate::utils;
use colored::*;

const DISPLAY_INDENT: usize = 4;

pub async fn build_request(target_url: Url, client: &reqwest::Client) -> reqwest::Result<reqwest::Response> {
    client.get(target_url).send().await
}

pub async fn handle_response(response: reqwest::Response) {
    // Get the status code and the path from the response
    let status = response.status();
    let path = response.url().path();

    // Check the status code and print the path and status code (as a string) with the corresponding color
    if status.is_success() {
        println!("{:<DISPLAY_INDENT$} - [{}]", path, status.as_str().green(), DISPLAY_INDENT = DISPLAY_INDENT as usize)
    }

    if status.is_redirection() {
        println!("{:<DISPLAY_INDENT$} - [{}]", path, status.as_str().yellow(), DISPLAY_INDENT = DISPLAY_INDENT as usize)
    }

    if status.is_client_error() {
        println!("{:<DISPLAY_INDENT$} - [{}]", path, status.as_str().red(), DISPLAY_INDENT = DISPLAY_INDENT as usize)
    }
}

pub async fn bust_url(scanner: &Scanner) -> Result<(), Box<dyn std::error::Error>> {
    let client = &scanner.client;

    // Validate the URL
    let target_url = utils::url_utls::validate_url(&scanner.url)?;

    // Validate the wordlist
    let wordlist = utils::wordlist_utils::validate_wordlist_path(&scanner.wordlist);

    // Read the wordlist
    let lines = utils::wordlist_utils::read_wordlist(&wordlist);

    // Iterate over the wordlist and build the request
    while let Some(line) = lines.iter().next() {
        let constructed_url = utils::url_utls::build_url(&target_url, line)?;
        let response = build_request(constructed_url, &client).await?;

        handle_response(response);
    }

    Ok(())
}