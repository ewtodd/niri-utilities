use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use niri_utilities::{commands, niri};

#[derive(Parser)]
#[command(name = "niri-utilities")]
#[command(about = "Utilities for the Niri window manager")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run the auto-center daemon
    CenteringDaemon,
    /// Generate shell completions
    Completions {
        #[arg(value_enum)]
        shell: Shell,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Completions { shell } => {
            clap_complete::generate(
                shell,
                &mut Cli::command(),
                "niri-utilities",
                &mut std::io::stdout(),
            );
            Ok(())
        }
        Commands::CenteringDaemon => {
            let socket = niri::connect()?;
            commands::centering_daemon(socket)
        }
    }
}
