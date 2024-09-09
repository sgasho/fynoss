use chrono::NaiveDateTime;

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