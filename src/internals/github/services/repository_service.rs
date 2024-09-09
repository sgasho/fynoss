use std::error::Error;
use crate::internals::github::models::dto::{Repositories, SearchRepositoriesRequest, ReadmeResponse};
use crate::internals::github::repositories::repository_repository::RepositoryRepository;
use crate::pkg::github::model::model as p_model;
use crate::pkg::github::repositories::{RepositoryClient};

pub trait RepositoryService {
    async fn fetch_repositories(&self, req: SearchRepositoriesRequest) -> Result<Repositories, Box<dyn Error>>;
    async fn fetch_top_readme(&self, owner_name: &str, repository_name: &str) -> Result<ReadmeResponse, Box<dyn Error>>;
}

pub struct GithubRepositoryService<C: RepositoryClient, R: RepositoryRepository> {
    client: C,
    repository: R,
}

impl<C: RepositoryClient, R: RepositoryRepository> GithubRepositoryService<C, R> {
    pub fn new(client: C, repository: R) -> Self {
        Self {
            client,
            repository
        }
    }
}

impl<C: RepositoryClient, R: RepositoryRepository> RepositoryService for GithubRepositoryService<C, R> {
    async fn fetch_repositories(&self, req: SearchRepositoriesRequest) -> Result<Repositories, Box<dyn Error>> {
        let client_req = p_model::SearchRepositoriesRequest {
            min_stars: req.min_stars,
            max_stars: req.max_stars,
            last_pushed: req.last_pushed,
            language: req.language,
            good_first_issues_count: req.good_first_issues_count,
            help_wanted_count: req.help_wanted_count,
        };

        let client_res = self.client.fetch_repositories(client_req).await?;
        Ok(client_res.into())
    }

    async fn fetch_top_readme(&self, owner_name: &str, repository_name: &str) -> Result<ReadmeResponse, Box<dyn Error>> {
        let res = self.client.fetch_top_readme(owner_name, repository_name).await?;
        Ok(res.into())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use crate::internals::github::services::repository_service::{GithubRepositoryService, RepositoryService};
    use crate::internals::github::models::dto as i_model;
    use crate::pkg::github::model::model as p_model;
    use crate::pkg::github::model::model::ReadmeResponse;
    use crate::pkg::github::repositories::RepositoryClient;

    struct MockClient {
        should_fail: bool
    }

    impl RepositoryClient for MockClient {
        fn fetch_repositories(&self, _req: p_model::SearchRepositoriesRequest) -> Result<p_model::Repositories, Box<dyn Error>> {
            if self.should_fail {
                Err("Failed to fetch repositories".into())
            } else {
                Ok(p_model::Repositories {
                    total_count: 1,
                    items: vec![p_model::Repository {
                        id: 2,
                        name: "name".to_string(),
                        full_name: "full_name".to_string(),
                        stargazers_count: 3,
                        html_url: "html_url".to_string(),
                        description: "description".to_string(),
                        owner: p_model::Owner { login: "owner_name".to_string(), avatar_url: "avatar_url".to_string() },
                    }],
                })
            }
        }

        fn fetch_top_readme(&self, _owner_name: &str, _repository_name: &str) -> Result<ReadmeResponse, Box<dyn Error>> {
            todo!()
        }
    }

    #[test]
    fn test_fetch_repositories_ok() {
        let client = MockClient {should_fail: false};
        let repository_info_service = GithubRepositoryService::new(client);
        let res = repository_info_service.fetch_repositories(i_model::SearchRepositoriesRequest {
            min_stars: 0,
            max_stars: None,
            last_pushed: "".to_string(),
            language: "".to_string(),
            good_first_issues_count: 0,
            help_wanted_count: 0,
        }).unwrap();

        assert_eq!(res.total_count, 1);
        assert_eq!(res.items.len(), 1);
        assert_eq!(res.items[0].id, 2);
        assert_eq!(res.items[0].name, "name");
        assert_eq!(res.items[0].full_name, "full_name");
        assert_eq!(res.items[0].stargazers_count, 3);
        assert_eq!(res.items[0].url, "html_url");
        assert_eq!(res.items[0].description, "description");
        assert_eq!(res.items[0].owner.name, "owner_name");
        assert_eq!(res.items[0].owner.avatar_url, "avatar_url");
    }

    #[test]
    fn test_fetch_repositories_ng() {
        let client = MockClient { should_fail: true };
        let repository_info_service = GithubRepositoryService::new(client);

        let res = repository_info_service.fetch_repositories(i_model::SearchRepositoriesRequest {
            min_stars: 0,
            max_stars: None,
            last_pushed: "".to_string(),
            language: "".to_string(),
            good_first_issues_count: 0,
            help_wanted_count: 0,
        });

        assert!(res.is_err());

        if let Err(e) = res {
            assert_eq!(e.to_string(), "Failed to fetch repositories");
        }
    }
}