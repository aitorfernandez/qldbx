use amazon_qldb_driver_core::{
    api::QldbSessionSdk,
    aws_sdk_qldbsession::{Config, Endpoint},
    QldbDriver, QldbDriverBuilder,
};

pub(crate) struct LedgerDriver(QldbDriver<QldbSessionSdk>);

impl LedgerDriver {
    pub async fn new(uri: &str, name: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let sdk_config = aws_config::from_env()
            .endpoint_resolver(Endpoint::immutable(uri)?)
            .load()
            .await;

        Ok(Self(
            QldbDriverBuilder::new()
                .ledger_name(name)
                .sdk_config(Config::new(&sdk_config))
                .await?,
        ))
    }
}
