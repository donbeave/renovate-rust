//! Config presets module.
//!
//! Renovate reference: `lib/config/presets/`.

pub mod common;
pub mod forgejo;
pub mod gitea;
pub mod github;
pub mod gitlab;
pub mod http;
pub mod index;
pub mod internal;
pub mod local;
pub mod npm;
pub mod parse;
pub mod util;

pub use index::{PresetResult, replace_args, resolve_config_presets};
pub use parse::{PresetReference, parse_preset};
pub use util::{
    PRESET_DEP_NOT_FOUND, PRESET_INVALID, PRESET_INVALID_JSON, PRESET_NOT_FOUND,
    PRESET_PROHIBITED_SUBPRESET, PRESET_RENOVATE_CONFIG_NOT_FOUND,
};
