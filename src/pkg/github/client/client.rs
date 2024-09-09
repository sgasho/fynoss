use reqwest::Client as ReqwestClient;
use std::error::Error;
use reqwest::StatusCode;

pub trait GithubApiClient {
    async fn get(&self, url: &str) -> Result<GithubClientResponse, Box<dyn Error>>;
}

pub struct GithubClient {
    token: String,
    reqwest_client: ReqwestClient
}

impl GithubClient {
    pub fn new(token: String) -> Self {
        Self {
            token,
            reqwest_client: ReqwestClient::new()
        }
    }
}

#[derive(Debug)]
pub struct GithubClientResponse {
    pub text: String,
    pub status: StatusCode,
}

impl GithubApiClient for GithubClient {
    async fn get(&self, url: &str) -> Result<GithubClientResponse, Box<dyn Error>> {
        let res = self.reqwest_client
            .get(url)
            .header("Authorization", format!("token {}", self.token))
            .header("User-Agent", "rust-api-client")
            .send()
            .await?;
        let status = res.status();
        let text = res.text().await?;

        Ok(GithubClientResponse {
            text,
            status
        })
    }
}