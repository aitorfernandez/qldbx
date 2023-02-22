use crate::{Migration, MigrationLedger};
use amazon_qldb_driver_core::{
    api::QldbSessionSdk,
    aws_sdk_qldbsession::{Config, Endpoint},
    ion_compat,
    transaction::StatementResults,
    QldbDriver, QldbDriverBuilder,
};

pub struct LedgerDriver(QldbDriver<QldbSessionSdk>);

impl LedgerDriver {
    /// Create a new LedgerDriver using amazon_qldb_driver_core using the uri and name from the .env file.
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

    /// Check if table '_qldbx_migrations' exists and create if it does not exist.
    pub async fn check_migrations(&self) -> Result<(), Box<dyn std::error::Error>> {
        let stmt =
            "select name from information_schema.user_tables where name = '_qldbx_migrations'";
        if self.execute_statement(stmt).await?.len() == 0 {
            for stmt in [
                "create table _qldbx_migrations",
                "create index on _qldbx_migrations (utc)",
            ] {
                self.execute_statement(stmt).await?;
            }
        }

        Ok(())
    }

    /// Select all migrations from '_qldbx_migrations'.
    pub async fn list_migrations(
        &self,
    ) -> Result<Vec<MigrationLedger>, Box<dyn std::error::Error>> {
        let stmt = "select * from _qldbx_migrations";
        let res = self.execute_statement(stmt).await?;

        Ok(res
            .readers()
            .map(|r| ion_compat::to_string_pretty(r?)?.try_into())
            .collect::<Result<_, _>>()?)
    }

    /// Prepares the statement for insert the migration.
    pub async fn apply(&self, migration: &Migration) -> Result<(), Box<dyn std::error::Error>> {
        self.execute_statement(&migration.statement.clone()).await?;

        let stmt = format!("insert into _qldbx_migrations `{migration}`");
        self.execute_statement(&stmt).await?;

        Ok(())
    }

    /// Executes the statement passed.
    async fn execute_statement(
        &self,
        stmt: &str,
    ) -> Result<StatementResults, Box<dyn std::error::Error>> {
        println!("> Executing statement {stmt}");
        Ok(self
            .0
            .transact(|mut tx| async {
                let res = tx.execute_statement(stmt).await?;
                tx.commit(res).await
            })
            .await?)
    }
}
