use clap::{Parser, Subcommand};
use colored::*;

fn banner() -> String {
    let banner = "
    ██████╗ ███████╗███╗   ███╗██╗███╗   ██╗██████╗ ███╗   ██╗███████╗██╗  ██╗ ██████╗ 
    ██╔══██╗██╔════╝████╗ ████║██║████╗  ██║██╔══██╗████╗  ██║██╔════╝██║ ██╔╝██╔═══██╗     ╱|、
    ██████╔╝█████╗  ██╔████╔██║██║██╔██╗ ██║██║  ██║██╔██╗ ██║█████╗  █████╔╝ ██║   ██║    (˚ˎ 。7
    ██╔══██╗██╔══╝  ██║╚██╔╝██║██║██║╚██╗██║██║  ██║██║╚██╗██║██╔══╝  ██╔═██╗ ██║   ██║     |、˜〵
    ██║  ██║███████╗██║ ╚═╝ ██║██║██║ ╚████║██████╔╝██║ ╚████║███████╗██║  ██╗╚██████╔╝     じしˍ,)ノ
    ╚═╝  ╚═╝╚══════╝╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝╚═════╝ ╚═╝  ╚═══╝╚══════╝╚═╝  ╚═╝ ╚═════╝ 
    ";
    banner.color(Color::Magenta).to_string()
}
#[derive(Debug, Parser)]
#[command(name = "RemindNeko", about = banner())]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum CliSubcommand {
    #[command(about = "set a reminder", alias = "new", alias = "n", alias = "add")]
    Set { name: String, time: String },

    #[command(about = "delete a reminder", alias = "rm", alias = "r")]
    Remove { name: String },

    #[command(about = "configure settings")]
    Settings { name: String, conf: String },

    #[command(about = "print current reminders")]
    List,

    #[command(about = "start a reminder")]
    Start { name: String },

    #[command(about = "stop a reminder")]
    Stop { name: String },
}
