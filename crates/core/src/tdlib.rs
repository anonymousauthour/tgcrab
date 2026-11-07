//! TDLib client wrapper (stub for now).
//!
//! Phase 3 will implement a real wrapper built on `tdlib-rs = "1.4.0"` with
//! the `download-tdlib` feature: create the client (`tdlib_rs::create_client`),
//! run the blocking receive loop (`tdlib_rs::receive`), and read the bundled
//! TDLib version via `functions::get_option("version", client_id)` (there is
//! no public `version()` helper; the bundled version is TDLib 1.8.61).
//!
//! The no-account smoke test will exercise client creation plus
//! `getCurrentState` without any Telegram login.

use std::path::PathBuf;

/// Placeholder client handle for the TDLib wrapper.
pub struct Client {
    /// XDG data directory where TDLib keeps its database and files.
    pub data_dir: PathBuf,
}

// TODO(Phase 3): add a constructor that creates a TDLib client session and
// a receive loop drained via `tdlib_rs::receive()`.
