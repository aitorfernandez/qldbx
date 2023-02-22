use aws_sdk_qldb::Endpoint;

pub struct LedgerClient(aws_sdk_qldb::Client);

impl LedgerClient {
    /// Creates a new client from aws_sdk_qldb for create and delete ledger.
    pub async fn new(uri: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let sdk_config = aws_config::from_env()
            .endpoint_resolver(Endpoint::immutable(uri)?)
            .load()
            .await;

        Ok(Self(aws_sdk_qldb::Client::new(&sdk_config)))
    }

    /// Creates a new ledger with the uri and region used in the .env file.
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

    /// Deletes a ledger and all of its contents updating the protection first.
    pub async fn delete(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.0
            .update_ledger()
            .name(name)
            .deletion_protection(false)
            .send()
            .await?;

        self.0.delete_ledger().name(name).send().await?;

        Ok(())
    }
}
