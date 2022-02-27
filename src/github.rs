extern crate serde;
extern crate serde_json;

use std::borrow::Cow;

use reqwest::{
    header::{AUTHORIZATION, USER_AGENT},
    Error,
};
use serde::{Deserialize, Serialize};
use skim::SkimItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub owner: Owner,
    pub html_url: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub git_url: String,
    pub ssh_url: String,
    pub clone_url: String,
    pub homepage: Option<String>,
    pub stargazers_count: i64,
    pub watchers_count: i64,
    pub language: Option<String>,
    pub forks_count: i64,
    pub archived: bool,
    pub disabled: bool,
    pub open_issues_count: i64,
    pub topics: Vec<String>,
    pub visibility: String,
}

impl Repo {
    pub async fn get_id(&self) -> Result<i64, Error> {
        Ok(self.id)
    }
}

impl SkimItem for Repo {
    fn text(&self) -> Cow<str> {
        let description = self.description.clone().unwrap_or("".to_string());
        let string = String::from(self.html_url.clone() + "\n" + description.as_str());
        Cow::Owned(string)
    }

    fn display<'a>(&'a self, _: skim::DisplayContext<'a>) -> skim::AnsiString<'a> {
        let description = self.description.clone().unwrap_or("".to_string());
        let language = self.language.clone().unwrap_or("No Language".to_string());
        let display_text = format!(
            "[{}] {} by {}: {}",
            language, self.name, self.owner.login, description
        );
        skim::AnsiString::from(display_text)
    }

    fn preview(&self, _context: skim::PreviewContext) -> skim::ItemPreview {
        skim::ItemPreview::Global
    }

    fn output(&self) -> Cow<str> {
        Cow::Owned(self.html_url.as_str().to_string())
    }

    fn get_matching_ranges(&self) -> Option<&[(usize, usize)]> {
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    pub login: String,
    pub id: i64,
}

pub async fn get_starred_repos(username: &str, token: &str, page: i32) -> Result<Vec<Repo>, Error> {
    let url = format!(
        "https://api.github.com/users/{}/starred?per_page=100&page={}",
        username, page
    );
    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header(AUTHORIZATION, format!("token {}", token))
        .header(USER_AGENT, "blurfx/ghost")
        .send()
        .await?;

    let repos: Vec<Repo> = response.json().await?;

    Ok(repos)
}
