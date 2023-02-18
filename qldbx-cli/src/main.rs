mod ledger;

use clap::{Args, Parser, Subcommand};

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
    Create {
        #[clap(flatten)]
        connect_opts: ConnectOpts,
    },
    Delete {
        #[clap(flatten)]
        connect_opts: ConnectOpts,
    },
}

#[derive(Args, Debug)]
pub struct ConnectOpts {
    #[clap(long, short = 'U', env)]
    pub uri: String,

    #[clap(long, short = 'N', env)]
    pub name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.cmd {
        Command::Ledger(l) => match l.cmd {
            LedgerCommand::Create { connect_opts } => ledger::create(&connect_opts).await?,
            LedgerCommand::Delete { connect_opts } => ledger::delete(&connect_opts).await?,
        },
    }

    Ok(())
}
