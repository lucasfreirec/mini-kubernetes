// Node/Instance model types are defined now (Stage 0) but only consumed starting Stage 1/4.
#![allow(dead_code, unused_imports)]

mod cli;
mod commands;
mod config;
mod model;
mod store;

use clap::Parser;
use cli::{Cli, Command};
use config::Config;
use store::WorkloadStore;
use tracing_subscriber::EnvFilter;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let cli = Cli::parse();
    let config = Config::resolve(cli.data_dir.clone());
    let store = WorkloadStore::open(&config.data_dir)?;

    match cli.command {
        Command::Apply { file } => commands::apply(&store, &file)?,
        Command::Workloads => commands::list_workloads(&store)?,
        Command::Describe { id } => commands::describe(&store, &id)?,
        Command::Delete { id } => commands::delete(&store, &id)?,
        Command::Nodes => commands::stub("nodes", "Stage 4 (node agent)"),
        Command::Instances => commands::stub("instances", "Stage 1 (process supervisor)"),
        Command::Logs { instance_id: _ } => {
            commands::stub("logs", "Stage 1 (process supervisor)")
        }
    }

    Ok(())
}
