//! tgcrab-core: core logic for the unofficial Telegram client for the terminal.
//!
//! The crate contains pure, testable logic: configuration loading, secrets
//! handling, and (later) a thin wrapper around TDLib. No UI code lives here.
//!
//! Module map:
//! - [`config`]: loading `TG_API_ID` / `TG_API_HASH` from the environment or
//!   from the XDG config directory (via the `directories` crate), plus
//!   locating the TDLib data directory under the XDG data directory.
//! - [`secrets`]: database encryption key storage. The preferred store is the
//!   OS keyring (Secret Service); when unavailable, callers fall back to the
//!   `TGCRAB_DB_KEY` environment variable.
//! - [`tdlib`]: the TDLib client wrapper. It is a stub for now; the real
//!   wrapper (built on `tdlib-rs`) lands in Phase 3.
//!
//! This crate is unofficial and not affiliated with Telegram.

pub mod config;
pub mod secrets;
pub mod tdlib;

/// Crate-wide error type.
///
/// TODO(Phase 4): add real variants (missing credentials, keyring errors).
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Placeholder variant for the scaffold period.
    #[error("core error: {0}")]
    Message(String),
}
