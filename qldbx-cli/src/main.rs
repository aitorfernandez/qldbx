mod ledger;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, name = "qldbx")]
struct Cli {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    #[clap(alias = "l")]
    Ledger(LedgerCli),
}

#[derive(Parser, Debug)]
pub struct LedgerCli {
    #[clap(subcommand)]
    pub cmd: LedgerCommand,
}

#[derive(Parser, Debug)]
pub enum LedgerCommand {
    Create { name: String },
    Delete { name: String },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Ledger(l) => match l.cmd {
            LedgerCommand::Create { name } => ledger::create().await?,
            LedgerCommand::Delete { name } => ledger::delete().await?,
        },
    }

    Ok(())
}
