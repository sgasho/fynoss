use serde::{Deserialize, Serialize};
use crate::pkg::github::model::model as pkg_model;

#[derive(Deserialize)]
pub struct SearchRepositoriesRequest {
    pub min_stars: u32,
    pub max_stars: Option<u32>,
    pub last_pushed: String,
    pub language: String,
    pub good_first_issues_count: u32,
    pub help_wanted_count: u32,
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Owner {
    pub name: String,
    pub avatar_url: String,
}

impl From<pkg_model::Owner> for Owner {
    fn from(value: pkg_model::Owner) -> Self {
        Self {
            name: value.login,
            avatar_url: value.avatar_url,
        }
    }
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct Repository {
    pub id: u32,
    pub name: String,
    pub full_name: String,
    pub stargazers_count: u32,
    pub url: String,
    pub description: String,
    pub owner: Owner,
}

impl From<pkg_model::Repository> for Repository {
    fn from(value: pkg_model::Repository) -> Self {
        Self {
            id: value.id,
            name: value.name,
            full_name: value.full_name,
            stargazers_count: value.stargazers_count,
            description: value.description,
            url: value.html_url,
            owner: value.owner.into(),
        }
    }
}

#[derive(sqlx::FromRow, Serialize, Debug)]
pub struct Repositories {
    pub total_count: u32,
    pub items: Vec<Repository>,
}

impl From<pkg_model::Repositories> for Repositories {
    fn from(value: pkg_model::Repositories) -> Self {
        Self {
            total_count: value.total_count,
            items: value.items.into_iter().map(Repository::from).collect()
        }
    }
}

#[derive(Deserialize, Debug, Serialize)]
pub struct ReadmeResponse {
    pub found: bool,
    pub content: Option<String>,
}

impl From<pkg_model::ReadmeResponse> for ReadmeResponse {
    fn from(value: pkg_model::ReadmeResponse) -> Self {
        Self {
            found: value.found,
            content: value.content
        }
    }
}