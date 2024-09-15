use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    StatusCode(u16),
    ReadmeNotFound
}

#[derive(Deserialize, Serialize)]
pub struct AIInquiryResponse {
    pub status: Status,
    pub text: String,
}
