//! tgcrab CLI: unofficial Telegram client for the terminal.
//!
//! Phase 5 smoke run: `--version` and `--help` work, with a placeholder list
//! of subcommands. Real subcommands (login, chats, history, send, me,
//! logout) land in later milestones.

use clap::{Parser, Subcommand};

/// Unofficial Telegram client for the terminal.
#[derive(Debug, Parser)]
#[command(
    name = "tgcrab",
    version,
    about = "Unofficial Telegram client for the terminal"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Placeholder login command (milestone 1).
    Login,
    /// Placeholder: list chats (milestone 2).
    Chats,
    /// Placeholder: show chat history (milestone 2).
    History,
    /// Placeholder: send a message (milestone 2).
    Send,
    /// Placeholder: show the current account (milestone 2).
    Me,
    /// Placeholder: log out (milestone 2).
    Logout,
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    match cli.command {
        Some(cmd) => {
            tracing::info!(?cmd, "subcommand not implemented yet");
            println!("`{cmd:?}` is a placeholder and is not implemented yet.");
        }
        None => {
            // `clap` prints usage; keep a friendly hint when run bare.
            Cli::parse();
        }
    }
}
