//! Secrets handling: the TDLib database encryption key.
//!
//! Plan (Phase 4):
//! - On first run, generate a random database encryption key.
//! - Store it in the OS keyring (Secret Service / gnome-keyring / KWallet).
//! - When no keyring is reachable, fall back to the clearly documented
//!   `TGCRAB_DB_KEY` environment variable.
//! - Never write the key into the repository or into logs.
//!
//! `setTdlibParameters` in TDLib accepts the key as `database_encryption_key`
//! (the TL `bytes` type maps to a `String` in `tdlib-rs`).

/// Environment variable used as a fallback when no OS keyring is available.
pub const DB_KEY_ENV: &str = "TGCRAB_DB_KEY";

/// Load or create the database encryption key.
///
/// TODO(Phase 4): check the keyring first; if that fails, read
/// [`DB_KEY_ENV`]; if that is unset, generate a random key and persist it via
/// the keyring. Never log the key.
pub fn database_key() -> Result<String, crate::Error> {
    Ok(String::new())
}
