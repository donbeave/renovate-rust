//! Logging initialization, mirroring Renovate's `LOG_LEVEL` and `LOG_FORMAT`
//! environment variable semantics from `lib/logger/`.
//!
//! ## Renovate compatibility
//!
//! | Renovate env var | Behavior | Notes |
//! |---|---|---|
//! | `LOG_LEVEL` | Stdout log level; default `info` | `fatal` maps to `error` — tracing has no `fatal` level |
//! | `LOG_FORMAT=json` | JSON log output | Any other value (or absent) → human-readable |
//!
//! ## Color and TTY handling
//!
//! ANSI color is enabled by default only when stderr is a TTY. `NO_COLOR=1`
//! (or any non-empty value) disables it, as per <https://no-color.org/>.
//! A future `--log-color` / `--no-log-color` flag can override this.

use std::io::IsTerminal as _;

use tracing::Level;
use tracing_subscriber::fmt;

/// Outcome of [`parse_log_level`].
pub(crate) enum ParseLevelResult {
    /// A valid Renovate level name, resolved to the corresponding tracing level.
    Valid(Level),
    /// The name was not a valid Renovate log level.
    Invalid(String),
}

/// Parse a Renovate-compatible log-level name.
///
/// Renovate's valid names are `trace`, `debug`, `info`, `warn`, `error`, and
/// `fatal`. `fatal` is Bunyan-specific; it maps to [`Level::ERROR`] here since
/// tracing has no level above `error`.
///
/// Invalid input returns [`ParseLevelResult::Invalid`] so the caller can emit
/// an error and exit 1, matching Renovate's `validateLogLevel` behavior.
pub(crate) fn parse_log_level(s: &str) -> ParseLevelResult {
    match s {
        "trace" => ParseLevelResult::Valid(Level::TRACE),
        "debug" => ParseLevelResult::Valid(Level::DEBUG),
        "info" => ParseLevelResult::Valid(Level::INFO),
        "warn" => ParseLevelResult::Valid(Level::WARN),
        // `fatal` is Bunyan-only; map to error to preserve severity ordering.
        "error" | "fatal" => ParseLevelResult::Valid(Level::ERROR),
        other => ParseLevelResult::Invalid(other.to_owned()),
    }
}

/// Returns whether to use ANSI colors for log output.
///
/// Rules (in priority order):
/// 1. `NO_COLOR` set to any non-empty value → no color.
/// 2. stderr is not a TTY → no color.
/// 3. Otherwise → color.
fn should_use_ansi() -> bool {
    // https://no-color.org/: any non-empty NO_COLOR value disables color.
    if std::env::var("NO_COLOR")
        .map(|v| !v.is_empty())
        .unwrap_or(false)
    {
        return false;
    }
    std::io::stderr().is_terminal()
}

/// Outcome of a logging initialization attempt.
pub(crate) enum InitResult {
    /// Subscriber initialized successfully.
    Ok,
    /// `LOG_LEVEL` was set to an unrecognized value; caller should exit 1.
    InvalidLevel(String),
}

/// Initialize the global `tracing` subscriber from environment variables.
///
/// Must be called once, early in `main`, before any `tracing` macros run.
/// Calling it a second time is a no-op because `set_global_default` will
/// return an error that we intentionally ignore (the subscriber is already
/// in place).
pub(crate) fn init() -> InitResult {
    let level_str = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_owned());

    let level = match parse_log_level(&level_str) {
        ParseLevelResult::Valid(l) => l,
        ParseLevelResult::Invalid(s) => return InitResult::InvalidLevel(s),
    };

    let json_mode = std::env::var("LOG_FORMAT")
        .map(|v| v == "json")
        .unwrap_or(false);
    let use_ansi = should_use_ansi();

    if json_mode {
        let _ = fmt()
            .json()
            .with_writer(std::io::stderr)
            .with_max_level(level)
            .try_init();
    } else {
        let _ = fmt()
            .with_writer(std::io::stderr)
            .with_ansi(use_ansi)
            .with_max_level(level)
            .try_init();
    }

    InitResult::Ok
}

#[cfg(test)]
mod tests {
    use tracing::Level;

    use super::{ParseLevelResult, parse_log_level};

    fn level(s: &str) -> Option<Level> {
        match parse_log_level(s) {
            ParseLevelResult::Valid(l) => Some(l),
            ParseLevelResult::Invalid(_) => None,
        }
    }

    #[test]
    fn parses_all_valid_renovate_levels() {
        assert_eq!(level("trace"), Some(Level::TRACE));
        assert_eq!(level("debug"), Some(Level::DEBUG));
        assert_eq!(level("info"), Some(Level::INFO));
        assert_eq!(level("warn"), Some(Level::WARN));
        assert_eq!(level("error"), Some(Level::ERROR));
    }

    #[test]
    fn fatal_maps_to_error() {
        // Bunyan-specific level — no tracing equivalent above error.
        assert_eq!(level("fatal"), Some(Level::ERROR));
    }

    #[test]
    fn invalid_level_returns_none() {
        assert!(level("WARN").is_none()); // case-sensitive, like Renovate
        assert!(level("INFO").is_none());
        assert!(level("verbose").is_none());
        assert!(level("").is_none());
        assert!(level("0").is_none());
    }
}
