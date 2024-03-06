use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "RemindNeko")]
#[command(about = "RemindNeko *should* remind you")]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CliSubcommand {
    #[command(about = "set a reminder")]
    Set { name: String, desc: String },

    #[command(about = "delete a reminder")]
    Remove { name: String },

    #[command(about = "configure settings")]
    Settings { name: String, conf: String },

    #[command(about = "start a reminder")]
    Start { name: String },

    #[command(about = "stop a reminder")]
    Stop { name: String },
}
