use super::Migration;
use amazon_qldb_driver_core::{
    api::QldbSessionSdk,
    aws_sdk_qldbsession::{Config, Endpoint},
    transaction::StatementResults,
    QldbDriver, QldbDriverBuilder,
};

pub struct LedgerDriver(QldbDriver<QldbSessionSdk>);

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

    pub async fn apply(&self, _migration: &Migration) -> Result<bool, Box<dyn std::error::Error>> {
        todo!()
    }

    pub async fn check_migrations(&self) -> Result<(), Box<dyn std::error::Error>> {
        for stmt in [
            "create table _qldbx_migrations",
            "create index on _qldbx_migrations (utc)",
        ] {
            let _ = self.execute_statement(stmt).await?;
        }

        Ok(())
    }

    pub async fn list_migrations(&self) -> Result<StatementResults, Box<dyn std::error::Error>> {
        todo!()
    }

    pub async fn execute_statement(
        &self,
        stmt: &str,
    ) -> Result<StatementResults, Box<dyn std::error::Error>> {
        Ok(self
            .0
            .transact(|mut tx| async {
                let res = tx.execute_statement(stmt).await?;
                tx.commit(res).await
            })
            .await?)
    }
}
