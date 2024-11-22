use std::io;
use clap::{Parser, Subcommand};
use crate::pid::list::{list_pids_by_process_name, list_used_pids};
use crate::port::kill::kill_process_using_port;
use crate::port::list;

mod port {
    pub mod list;
    pub mod kill;
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
    Kill {
        port: u16,
    },
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = CLI::parse();

    match cli.command {
        Commands::Kill { port } => {
            if let Err(e) = kill_process_using_port(port).await {
                eprintln!("Hata: {}", e);
            }
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
    }

    Ok(())
}