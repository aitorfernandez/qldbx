use crate::{Migration, MigrationArgs, MigrationType};
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
            stmt.pop();

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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn resolve_test() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("migrations");
        fs::create_dir(&path).await.unwrap();

        let migration1_path = path.join("123456_test_migration_1.partiql");
        fs::write(&migration1_path, "select * from 1\n")
            .await
            .unwrap();

        let migration2_path = path.join("234567_test_migration_2.partiql");
        fs::write(&migration2_path, "select * from 2\n")
            .await
            .unwrap();

        let migrations = path.resolve().await.unwrap();

        assert_eq!(migrations.len(), 2);

        let migration1 = &migrations[0];
        assert_eq!(migration1.name, "test_migration_1");
        assert_eq!(migration1.statement, "select * from 1");
        assert_eq!(migration1.utc, 123456);

        let migration2 = &migrations[1];
        assert_eq!(migration2.name, "test_migration_2");
        assert_eq!(migration2.statement, "select * from 2");
        assert_eq!(migration2.utc, 234567);
    }
}
