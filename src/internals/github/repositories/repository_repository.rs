use std::error::Error;
use sea_query::{Expr, MysqlQueryBuilder, Order, Query};
use sea_query_binder::SqlxBinder;
use sqlx::{MySql, Pool};
use crate::internals::github::models::dto::{SearchRepositoriesRequest};
use crate::internals::github::models::entity::GithubRepository;
use crate::internals::github::models::sea_query::GHRepo;

pub trait RepositoryRepository {
    async fn find_list(&self, req: SearchRepositoriesRequest) -> Result<Vec<GithubRepository>, Box<dyn Error>>;
    async fn bulk_insert(&self, repos: Vec<GithubRepository>) -> Result<(), Box<dyn Error>>;
}

#[derive(Clone)]
pub struct GithubRepositoryRepository {
    pool: Pool<MySql>
}

impl GithubRepositoryRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self {
            pool,
        }
    }
}

impl RepositoryRepository for GithubRepositoryRepository {
    async fn find_list(&self, req: SearchRepositoriesRequest) -> Result<Vec<GithubRepository>, Box<dyn Error>> {
        let (q, args) = Query::select()
            .columns([
                GHRepo::ID,
                GHRepo::OwnerID,
                GHRepo::RepoName,
                GHRepo::Stars,
                GHRepo::Lang,
                GHRepo::URL,
                GHRepo::Description,
                GHRepo::Readme,
                GHRepo::CreatedAt,
                GHRepo::UpdatedAt,
            ])
            .from(GHRepo::Table)
            .and_where(Expr::col(GHRepo::Lang).eq(req.language))
            .and_where(Expr::col(GHRepo::Stars).gte(req.min_stars))
            .and_where(Expr::col(GHRepo::Stars).lte(req.max_stars))
            .order_by(GHRepo::Stars, Order::Desc)
            .build_sqlx(MysqlQueryBuilder);

        let pool = self.pool.clone();

        let rows: Vec<GithubRepository> = sqlx::query_as_with(&q, args)
            .fetch_all(&pool)
            .await?;

       Ok(rows)
    }

    async fn bulk_insert(&self, repos: Vec<GithubRepository>) -> Result<(), Box<dyn Error>> {
        let mut stmt = Query::insert()
            .into_table(GHRepo::Table)
            .columns([
                GHRepo::ID,
                GHRepo::OwnerID,
                GHRepo::RepoName,
                GHRepo::Stars,
                GHRepo::URL,
                GHRepo::Description,
                GHRepo::Readme,
                GHRepo::CreatedAt,
                GHRepo::UpdatedAt,
            ]).to_owned();

        for repo in repos {
            stmt.values(vec![
                repo.id.into(),
                repo.owner_id.into(),
                repo.repo_name.into(),
                repo.stars.into(),
                repo.url.into(),
                repo.description.into(),
                repo.readme.into(),
                repo.created_at.into(),
                repo.updated_at.into()
            ])?;
        }

        let (q, args) = stmt.build_sqlx(MysqlQueryBuilder);
        let pool = self.pool.clone();

        match sqlx::query_with(&q, args).execute(&pool).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into())
        }
    }
}