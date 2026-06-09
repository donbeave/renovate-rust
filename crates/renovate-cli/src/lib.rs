#![expect(
    dead_code,
    unused,
    unreachable_pub,
    unused_qualifications,
    let_underscore_drop,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    reason = "Large partial port from Renovate TypeScript (@parity tags). Strict workspace lints enforced; targeted expect for port debt. Remove as completed."
)]

pub mod config_builder;
pub mod config_env;
