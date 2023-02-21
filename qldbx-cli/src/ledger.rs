use crate::cli::ConnectOpts;
use qldbx_core::LedgerClient;

pub async fn create(connect_opts: &ConnectOpts) -> Result<(), Box<dyn std::error::Error>> {
    let client = LedgerClient::new(&connect_opts.uri).await?;
    client.create(&connect_opts.name).await?;

    Ok(())
}

pub async fn delete(connect_opts: &ConnectOpts) -> Result<(), Box<dyn std::error::Error>> {
    let client = LedgerClient::new(&connect_opts.uri).await?;
    client.delete(&connect_opts.name).await?;

    Ok(())
}

pub async fn reset(connect_opts: &ConnectOpts) -> Result<(), Box<dyn std::error::Error>> {
    delete(connect_opts).await?;
    create(connect_opts).await?;

    super::migrate::run(connect_opts).await
}
