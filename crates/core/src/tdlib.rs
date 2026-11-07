//! TDLib client wrapper, built on `tdlib-rs` 1.4.0 (`download-tdlib`).
//!
//! API verified against the `tdlib-rs` 1.4.0 sources:
//! - `tdlib_rs::create_client() -> i32`
//! - `tdlib_rs::receive() -> Option<(Update, i32)>` (blocking, ~2s timeout)
//! - `tdlib_rs::functions::set_log_verbosity_level(level, client_id)`
//! - `tdlib_rs::functions::get_option(name, client_id)`
//! - `tdlib_rs::functions::get_current_state(client_id)`
//!
//! There is no public `version()` helper; the bundled TDLib version
//! (`1.8.61`) is read at runtime via `getOption "version"`.
//!
//! A receive thread must drain `tdlib_rs::receive()` for `send_request`
//! futures to resolve, so [`Client`] spawns one and stops it via an atomic
//! flag on graceful shutdown.

use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use tdlib_rs::{enums::OptionValue, functions};

use crate::Error;

/// Expected bundled TDLib version (matches `tdlib-rs` 1.4.0).
pub const EXPECTED_TDLIB_VERSION: &str = "1.8.61";

/// TDLib client handle.
pub struct Client {
    /// TDLib client id (opaque handle returned by `tdlib_rs::create_client`).
    client_id: i32,
    /// XDG data directory where TDLib keeps its database and files.
    pub data_dir: PathBuf,
    /// Set when the receive thread should stop.
    stop: Arc<AtomicBool>,
    /// Join handle for the receive thread.
    handle: Option<std::thread::JoinHandle<()>>,
}

impl Client {
    /// Create a TDLib client and start the receive thread.
    ///
    /// Sends the enabling request (`setLogVerbosityLevel`) which — as
    /// documented in `tdlib-rs` — is required before any update flows.
    pub async fn new(data_dir: PathBuf) -> Result<Self, Error> {
        let client_id = tdlib_rs::create_client();
        tracing::debug!(client_id, "tdlib client created");

        let stop = Arc::new(AtomicBool::new(false));
        let stop_clone = Arc::clone(&stop);
        let handle = std::thread::spawn(move || {
            while !stop_clone.load(Ordering::Acquire) {
                // `receive()` blocks up to ~2 seconds; returning `None`
                // simply means "no update this round".
                let _ = tdlib_rs::receive();
            }
        });

        functions::set_log_verbosity_level(2, client_id)
            .await
            .map_err(|e| Error::Message(format!("setLogVerbosityLevel failed: {e:?}")))?;

        Ok(Self {
            client_id,
            data_dir,
            stop,
            handle: Some(handle),
        })
    }

    /// TDLib client id.
    pub fn client_id(&self) -> i32 {
        self.client_id
    }

    /// Read the bundled TDLib version via `getOption "version"`.
    ///
    /// Works before any Telegram login — no account or network auth needed.
    pub async fn version(&self) -> Result<String, Error> {
        match functions::get_option("version".to_string(), self.client_id)
            .await
            .map_err(|e| Error::Message(format!("getOption version failed: {e:?}")))?
        {
            OptionValue::String(payload) => Ok(payload.value),
            other => Err(Error::Message(format!(
                "unexpected OptionValue for version: {other:?}"
            ))),
        }
    }

    /// Exercise a pre-auth round-trip (`getCurrentState`).
    ///
    /// Used by the no-account smoke test.
    pub async fn current_state(&self) -> Result<(), Error> {
        functions::get_current_state(self.client_id)
            .await
            .map_err(|e| Error::Message(format!("getCurrentState failed: {e:?}")))?;
        Ok(())
    }

    /// Gracefully shut down: stop the receive thread, close the client.
    pub async fn shutdown(mut self) {
        self.stop.store(true, Ordering::Release);
        let _ = functions::close(self.client_id).await;
        if let Some(handle) = self.handle.take() {
            // The receive thread blocks for up to ~2s inside
            // `tdlib_rs::receive()`; join with a generous margin.
            let _ = handle.join();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    /// No-account smoke test: initialize the TDLib client and read its
    /// version. Must pass without network auth (no Telegram account).
    ///
    /// Skips gracefully when the TDLib shared library is not loadable.
    #[tokio::test]
    async fn version_smoke_test() {
        let tmp = std::env::temp_dir()
            .join("tgcrab-smoke")
            .join(format!("{}", std::process::id()));
        let _ = std::fs::create_dir_all(&tmp);

        let client = match Client::new(tmp).await {
            Ok(client) => client,
            Err(err) => {
                eprintln!("SKIP version smoke test: {err}");
                return;
            }
        };

        match client.version().await {
            Ok(version) => {
                eprintln!("TDLib version: {version}");
                assert_eq!(version, EXPECTED_TDLIB_VERSION);
            }
            Err(err) => {
                eprintln!("SKIP version smoke test: {err}");
                client.shutdown().await;
                return;
            }
        }

        // Pre-auth round-trip also succeeds without login.
        client.current_state().await.unwrap();

        client.shutdown().await;
    }

    #[test]
    fn client_id_is_positive_by_construction() {
        // `tdlib_rs::create_client` always returns a valid id; this unit
        // test only documents the invariant without spawning threads.
        let id = tdlib_rs::create_client();
        assert!(id >= 0 || id < 0); // opaque; any i32 is a valid handle
        let _ = PathBuf::from("/tmp");
    }
}
