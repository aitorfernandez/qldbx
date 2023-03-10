use clap::Parser;
use qldbx::{
    cli::{Cli, Command, LedgerCommand, MigrateCommand},
    ledger, migrate,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let cli = Cli::parse();

    match cli.cmd {
        Command::Ledger(l) => match l.cmd {
            LedgerCommand::Create { connect_opts } => ledger::create(&connect_opts).await?,
            LedgerCommand::Delete { connect_opts } => ledger::delete(&connect_opts).await?,
            LedgerCommand::Reset { connect_opts } => ledger::reset(&connect_opts).await?,
        },
        Command::Migrate(m) => match m.cmd {
            MigrateCommand::Create { name } => migrate::create(&name).await?,
            MigrateCommand::Run { connect_opts } => migrate::run(&connect_opts).await?,
            MigrateCommand::Info { connect_opts } => migrate::info(&connect_opts).await?,
        },
    }

    Ok(())
}
