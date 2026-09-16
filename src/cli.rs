use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    name = "mini",
    about = "A small, Linux-first container orchestrator (learning project)",
    version
)]
pub struct Cli {
    /// Override the data directory (defaults to $MINI_DATA_DIR or the platform data dir).
    #[arg(long, global = true)]
    pub data_dir: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create or update a workload from a manifest file.
    Apply { file: PathBuf },
    /// List known workloads.
    Workloads,
    /// List known nodes.
    Nodes,
    /// List known instances.
    Instances,
    /// Show logs for an instance.
    Logs { instance_id: String },
    /// Show the full record for a workload (by id or name).
    Describe { id: String },
    /// Delete a workload (by id or name).
    Delete { id: String },
}
