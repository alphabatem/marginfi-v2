use crate::commands::{
    backfill::{backfill, BackfillConfig},
    create_table::{create_table},
    index_transactions::{index_transactions, IndexTransactionsConfig},
};
use anyhow::Result;
use clap::Parser;
use dotenv::dotenv;
use envconfig::Envconfig;
use log::debug;
use std::{panic, process};
use crate::commands::create_table::TableType;
use crate::commands::index_accounts::{index_accounts, IndexAccountsConfig};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Parser)]
pub struct GlobalOptions {}

#[derive(Debug, Parser)]
#[clap(version = VERSION)]
pub struct Opts {
    #[clap(flatten)]
    pub global_config: GlobalOptions,
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    CreateTable {
        #[clap(long)]
        table_type: TableType,
        #[clap(long)]
        project_id: String,
        #[clap(long)]
        dataset_id: String,
        #[clap(long)]
        table_id: String,
        #[clap(long)]
        table_friendly_name: Option<String>,
        #[clap(long)]
        table_description: Option<String>,
    },
    Backfill,
    IndexTransactions,
    IndexAccounts,
}

#[tokio::main]
pub async fn entry(opts: Opts) -> Result<()> {
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        orig_hook(panic_info);
        process::exit(1);
    }));

    dotenv().ok();
    env_logger::init();

    match opts.command {
        Command::CreateTable { project_id, dataset_id, table_type, table_id, table_friendly_name, table_description } => {
            create_table(project_id, dataset_id, table_id, table_type, table_friendly_name, table_description).await
        }
        Command::Backfill => {
            let config = BackfillConfig::init_from_env().unwrap();
            debug!("Config -> {:#?}", &config.clone());

            backfill(config).await
        }
        Command::IndexTransactions => {
            let config = IndexTransactionsConfig::init_from_env().unwrap();
            debug!("Config -> {:#?}", &config.clone());

            index_transactions(config).await
        }
        Command::IndexAccounts => {
            let config = IndexAccountsConfig::init_from_env().unwrap();
            debug!("Config -> {:#?}", &config.clone());

            index_accounts(config).await
        }
    }
}
