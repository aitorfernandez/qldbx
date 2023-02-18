pub async fn create() -> Result<(), Box<dyn std::error::Error>> {
    qldbx_core::ledger::create();

    Ok(())
}

pub async fn delete() -> Result<(), Box<dyn std::error::Error>> {
    qldbx_core::ledger::delete();

    Ok(())
}
