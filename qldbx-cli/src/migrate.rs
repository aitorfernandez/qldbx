use crate::cli::ConnectOpts;
use chrono::Utc;
use qldbx_core::{LedgerDriver, MigrationType, Migrator};
use std::path::Path;
use tokio::{fs, io::AsyncWriteExt};

pub async fn create(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(MigrationType::path()).await?;

    let mut file_name = Utc::now().format("%Y%m%d%H%M%S").to_string();
    file_name.push(MigrationType::split_char());
    file_name.push_str(&name.replace(' ', "_"));
    file_name.push_str(MigrationType::extension());

    let path = Path::new(MigrationType::path()).join(file_name);

    let mut file = fs::File::create(&path).await?;
    file.write_all(MigrationType::content().as_bytes()).await?;

    Ok(())
}

pub async fn run(connect_opts: &ConnectOpts) -> Result<(), Box<dyn std::error::Error>> {
    let _migrator = Migrator::new(Path::new(MigrationType::path())).await?;
    let _driver = LedgerDriver::new(&connect_opts.uri, &connect_opts.name).await?;

    todo!()
}
