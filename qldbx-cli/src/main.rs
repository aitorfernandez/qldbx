mod cli;
mod ledger;
mod migrate;

use clap::Parser;
use cli::{Cli, Command, LedgerCommand, MigrateCommand};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.cmd {
        Command::Ledger(l) => match l.cmd {
            LedgerCommand::Create { connect_opts } => ledger::create(&connect_opts).await?,
            LedgerCommand::Delete { connect_opts } => ledger::delete(&connect_opts).await?,
        },
        Command::Migrate(m) => match m.cmd {
            MigrateCommand::Create { name } => migrate::create(&name).await?,
            MigrateCommand::Run { connect_opts } => migrate::run(&connect_opts).await?,
        },
    }

    Ok(())
}
