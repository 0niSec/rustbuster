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
}

impl Scanner {
    pub fn new(args: cli::Args) -> Self {
        let client: Client = ClientBuilder::new()
            .user_agent(args.user_agent.clone())
            .build()
            .unwrap();


        Self {
            url: args.url,
            user_agent: args.user_agent,
            wordlist: args.wordlist,
            client: client,
        }
    }

    pub async fn scan(&self) {
        // TODO: Put this after the validation
        println!("{}", format!("Scanning {} with wordlist {}...", &self.url, &self.wordlist).truecolor(255, 79, 0));

        let buster = Scanner::new(cli::Args::parse());
        
        utils::http::bust_url(&buster).await.unwrap();
    }
}