#![allow(
    clippy::mod_module_files,
    reason = "Traditional mod.rs for deep directory structure of the partial TS port (@parity); mod_module_files=deny from strict jackin baseline but this is port debt. Will clean as modules complete."
)]
pub mod fetch;
pub mod lookup;
pub mod sort;
pub mod vulnerabilities;
pub mod write;
