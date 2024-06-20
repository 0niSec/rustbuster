use url::Url;

use crate::buster::Scanner;
use crate::utils;
use colored::*;

const DISPLAY_INDENT: usize = 24;

pub async fn build_request(target_url: Url, client: &reqwest::Client) -> reqwest::Result<reqwest::Response> {
    match client.get(target_url).send().await {
        Ok(response) => Ok(response),
        Err(e) => {
            eprintln!("{}", format!("Error: {}", e).red());
            std::process::exit(1);
        }
    }
}

pub async fn handle_response(response: reqwest::Response, negative_status_codes: &Vec<u16>) {
    // Get the status code and the path from the response
    let status = response.status();
    let response_size = response.content_length().unwrap_or(0);
    let path = response.url().path();

    // Check if the status code is in the negative status codes list
    if negative_status_codes.contains(&status.as_u16()) {
        return;
    }

    // Check the status code and print the path and status code (as a string) with the corresponding color
    if status.is_success() {
        let status_str = format!("[Status: {}]  (Size: {})", status.as_str(), response_size);
        println!("{:<DISPLAY_INDENT$} {}", path, status_str.green().bold(), DISPLAY_INDENT = DISPLAY_INDENT as usize);
    }
    
    if status.is_redirection() {
        let status_str = format!("[Status: {}]  (Size: {})", status.as_str(), response_size);
        println!("{:<DISPLAY_INDENT$} {}", path, status_str.yellow().bold(), DISPLAY_INDENT = DISPLAY_INDENT as usize);
    }
    
    if status.is_client_error() {
        let status_str = format!("[Status: {}]  (Size: {})", status.as_str(), response_size);
        println!("{:<DISPLAY_INDENT$} {}", path, status_str.red().bold(), DISPLAY_INDENT = DISPLAY_INDENT as usize);
    }}

pub async fn bust_url(scanner: &Scanner) -> Result<(), Box<dyn std::error::Error>> {
    let client = &scanner.client;

    // Validate the URL
    let target_url = utils::url_utls::validate_url(&scanner.url)?;

    // Validate the wordlist
    let wordlist = utils::wordlist_utils::validate_wordlist_path(&scanner.wordlist);

    // Read the wordlist
    let lines = utils::wordlist_utils::read_wordlist(&wordlist);

    // Iterate over the wordlist and build the request
    for line in &lines {
        let constructed_url = utils::url_utls::build_url(&target_url, &line)?;
        let response = build_request(constructed_url, &client).await?;

        handle_response(response, &scanner.negative_status_codes).await;
    }

    Ok(())
}