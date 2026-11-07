//! tgcrab-core: core logic for the unofficial Telegram client for the terminal.
//!
//! The crate contains pure, testable logic: configuration loading, secrets
//! handling, and a thin wrapper around TDLib. No UI code lives here.
//!
//! Module map:
//! - [`config`]: loading `TG_API_ID` / `TG_API_HASH` from the environment or
//!   from the XDG config directory (via the `directories` crate), plus
//!   locating the TDLib data directory under the XDG data directory.
//! - [`secrets`]: database encryption key storage. The preferred store is the
//!   OS keyring (Secret Service); when unavailable, callers fall back to the
//!   `TGCRAB_DB_KEY` environment variable.
//! - [`tdlib`]: the TDLib client wrapper, built on `tdlib-rs` (`download-tdlib`
//!   feature). Verified against `tdlib-rs` 1.4.0 sources (TDLib 1.8.61).
//!
//! This crate is unofficial and not affiliated with Telegram.

pub mod config;
pub mod secrets;
pub mod tdlib;

/// Crate-wide error type.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// A generic message (used for simple aborts; prefer a typed variant).
    #[error("{0}")]
    Message(String),
    /// Telegram API credentials could not be resolved from the environment
    /// or the config file. `missing` names the absent variables.
    #[error(
        "Missing Telegram API credentials ({missing}). \
         Get API credentials at https://my.telegram.org and set \
         TG_API_ID / TG_API_HASH in the environment or in {path}"
    )]
    MissingCredentials {
        /// Comma-separated list of missing variable names.
        missing: String,
        /// Where to put them (config file path).
        path: String,
    },
    /// `TG_API_ID` was present but not a valid integer.
    #[error(
        "TG_API_ID must be a number (got {got:?}). \
         Get API credentials at https://my.telegram.org"
    )]
    InvalidApiId {
        /// The offending value. It is an id, not a secret.
        got: String,
    },
    /// The XDG directories could not be resolved on this platform.
    #[error("could not resolve {what} directory on this platform")]
    MissingDirectory {
        /// Which directory (`config` or `data`).
        what: &'static str,
    },
    /// An I/O error while reading the config file.
    #[error("failed to read config file {path}: {source}")]
    ConfigRead {
        /// Config file path.
        path: String,
        /// Underlying error.
        #[source]
        source: std::io::Error,
    },
    /// No database encryption key source is available.
    #[error(
        "No database encryption key available: the OS keyring is unreachable \
         and the TGCRAB_DB_KEY environment variable is not set. \
         Set TGCRAB_DB_KEY to your database encryption key."
    )]
    NoDatabaseKey,
}
