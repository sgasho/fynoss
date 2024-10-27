use std::fmt::{Debug, Formatter};
use chrono::NaiveDateTime;
use serde::Deserialize;
use crate::pkg::github::model::model::{IssueState as PkgIssueState, SearchIssuesSortKey as PkgSearchIssuesSortKey, SortOrder as PkgSortOrder};

#[derive(sqlx::FromRow, Debug)]
pub struct GithubRepository {
    pub id: u32,
    pub owner_id: u32,
    pub repo_name: String,
    pub stars: u32,
    pub url: String,
    pub description: String,
    pub readme: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub enum IssueState {
    Open,
    Closed,
    All,
}

impl IssueState {
    pub fn new(state: Option<String>) -> Self {
        match state.as_deref() {
            None => IssueState::All,
            Some("open") => IssueState::Open,
            Some("closed") => IssueState::Closed,
            _ => IssueState::All,
        }
    }
}

impl From<IssueState> for PkgIssueState  {
    fn from(value: IssueState) -> Self {
        match value {
            IssueState::Open => PkgIssueState::Open,
            IssueState::Closed => PkgIssueState::Closed,
            _ => PkgIssueState::All,
        }
    }
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

impl SearchIssuesSortKey {
    pub fn new(key: Option<String>) -> Self {
        match key.as_deref() {
            None => SearchIssuesSortKey::Created,
            Some("created") => SearchIssuesSortKey::Created,
            Some("updated") => SearchIssuesSortKey::Updated,
            Some("comments") => SearchIssuesSortKey::Comments,
            _ => SearchIssuesSortKey::Created,
        }
    }
}

impl From<SearchIssuesSortKey> for PkgSearchIssuesSortKey {
    fn from(value: SearchIssuesSortKey) -> Self {
        match value {
            SearchIssuesSortKey::Created => PkgSearchIssuesSortKey::Created,
            SearchIssuesSortKey::Updated => PkgSearchIssuesSortKey::Updated,
            SearchIssuesSortKey::Comments => PkgSearchIssuesSortKey::Comments,
        }
    }
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

impl SortOrder {
    pub fn new(order: Option<String>) -> Self {
        match order.as_deref() {
            Some("asc") => SortOrder::Asc,
            Some("desc") => SortOrder::Desc,
            None => SortOrder::Desc,
            _ => SortOrder::Desc,
        }
    }
}

impl From<SortOrder> for PkgSortOrder {
    fn from(value: SortOrder) -> Self {
        match value {
            SortOrder::Asc => PkgSortOrder::Asc,
            SortOrder::Desc => PkgSortOrder::Desc,
        }
    }
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

impl From<SearchIssuesRequestQueries> for SearchIssuesRequest {
    fn from(q: SearchIssuesRequestQueries) -> Self {
        let assignee = q.assignee.unwrap_or_else(|| "none".to_string());
        let labels = q.labels.unwrap_or_default().split(',').map(|s| s.to_string()).collect();
        Self {
            state: IssueState::new(q.state),
            assignee,
            labels,
            sort_key: SearchIssuesSortKey::new(q.sort_key),
            sort_order: SortOrder::new(q.sort_order),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct SearchIssuesRequestQueries {
    pub state: Option<String>,
    pub assignee: Option<String>,
    pub labels: Option<String>,
    pub sort_key: Option<String>,
    pub sort_order: Option<String>,
}
