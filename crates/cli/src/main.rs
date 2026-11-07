//! tgcrab CLI: unofficial Telegram client for the terminal.
//!
//! Phase 5 smoke run: `--version` and `--help` work, with a placeholder list
//! of subcommands. Real subcommands (login, chats, history, send, me,
//! logout) land in later milestones.
//!
//! Startup validates Telegram API credentials (Phase 4) so misconfiguration
//! fails with a clear message pointing at https://my.telegram.org instead of
//! a cryptic TDLib error. The check is deferred behind `--help`/`--version`
//! handling so those keep working offline.

use clap::{Parser, Subcommand};

use tgcrab_core::config;

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

fn init_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
}

fn main() {
    // Parse first so `--help` / `--version` exit before any config check
    // (clap handles those during `parse` via early exit).
    let cli = Cli::parse();

    init_tracing();
    tracing::debug!("tgcrab starting");

    // Startup config validation (Phase 4): fail fast with a clear message
    // when API credentials are missing. Placeholder subcommands only need
    // this check when they actually talk to TDLib; for now we validate
    // eagerly so operators notice misconfiguration immediately.
    if let Err(err) = config::load() {
        tracing::error!("{err}");
        eprintln!("error: {err}");
        std::process::exit(1);
    }
    tracing::debug!("telegram api credentials present");

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
