//! Configuration loading: Telegram API credentials and XDG paths.
//!
//! TODO(Phase 4): implement `Config::load()`:
//! 1. Read `TG_API_ID` and `TG_API_HASH` from the environment, falling back
//!    to a `config.toml` in the XDG config directory (see [`directories`]).
//! 2. When either value is missing, return a clear error pointing the user to
//!    <https://my.telegram.org> for creating an application and issuing API
//!    credentials. Do not print the values themselves.
//! 3. Resolve the TDLib data directory under the XDG data directory, never
//!    inside the repository tree.
//!
//! Logging rule: secrets (API hash, phone numbers, codes, tokens, message
//! text) must never be logged at info level or above.

/// Telegram API credentials resolved from the environment or config file.
#[derive(Debug, Clone)]
pub struct Config {
    /// Numeric Telegram API id (`TG_API_ID`).
    pub api_id: i32,
    /// Telegram API hash (`TG_API_HASH`).
    pub api_hash: String,
}

// TODO(Phase 4): load from env vars or an XDG config file; fail with a
// helpful message pointing to https://my.telegram.org when missing.
// pub fn load() -> anyhow::Result<Config> { todo!() }
