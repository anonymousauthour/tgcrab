//! Configuration loading: Telegram API credentials and XDG paths.
//!
//! Credentials (`TG_API_ID` / `TG_API_HASH`) are read from the environment
//! first; when absent there, they are read from the config file at
//! `<config_dir>/tgcrab/config.toml`. The file is a minimal TOML subset:
//!
//! ```toml
//! api_id = 1234567
//! api_hash = "0123456789abcdef0123456789abcdef"
//! ```
//!
//! The TDLib data directory lives under the XDG data dir
//! (`<data_dir>/tgcrab/tdlib`), never inside the repository tree.
//!
//! Logging rule: secrets (API hash, phone numbers, codes, tokens, message
//! text) must never be logged at info level or above.

use std::fmt;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;

use crate::Error;

/// Application name used to locate XDG directories.
const APP_QUALIFIER: &str = "tgcrab";
#[allow(dead_code)]
const APP_NAME: &str = "tgcrab";

/// Environment variable names.
pub const API_ID_ENV: &str = "TG_API_ID";
pub const API_HASH_ENV: &str = "TG_API_HASH";

/// Telegram API credentials resolved from the environment or config file.
#[derive(Clone)]
pub struct Config {
    /// Numeric Telegram API id (`TG_API_ID`).
    pub api_id: i32,
    /// Telegram API hash (`TG_API_HASH`). Hidden from `Debug` output.
    pub api_hash: String,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("api_id", &self.api_id)
            .field("api_hash", &"<redacted>")
            .finish()
    }
}

/// XDG paths resolved to the app directories.
///
/// `ProjectDirs::config_dir()` / `data_dir()` already end with the app
/// subdirectory (`tgcrab`) on Linux, so:
/// - `config_path = <config_dir>/config.toml` (i.e. `~/.config/tgcrab/config.toml`)
/// - `tdlib_data_dir = <data_dir>/tdlib` (i.e. `~/.local/share/tgcrab/tdlib`)
#[derive(Debug, Clone)]
pub struct AppPaths {
    /// Path to the config file.
    pub config_path: PathBuf,
    /// Directory holding the TDLib database and files.
    pub tdlib_data_dir: PathBuf,
}

/// Build [`AppPaths`] from a config dir and a data dir.
///
/// Pure function; unit-tested below. The dirs are expected to already be
/// the app directories (as `ProjectDirs::config_dir()` returns them).
pub fn app_paths_from(config_dir: &Path, data_dir: &Path) -> AppPaths {
    AppPaths {
        config_path: config_dir.join("config.toml"),
        tdlib_data_dir: data_dir.join("tdlib"),
    }
}

/// Resolve [`AppPaths`] from the platform XDG directories.
pub fn app_paths() -> Result<AppPaths, Error> {
    let project_dirs = ProjectDirs::from("", "", APP_QUALIFIER)
        .ok_or(Error::MissingDirectory { what: "project" })?;
    Ok(app_paths_from(
        project_dirs.config_dir(),
        project_dirs.data_dir(),
    ))
}

/// Parse the minimal TOML-style config file.
///
/// Supports `api_id = 42` and `api_hash = "abc"` (with optional quote
/// styles and whitespace). Unknown keys are ignored. Returns only the
/// fields that were found so callers can distinguish "absent" from "empty".
#[derive(Debug, Default, Clone)]
pub struct RawFileConfig {
    /// Numeric API id as raw string (parsed later so errors are clear).
    pub api_id: Option<String>,
    /// API hash, trimmed of quotes and whitespace.
    pub api_hash: Option<String>,
}

/// Parse the text of a `config.toml` file.
///
/// Pure function; unit-tested below.
pub fn parse_config_text(text: &str) -> RawFileConfig {
    let mut raw = RawFileConfig::default();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim().trim_end_matches(',');
        let value = value
            .trim_start_matches('"')
            .trim_end_matches('"')
            .trim()
            .to_string();
        match key {
            "api_id" => raw.api_id = Some(value),
            "api_hash" => raw.api_hash = Some(value),
            _ => {}
        }
    }
    raw
}

/// Non-empty environment value lookup (whitespace-only counts as absent).
fn env_value(name: &str) -> Option<String> {
    std::env::var(name).ok().filter(|v| !v.trim().is_empty())
}

/// Parse and validate an API id string.
fn parse_api_id(value: &str) -> Result<i32, Error> {
    value
        .trim()
        .parse::<i32>()
        .map_err(|_| Error::InvalidApiId {
            got: value.to_owned(),
        })
}

/// Load credentials, preferring the environment over the config file.
///
/// Fails with a clear error pointing at https://my.telegram.org when
/// credentials are missing, and never logs the values.
pub fn load() -> Result<Config, Error> {
    let paths = app_paths()?;
    load_from(&paths)
}

/// Load credentials for explicit paths. Used by [`load`] and tests.
pub fn load_from(paths: &AppPaths) -> Result<Config, Error> {
    let env_id = env_value(API_ID_ENV);
    let env_hash = env_value(API_HASH_ENV);

    // Read the file only for fields missing from the environment.
    let mut file: Option<RawFileConfig> = None;
    if (env_id.is_none() || env_hash.is_none()) && paths.config_path.exists() {
        let text =
            std::fs::read_to_string(&paths.config_path).map_err(|source| Error::ConfigRead {
                path: paths.config_path.display().to_string(),
                source,
            })?;
        file = Some(parse_config_text(&text));
    }

    let api_id_str = env_id.or_else(|| file.as_ref().and_then(|f| f.api_id.clone()));
    let api_hash = env_hash.or_else(|| file.as_ref().and_then(|f| f.api_hash.clone()));

    let mut missing: Vec<&'static str> = Vec::new();
    if api_id_str.as_deref().map_or(true, |v| v.trim().is_empty()) {
        missing.push(API_ID_ENV);
    }
    if api_hash.as_deref().map_or(true, |v| v.trim().is_empty()) {
        missing.push(API_HASH_ENV);
    }
    if !missing.is_empty() {
        return Err(Error::MissingCredentials {
            missing: missing.join(", "),
            path: paths.config_path.display().to_string(),
        });
    }

    let api_id = parse_api_id(api_id_str.as_deref().unwrap())?;
    let api_hash = api_hash.unwrap().trim().to_owned();
    tracing::debug!(api_id, "telegram api credentials loaded");
    Ok(Config { api_id, api_hash })
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

    /// Restore environment after a test mutates it.
    struct EnvGuard {
        id: Option<String>,
        hash: Option<String>,
    }

    impl EnvGuard {
        fn take(name: &str) -> Option<String> {
            std::env::var(name).ok()
        }

        fn capture() -> Self {
            Self {
                id: Self::take(API_ID_ENV),
                hash: Self::take(API_HASH_ENV),
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            match &self.id {
                Some(v) => std::env::set_var(API_ID_ENV, v),
                None => std::env::remove_var(API_ID_ENV),
            }
            match &self.hash {
                Some(v) => std::env::set_var(API_HASH_ENV, v),
                None => std::env::remove_var(API_HASH_ENV),
            }
        }
    }

    #[test]
    fn paths_use_app_subdir() {
        let paths = app_paths_from(Path::new("/tmp/xdg/tgcrab"), Path::new("/tmp/data/tgcrab"));
        assert_eq!(paths.config_path, Path::new("/tmp/xdg/tgcrab/config.toml"));
        assert_eq!(paths.tdlib_data_dir, Path::new("/tmp/data/tgcrab/tdlib"));
    }

    #[test]
    fn parse_quoted_and_bare_values() {
        let text = r#"
            # comment
            api_id = 1234567
            api_hash = "0123456789abcdef0123456789abcdef"
            unknown = ignored
        "#;
        let raw = parse_config_text(text);
        assert_eq!(raw.api_id.as_deref(), Some("1234567"));
        assert_eq!(
            raw.api_hash.as_deref(),
            Some("0123456789abcdef0123456789abcdef")
        );
    }

    #[test]
    fn parse_ignores_blank_and_comments() {
        let raw = parse_config_text("\n\n# nothing\n");
        assert_eq!(raw.api_id, None);
    }

    #[test]
    fn debug_redacts_api_hash() {
        let config = Config {
            api_id: 1,
            api_hash: "supersecret".to_string(),
        };
        let rendered = format!("{config:?}");
        assert!(!rendered.contains("supersecret"));
        assert!(rendered.contains("<redacted>"));
    }

    #[test]
    fn missing_credentials_points_at_telegram() {
        let _guard = env_lock().lock().unwrap();
        let _env = EnvGuard::capture();
        std::env::remove_var(API_ID_ENV);
        std::env::remove_var(API_HASH_ENV);

        let paths = AppPaths {
            config_path: PathBuf::from("/definitely/not/here/config.toml"),
            tdlib_data_dir: PathBuf::from("/tmp/data/tgcrab/tdlib"),
        };
        let err = load_from(&paths).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("TG_API_ID"));
        assert!(msg.contains("TG_API_HASH"));
        assert!(msg.contains("my.telegram.org"));
        assert!(matches!(err, Error::MissingCredentials { .. }));
    }

    #[test]
    fn invalid_api_id_is_rejected() {
        let _guard = env_lock().lock().unwrap();
        let _env = EnvGuard::capture();
        std::env::set_var(API_ID_ENV, "not-a-number");
        std::env::set_var(API_HASH_ENV, "0123456789abcdef0123456789abcdef");

        let paths = AppPaths {
            config_path: PathBuf::from("/definitely/not/here/config.toml"),
            tdlib_data_dir: PathBuf::from("/tmp/data/tgcrab/tdlib"),
        };
        let err = load_from(&paths).unwrap_err();
        assert!(matches!(err, Error::InvalidApiId { .. }));
    }

    #[test]
    fn env_credentials_load_successfully() {
        let _guard = env_lock().lock().unwrap();
        let _env = EnvGuard::capture();
        std::env::set_var(API_ID_ENV, "7654321");
        std::env::set_var(API_HASH_ENV, "0123456789abcdef0123456789abcdef");

        let paths = AppPaths {
            config_path: PathBuf::from("/definitely/not/here/config.toml"),
            tdlib_data_dir: PathBuf::from("/tmp/data/tgcrab/tdlib"),
        };
        let config = load_from(&paths).unwrap();
        assert_eq!(config.api_id, 7654321);
        assert_eq!(config.api_hash, "0123456789abcdef0123456789abcdef");
    }
}
