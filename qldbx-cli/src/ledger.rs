use crate::ConnectOpts;

pub async fn create(connect_opts: &ConnectOpts) -> Result<(), Box<dyn std::error::Error>> {
    qldbx_core::ledger::create(&connect_opts.uri, &connect_opts.name).await?;

    Ok(())
}

pub async fn delete(connect_opts: &ConnectOpts) -> Result<(), Box<dyn std::error::Error>> {
    qldbx_core::ledger::delete(&connect_opts.uri, &connect_opts.name).await?;

    Ok(())
}
