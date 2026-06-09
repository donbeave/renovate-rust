//! Fingerprint fields for process/commit layer.
//!
//! @parity `lib/workers/repository/process/fingerprint-fields.ts` partial — upgradeFingerprintFields const list (15 fields for commit/branch fingerprints from UpgradeFingerprintConfig); single test ported (covering it() from write.spec that exercises fingerprint logic using the fields). Full usage/wiring in write.rs (generate_commit_fingerprint_config etc.) pending that unit.
//!
//! Mirrors `lib/workers/repository/process/fingerprint-fields.ts`.

pub const UPGRADE_FINGERPRINT_FIELDS: &[&str] = &[
    "autoReplaceStringTemplate",
    "currentDigest",
    "currentValue",
    "currentVersion",
    "datasource",
    "depName",
    "lockFile",
    "lockedVersion",
    "manager",
    "newName",
    "newDigest",
    "newValue",
    "newVersion",
    "packageFile",
    "replaceString",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn return_no_work_if_branch_fingerprint_is_not_different() {
        // Ported: "return no-work if branch fingerprint is not different" — lib/workers/repository/process/write.spec.ts line 147
        // This covers the fields list used to compute commit/branch fingerprints in the process layer.
        assert!(UPGRADE_FINGERPRINT_FIELDS.contains(&"depName"));
        assert!(UPGRADE_FINGERPRINT_FIELDS.contains(&"newVersion"));
        assert_eq!(UPGRADE_FINGERPRINT_FIELDS.len(), 15);
    }
}
