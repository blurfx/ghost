extern crate skim;
extern crate tokio;

mod github;
mod config;

use std::{io::{stdin, stdout, Write}};

use github::get_starred_repos;
use skim::prelude::*;

async fn async_repo_pusher(config: config::Config, tx: SkimItemSender) {
    let mut page = 1;
    loop {
        let fetch_result = get_starred_repos(&config.username, &config.token, page).await;

        if fetch_result.is_err() {
            println!("Error: {}", fetch_result.err().unwrap());
            break;
        }
    
        let repos = fetch_result.ok().unwrap();

        if repos.len() == 0 {
            break;
        }

        for repo in repos {
            tx.send(Arc::new(repo)).unwrap_or_default();
        }

        page += 1;
    }
}


fn get_username_and_access_token() -> config::Config {
    let mut username = String::new();
    let mut token = String::new();
    print!("Your GitHub username (i.e. blurfx): ");
    stdout().flush().unwrap_or_default();
    stdin().read_line(&mut username).unwrap();
    print!("Your GitHub personal access token: ");
    stdout().flush().unwrap_or_default();
    stdin().read_line(&mut token).unwrap();

    username = username.trim().to_string();
    token = token.trim().to_string();

    config::Config {
        username,
        token,
    }
}

#[tokio::main]
pub async fn main() {
    let config_path = config::get_filepath();

    if !config_path.as_path().exists() {
        let config = get_username_and_access_token();
        config::write(config_path.clone(), config);
    }

    let config = config::read(config_path);

    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(false)
        .build()
        .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();
    tokio::spawn(async_repo_pusher(config, tx.clone()));
    
    let selected_items = Skim::run_with(&options, Some(rx))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        print!("{}\n", item.output());
    }
}
