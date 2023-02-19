use super::{Migration, MigrationPath};
use std::borrow::Cow;

#[derive(Debug)]
pub struct Migrator {
    pub migrations: Cow<'static, [Migration]>,
}

impl Migrator {
    pub async fn new<'s, P>(path: P) -> Result<Self, Box<dyn std::error::Error>>
    where
        P: MigrationPath<'s>,
    {
        Ok(Self {
            migrations: Cow::Owned(path.resolve().await?),
        })
    }
}
