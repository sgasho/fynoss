use serde::Deserialize;

pub struct SearchRepositoriesRequest {
    pub min_stars: u32,
    pub max_stars: Option<u32>,
    pub last_pushed: String,
    pub language: String,
    pub good_first_issues_count: u32,
    pub help_wanted_count: u32,
}

#[derive(Deserialize, Debug)]
pub struct Owner {
    pub login: String,
    pub avatar_url: String,
}

#[derive(Deserialize, Debug)]
pub struct Repository {
    pub id: u32,
    pub name: String,
    pub full_name: String,
    pub stargazers_count: u32,
    pub html_url: String,
    pub description: String,
    pub owner: Owner,
}

#[derive(Deserialize, Debug)]
pub struct Repositories {
    pub total_count: u32,
    pub items: Vec<Repository>,
}

#[derive(Deserialize, Debug)]
pub struct ReadmeClientResponse {
    pub content: String,
    pub encoding: String,
}

#[derive(Deserialize, Debug)]
pub struct ReadmeResponse {
    pub found: bool,
    pub content: Option<String>,
}