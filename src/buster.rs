use reqwest::{Client, ClientBuilder};
use crate::{cli, utils};
use colored::*;
use clap::Parser;

#[derive(Debug)]
pub struct Scanner {
    pub url: String,
    pub wordlist: String,
    pub user_agent: String,
    pub client: Client,
    pub redirect_policy: String,
}

impl Scanner {
    pub fn new(args: cli::Args) -> Self {
        let redirect_policy = match args.redirect_policy.as_str() {
            "none" => reqwest::redirect::Policy::none(),
            "follow" => reqwest::redirect::Policy::limited(10),
            _ => {
                eprintln!("{}", format!("Error: Invalid redirect policy").red());
                std::process::exit(1);
            }
        };
        let client: Client = ClientBuilder::new()
            .user_agent(args.user_agent.clone())
            .redirect(redirect_policy)
            .build()
            .unwrap();


        Self {
            url: args.url,
            user_agent: args.user_agent,
            wordlist: args.wordlist,
            client: client,
            redirect_policy: args.redirect_policy
        }
    }

    pub async fn scan(&self) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Put this after the validation
        let buster = Scanner::new(cli::Args::parse());

        utils::http::bust_url(&buster).await?;

        println!("{}", format!("Scanning {} with wordlist {}...", &self.url, &self.wordlist).truecolor(255, 79, 0));



        Ok(())
    }
}