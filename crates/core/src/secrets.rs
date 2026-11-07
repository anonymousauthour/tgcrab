//! Secrets handling: the TDLib database encryption key.
//!
//! Resolution order (Phase 4):
//! - On first run, generate a random database encryption key.
//! - Store it in the OS keyring (Secret Service / gnome-keyring / KWallet).
//! - When no keyring is reachable, fall back to the clearly documented
//!   `TGCRAB_DB_KEY` environment variable.
//! - Never write the key into the repository or into logs.
//!
//! `setTdlibParameters` in TDLib accepts the key as `database_encryption_key`
//! (the TL `bytes` type maps to a `String` in `tdlib-rs`).

use rand::RngCore as _;

use crate::Error;

/// Environment variable used as a fallback when no OS keyring is available.
pub const DB_KEY_ENV: &str = "TGCRAB_DB_KEY";
/// Service name used in the OS keyring.
pub const KEYRING_SERVICE: &str = "tgcrab";
/// Account / key name used in the OS keyring.
pub const KEYRING_ACCOUNT: &str = "db-encryption-key";

/// Generate a random database encryption key.
///
/// The TDLib `bytes` type maps to a hex string in `tdlib-rs`, so we encode
/// 32 random bytes as 64 hex characters.
pub fn generate_key() -> String {
    let mut bytes = [0u8; 32];
    rand::rng().fill_bytes(&mut bytes);
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

/// Read the key from the OS keyring, if one is reachable and holds a key.
pub fn keyring_get() -> Option<String> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_ACCOUNT).ok()?;
    entry.get_password().ok().filter(|v| !v.trim().is_empty())
}

/// Store the key in the OS keyring. Returns `Err` when unavailable.
pub fn keyring_set(key: &str) -> Result<(), keyring::Error> {
    let entry = keyring::Entry::new(KEYRING_SERVICE, KEYRING_ACCOUNT)?;
    entry.set_password(key)
}

/// Read the key from the fallback environment variable.
pub fn env_key() -> Option<String> {
    std::env::var(DB_KEY_ENV)
        .ok()
        .filter(|v| !v.trim().is_empty())
        .map(|v| v.trim().to_owned())
}

/// Load or create the database encryption key.
///
/// Order:
/// 1. Existing keyring entry (normal path once the key was stored).
/// 2. [`DB_KEY_ENV`] fallback (active path on hosts without a Secret Service).
/// 3. Freshly generated key, stored in the keyring when available.
///
/// Never logs the key.
pub fn database_key() -> Result<String, Error> {
    // 1. Already stored in the keyring.
    if let Some(key) = keyring_get() {
        tracing::debug!("database key loaded from os keyring");
        return Ok(key);
    }

    // 2. Documented environment fallback.
    if let Some(key) = env_key() {
        tracing::debug!(
            env = DB_KEY_ENV,
            "database key loaded from environment fallback"
        );
        return Ok(key);
    }

    // 3. First run: generate and persist via the keyring.
    let key = generate_key();
    match keyring_set(&key) {
        Ok(()) => tracing::debug!("generated and stored database key in os keyring"),
        Err(err) => tracing::debug!(?err, "os keyring unavailable; generated key not persisted"),
    }
    Ok(key)
}

/// Error diagnostic that never contains the key itself.
pub fn no_key_message() -> String {
    Error::NoDatabaseKey.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};

    /// Serializes tests that mutate process environment variables.
    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    struct EnvGuard {
        value: Option<String>,
    }

    impl EnvGuard {
        fn capture() -> Self {
            Self {
                value: std::env::var(DB_KEY_ENV).ok(),
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.value {
                Some(v) => std::env::set_var(DB_KEY_ENV, v),
                None => std::env::remove_var(DB_KEY_ENV),
            }
        }
    }

    #[test]
    fn generated_key_is_64_hex_chars() {
        let key = generate_key();
        assert_eq!(key.len(), 64);
        assert!(key.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn generated_keys_are_random() {
        assert_ne!(generate_key(), generate_key());
    }

    #[test]
    fn env_fallback_is_used_when_set() {
        let _guard = env_lock().lock().unwrap();
        let _env = EnvGuard::capture();
        std::env::set_var(DB_KEY_ENV, "deadbeefcafe");

        // Only assert the env helper when no keyring holds a key (on this
        // host there is no Secret Service, so the fallback is the active
        // path); otherwise the keyring path legitimately wins.
        if keyring_get().is_none() {
            assert_eq!(env_key().as_deref(), Some("deadbeefcafe"));
            let key = database_key().unwrap();
            assert_eq!(key, "deadbeefcafe");
        }
    }

    #[test]
    fn no_key_error_message_hides_key() {
        let msg = no_key_message();
        assert!(msg.contains(DB_KEY_ENV));
    }
}
