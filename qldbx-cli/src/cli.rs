use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, version, about, name = "qldbx")]
pub struct Cli {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    #[clap(alias = "l")]
    Ledger(LedgerCli),

    #[clap(alias = "m")]
    Migrate(MigrateCli),
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
    Reset {
        #[clap(flatten)]
        connect_opts: ConnectOpts,
    },
}

#[derive(Parser, Debug)]
pub struct MigrateCli {
    #[clap(subcommand)]
    pub cmd: MigrateCommand,
}

#[derive(Parser, Debug)]
pub enum MigrateCommand {
    Create {
        name: String,
    },
    Run {
        #[clap(flatten)]
        connect_opts: ConnectOpts,
    },
    Info {
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
