use chrono::Utc;
use qldbx_core::MigrationType;
use std::path::Path;
use tokio::{fs, io::AsyncWriteExt};

const MIGRATIONS_PATH: &'static str = "./migrations";

pub async fn create(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(MIGRATIONS_PATH).await?;

    let mut file_name = Utc::now().format("%Y%m%d%H%M%S").to_string();
    file_name.push_str("_");
    file_name.push_str(&name.replace(' ', "_"));
    file_name.push_str(MigrationType::extension());

    let path = Path::new(MIGRATIONS_PATH).join(file_name);

    let mut file = fs::File::create(&path).await?;
    file.write_all(MigrationType::content().as_bytes()).await?;

    Ok(())
}
