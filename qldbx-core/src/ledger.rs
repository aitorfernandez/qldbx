struct LedgerClient(aws_sdk_qldb::Client);

impl LedgerClient {
    pub async fn new(uri: &str) -> Self {
        let aws_config = aws_config::from_env().load().await;
        let config = aws_sdk_qldb::config::Builder::from(&aws_config)
            .endpoint_url(uri)
            .build();

        Self(aws_sdk_qldb::Client::from_conf(config))
    }

    pub async fn create(&self, name: &str) -> Result<Option<String>, Box<dyn std::error::Error>> {
        Ok(self
            .0
            .create_ledger()
            .name(name)
            .permissions_mode(aws_sdk_qldb::model::PermissionsMode::AllowAll)
            .send()
            .await?
            .arn)
    }

    pub async fn delete(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self
            .0
            .update_ledger()
            .name(name)
            .deletion_protection(false)
            .send()
            .await?;

        let _ = self.0.delete_ledger().name(name).send().await?;

        Ok(())
    }
}

pub async fn create(uri: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = LedgerClient::new(uri).await;
    let res = client.create(name).await?;

    println!("{res:?}");

    Ok(())
}

pub async fn delete(uri: &str, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = LedgerClient::new(uri).await;
    client.delete(name).await?;

    Ok(())
}
