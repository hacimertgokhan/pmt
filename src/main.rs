use std::io;
use clap::{Parser, Subcommand};
use crate::pid::list::{list_pids_by_process_name, list_used_pids};
use crate::port::kill::{kill_port_async, kill_process_by_name_and_port_async};
use crate::port::list;
use crate::port::monitor::run_monitor;

mod port {
    pub mod list;
    pub mod kill;
    pub mod monitor;
}

mod pid {
    pub mod list;
}

#[derive(Parser)]
#[command(
    name = "pmt",
    about = "Port management tool.",
    version = "1.0.0",
    author = "hacimertgokhan"
)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Pids {
        #[arg(required = false)]
        pid: Option<String>,
    },
    Ports,
    Monitor,
    KillPortByName {
        name: String,
        port: u16,
    },
    KillPortByU16 {
        port: u16,
    },
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = CLI::parse();

    match cli.command {
        Commands::KillPortByU16 { port } => {
            kill_port_async(port).await?;
        }
        Commands::Ports => {
            list::list_used_ports().await;
        }
        Commands::Pids { pid } => {
            return match pid {
                Some(process_name) => {
                    list_pids_by_process_name(&*process_name).await;
                    Ok(())
                }
                None => {
                    list_used_pids().await;
                    Ok(())
                }
            }
        }
        Commands::KillPortByName { name, port } => {
            kill_process_by_name_and_port_async(&name, port).await?;
        }
        Commands::Monitor => {
            run_monitor()?;
        }
    }

    Ok(())
}