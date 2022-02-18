extern crate clap;
extern crate skim;
extern crate tokio;

mod github;
mod config;
mod cache;
mod util;

use std::{io::{stdin, stdout, Write}};
use futures::future;

use clap::Parser;
use github::get_starred_repos;
use skim::prelude::*;


#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(long)]
    sync: bool,
}


async fn async_repo_pusher(cache: Vec<github::Repo>, config: config::Config, tx: SkimItemSender) {
    let mut page = 1;
    let mut all_repo: Vec<github::Repo> = vec![];
    let mut ids = future::try_join_all(cache.iter().map(| item | item.get_id()))
        .await
        .unwrap();
    ids.sort();

    loop {
        let fetch_result = get_starred_repos(&config.username, &config.token, page).await;

        if fetch_result.is_err() {
            println!("Error: {}", fetch_result.err().unwrap());
            break;
        }
    
        let repos = fetch_result.ok().unwrap();
        all_repo.append(&mut repos.clone());
        if repos.len() == 0 {
            break;
        }

        for repo in repos {
            if ids.binary_search(&repo.id).is_err() {
                tx.send(Arc::new(repo)).unwrap_or_default();
            }
        }

        page += 1;
    }

    cache::write(all_repo);
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
    let args = Args::parse();

    let config = config::read();

    let config = match config {
        Some(config) => config,
        None => {
            let config = get_username_and_access_token();
            config::write(config.clone());
            config
        }
    };

    let cache = cache::read();

    let options = SkimOptionsBuilder::default()
        .height(Some("100%"))
        .multi(false)
        .build()
        .unwrap();

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    if cache.is_none() || args.sync {
        tokio::spawn(async_repo_pusher(vec![], config, tx.clone()));
    } else {
        for repo in cache.unwrap().data {
            tx.send(Arc::new(repo)).unwrap_or_default();
        }
    }
    
    
    let selected_items = Skim::run_with(&options, Some(rx))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    for item in selected_items.iter() {
        print!("{}", item.output());
    }
}
