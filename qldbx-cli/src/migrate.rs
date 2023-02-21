use crate::cli::ConnectOpts;
use chrono::Utc;
use qldbx_core::{LedgerDriver, MigrationType, Migrator};
use std::path::Path;
use tokio::fs;

pub async fn create(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(MigrationType::path()).await?;

    let mut file_name = Utc::now().format("%Y%m%d%H%M%S").to_string();
    file_name.push(MigrationType::split_char());
    file_name.push_str(&name.replace(' ', "_"));
    file_name.push_str(MigrationType::extension());

    let path = Path::new(MigrationType::path()).join(file_name);

    fs::File::create(&path).await?;

    Ok(())
}

pub async fn run(connect_opts: &ConnectOpts) -> Result<(), Box<dyn std::error::Error>> {
    let migrator = Migrator::new(Path::new(MigrationType::path())).await?;
    let driver = LedgerDriver::new(&connect_opts.uri, &connect_opts.name).await?;

    driver.check_migrations().await?;

    let applied_migrations = driver.list_migrations().await?;

    for m in migrator.iter() {
        if !applied_migrations
            .iter()
            .any(|a| a.checksum == m.checksum())
        {
            driver.apply(m).await?;
            println!("> Applied {} {}", m.utc, m.name);
        }
    }

    Ok(())
}

pub async fn info(connect_opts: &ConnectOpts) -> Result<(), Box<dyn std::error::Error>> {
    let migrator = Migrator::new(Path::new(MigrationType::path())).await?;
    let driver = LedgerDriver::new(&connect_opts.uri, &connect_opts.name).await?;

    driver.check_migrations().await?;

    let applied_migrations = driver.list_migrations().await?;

    for m in migrator.iter() {
        let status = if applied_migrations
            .iter()
            .any(|a| a.checksum == m.checksum())
        {
            "Applied"
        } else {
            "Not Applied"
        };

        println!("> {status} {} {}", m.utc, m.name);
    }

    Ok(())
}
