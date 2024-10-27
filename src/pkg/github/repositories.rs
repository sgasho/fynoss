use std::error::Error;
use log::info;
use crate::pkg::github::client::client::GithubApiClient;
use crate::pkg::github::model::model::{SearchRepositoriesRequest, Repositories, ReadmeClientResponse, ReadmeResponse, SearchIssuesRequest, Issue};
use crate::pkg::utils::base64::base64::decode_to_string;

const SEARCH_REPOSITORIES_URL: &str = "https://api.github.com/search/repositories";
const REPOSITORY_URL: &str = "https://api.github.com/repos";

pub trait RepositoryClient {
    async fn fetch_repositories(&self, req: SearchRepositoriesRequest) -> Result<Repositories, Box<dyn Error>>;
    async fn fetch_top_readme(&self, owner_name: &str, repository_name: &str) -> Result<ReadmeResponse, Box<dyn Error>>;
    async fn fetch_issues(&self, owner_name: &str, repository_name: &str, req: SearchIssuesRequest) -> Result<Vec<Issue>, Box<dyn Error>>;
}

#[derive(Clone)]
pub struct GithubRepositoryClient<C: GithubApiClient> {
    client: C,
}

impl<C: GithubApiClient> GithubRepositoryClient<C> {
    pub fn new(client: C) -> Self {
        Self {
            client,
        }
    }
}

impl<C: GithubApiClient> RepositoryClient for GithubRepositoryClient<C> {
    async fn fetch_repositories(
        &self,
        req: SearchRepositoriesRequest,
    ) -> Result<Repositories, Box<dyn Error>> {
        let q_stars =
            if let Some(max_stars) = req.max_stars {
                format!("stars:{}..{}", req.min_stars, max_stars)
            } else {
                format!("stars:>={}", req.min_stars)
            };
        let url = format!(
            "{}\
            ?q={}+language:{}+archived:false+good-first-issues:>={}+help-wanted-issues:>={}+pushed:>={}\
            &sort=stars&order=desc",
            SEARCH_REPOSITORIES_URL,
            q_stars, req.language, req.good_first_issues_count, req.help_wanted_count, req.last_pushed
        );

        let res = self.client.get(&url).await?;
        let repos: Repositories = serde_json::from_str(&res.text)?;

        Ok(repos)
    }

    async fn fetch_top_readme(&self, owner_name: &str, repository_name: &str) -> Result<ReadmeResponse, Box<dyn Error>> {
        let url = format!("{}/{}/{}/readme", REPOSITORY_URL, owner_name, repository_name);

        let res = self.client.get(&url).await?;

        if res.status == 404 {
            return Ok(ReadmeResponse {
                found: false,
                content: None,
            })
        }

        let readme: ReadmeClientResponse = serde_json::from_str(&res.text)?;

        if readme.encoding == "base64" {
            let content = decode_to_string(&readme.content)?;
            Ok(ReadmeResponse {
                found: true,
                content: Some(content)
            })
        } else {
            Err("Unknown encoding for README content".into())
        }
    }

    async fn fetch_issues(&self, owner_name: &str, repository_name: &str, req: SearchIssuesRequest) -> Result<Vec<Issue>, Box<dyn Error>> {
        let url = format!(
            "{}/{}/{}/issues\
            ?state={:?}&assignee={}&labels={}&sort={:?}&direction={:?}",
            REPOSITORY_URL, owner_name, repository_name, req.state, req.assignee, req.labels.join(","), req.sort_key, req.sort_order
        );

        let res = self.client.get(&url).await?;
        let issues: Vec<Issue> = serde_json::from_str(&res.text)?;

        Ok(issues)
    }
}

#[cfg(test)]
mod tests {
    use reqwest::StatusCode;
    use crate::pkg::github::client::client::{GithubApiClient, GithubClientResponse};
    use crate::pkg::github::model::model::{IssueState, SearchIssuesSortKey, SortOrder};
    use crate::pkg::github::repositories::tests::Mode::{FetchRepositories, FetchTopReadmeOkFound, FetchTopReadmeOkNotFound, FetchIssues};
    use super::*;

    enum Mode {
        FetchRepositories,
        FetchTopReadmeOkFound,
        FetchTopReadmeOkNotFound,
        FetchIssues,
    }

    struct MockClient {
        mode: Mode
    }

    impl GithubApiClient for MockClient {
        async fn get(&self, _url: &str) -> Result<GithubClientResponse, Box<dyn Error>> {
            match self.mode {
                FetchRepositories => {
                    Ok(GithubClientResponse {
                        text: r#"{
                            "total_count": 2,
                            "items": [
                                {
                                    "id": 1,
                                    "name": "repo1",
                                    "full_name": "mock/repo1",
                                    "stargazers_count": 1000,
                                    "html_url": "https://github.com/mock/repo1",
                                    "description": "dsc1",
                                    "owner": {
                                        "login": "mock",
                                        "avatar_url": "https://avatar.com/1"
                                    }
                                },
                                {
                                    "id": 2,
                                    "name": "repo2",
                                    "full_name": "mock/repo2",
                                    "stargazers_count": 1001,
                                    "html_url": "https://github.com/mock/repo2",
                                    "description": "dsc2",
                                    "owner": {
                                        "login": "mock",
                                        "avatar_url": "https://avatar.com/2"
                                    }
                                }
                            ]
                        }"#.to_string(),
                        status: StatusCode::OK,
                    })
                }
                FetchTopReadmeOkFound => {
                    Ok(GithubClientResponse {
                        text: r#"{
                            "content": "PGRpdiBhbGlnbj0iY2VudGVyIj4KPHAgYWxpZ249ImNlbnRlciI+Cgo8aW1n",
                            "encoding": "base64"
                        }"#.to_string(),
                        status: StatusCode::OK,
                    })
                }
                FetchTopReadmeOkNotFound => {
                    Ok(GithubClientResponse {
                        text: "".to_string(),
                        status: StatusCode::NOT_FOUND
                    })
                }
                FetchIssues => {
                    Ok(GithubClientResponse {
                        text: r#"[
                            {
                                "id": 1,
                                "url": "url1",
                                "title": "title1",
                                "body": "body1"
                            },
                            {
                                "id": 2,
                                "url": "url2",
                                "title": "title2",
                                "body": "body2"
                            }
                        ]"#.to_string(),
                        status: StatusCode::OK
                    })
                }
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_repositories() {
        let client = MockClient {
            mode: FetchRepositories
        };
        let repository_client = GithubRepositoryClient::new(client);
        let result = repository_client.fetch_repositories(SearchRepositoriesRequest {
            min_stars: 1000,
            max_stars: Some(1001),
            last_pushed: "2024-09-07".to_string(),
            language: "go".to_string(),
            good_first_issues_count: 1,
            help_wanted_count: 1,
        }).await.unwrap();

        assert_eq!(result.total_count, 2);
        assert_eq!(result.items.len(), 2);
        assert_eq!(result.items[0].id, 1);
        assert_eq!(result.items[0].name, "repo1");
        assert_eq!(result.items[0].full_name, "mock/repo1");
        assert_eq!(result.items[0].stargazers_count, 1000);
        assert_eq!(result.items[0].html_url, "https://github.com/mock/repo1");
        assert_eq!(result.items[0].description, "dsc1");
        assert_eq!(result.items[0].owner.login, "mock");
        assert_eq!(result.items[0].owner.avatar_url, "https://avatar.com/1");
        assert_eq!(result.items[1].id, 2);
        assert_eq!(result.items[1].name, "repo2");
        assert_eq!(result.items[1].full_name, "mock/repo2");
        assert_eq!(result.items[1].stargazers_count, 1001);
        assert_eq!(result.items[1].html_url, "https://github.com/mock/repo2");
        assert_eq!(result.items[1].description, "dsc2");
        assert_eq!(result.items[1].owner.login, "mock");
        assert_eq!(result.items[1].owner.avatar_url, "https://avatar.com/2");
    }

    #[tokio::test]
    async fn test_fetch_top_readme_ok_found() {
        let client = MockClient {
            mode: FetchTopReadmeOkFound
        };
        let repository_client = GithubRepositoryClient::new(client);
        let result = repository_client.fetch_top_readme("owner", "repo").await.unwrap();

        assert!(result.found);
        assert_eq!(result.content.unwrap(), "<div align=\"center\">\n<p align=\"center\">\n\n<img");
    }

    #[tokio::test]
    async fn test_fetch_top_readme_ok_not_found() {
        let client = MockClient {
            mode: FetchTopReadmeOkNotFound
        };
        let repository_client = GithubRepositoryClient::new(client);
        let result = repository_client.fetch_top_readme("owner", "repo").await.unwrap();

        assert!(!result.found);
    }
    
    #[tokio::test]
    async fn test_fetch_issues_ok() {
        let client = MockClient {
            mode: FetchIssues,
        };
        let repository_client = GithubRepositoryClient::new(client);
        let result = repository_client.fetch_issues("owner", "repo", SearchIssuesRequest {
            state: IssueState::Open,
            assignee: "none".to_string(),
            labels: vec!["label1".to_string(), "label2".to_string()],
            sort_key: SearchIssuesSortKey::Created,
            sort_order: SortOrder::Asc,
        }).await.unwrap();

        assert_eq!(result.len(), 2);
    }
}
