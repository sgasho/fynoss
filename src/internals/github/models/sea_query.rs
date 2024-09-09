use std::fmt::Write;
use sea_query::Iden;

pub enum GHRepo {
    Table,
    ID,
    OwnerID,
    RepoName,
    Lang,
    Stars,
    URL,
    Description,
    Readme,
    CreatedAt,
    UpdatedAt,
}

impl Iden for GHRepo {
    fn unquoted(&self, s: &mut dyn Write) {
        write!(
            s,
            "{}", match self {
                Self::Table => "gh_repo",
                Self::ID => "id",
                Self::OwnerID => "owner_id",
                Self::RepoName => "repo_name",
                Self::Lang => "lang",
                Self::Stars => "stars",
                Self::URL => "url",
                Self::Description => "description",
                Self::Readme => "readme",
                Self::CreatedAt => "created_at",
                Self::UpdatedAt => "updated_at",
            }
        ).unwrap();
    }
}