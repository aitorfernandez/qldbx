use super::{Migration, MigrationArgs, MigrationType};
use std::{borrow::Cow, path::Path};
use tokio::fs;

#[async_trait::async_trait]
pub trait MigrationPath<'s> {
    async fn resolve(self) -> Result<Vec<Migration>, Box<dyn std::error::Error>>;
}

#[async_trait::async_trait]
impl<'s> MigrationPath<'s> for &'s Path {
    async fn resolve(self) -> Result<Vec<Migration>, Box<dyn std::error::Error>> {
        let mut dir = fs::read_dir(self.canonicalize()?).await?;
        let mut migrations = Vec::new();

        while let Some(entry) = dir.next_entry().await? {
            let file = entry.file_name();
            let file = file.to_string_lossy();
            let parts = file
                .splitn(2, MigrationType::split_char())
                .collect::<Vec<_>>();

            let utc: i64 = parts[0].parse()?;
            let name = parts[1]
                .trim_end_matches(MigrationType::extension())
                .to_owned();
            let mut stmt = fs::read_to_string(&entry.path()).await?;
            let _ = stmt.pop();

            migrations.push(Migration::new(MigrationArgs {
                name: Cow::Owned(name),
                statement: Cow::Owned(stmt),
                utc,
            }));
        }

        migrations.sort_by_key(|k| k.utc);

        Ok(migrations)
    }
}
