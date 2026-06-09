#![allow(
    clippy::mod_module_files,
    reason = "Traditional mod.rs for deep directory structure of the partial TS port (@parity); mod_module_files=deny from strict jackin baseline but this is port debt. Will clean as modules complete."
)]
pub mod auto_replace;
pub mod automerge;
pub mod bump_versions;
pub mod check_existing;
pub mod commit;
pub mod execute_post_upgrade_commands;
pub mod get_updated;
pub mod handle_existing;
pub mod index;
