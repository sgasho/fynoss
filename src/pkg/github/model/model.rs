use std::fmt::{Debug, Formatter};
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

pub enum IssueState {
    Open,
    Closed,
    All,
}

impl Debug for IssueState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            IssueState::Open => write!(f, "open"),
            IssueState::Closed => write!(f, "closed"),
            IssueState::All => write!(f, "all"),
        }
    }
}

pub enum SearchIssuesSortKey {
    Created,
    Updated,
    Comments,
}

impl Debug for SearchIssuesSortKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchIssuesSortKey::Created => write!(f, "created"),
            SearchIssuesSortKey::Updated => write!(f, "updated"),
            SearchIssuesSortKey::Comments => write!(f, "comments"),
        }
    }
}

pub enum SortOrder {
    Asc,
    Desc,
}

impl Debug for SortOrder {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Asc => write!(f, "asc"),
            SortOrder::Desc => write!(f, "desc"),
        }
    }
}

pub struct SearchIssuesRequest {
    pub state: IssueState,
    pub assignee: String,
    pub labels: Vec<String>,
    pub sort_key: SearchIssuesSortKey,
    pub sort_order: SortOrder
}

#[derive(Deserialize, Debug)]
pub struct Issue {
    pub html_url: String,
    pub title: String,
    pub body: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Issues {
    pub total_count: u32,
    pub items: Vec<Issue>,
}
