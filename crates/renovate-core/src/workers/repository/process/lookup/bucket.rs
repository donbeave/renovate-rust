//! Bucketing logic for lookup updates (decides 'major', 'minor', 'patch', 'non-major', 'latest', or version-specific bucket for grouping PRs).
//!
//! @parity `lib/workers/repository/process/lookup/bucket.ts` partial — getBucket (uses separate* flags from config + versioning getMajor/getMinor to decide bucket for update grouping); single test ported (covering it from lookup/index.spec that exercises separateMajorMinor etc for bucketing). Full callers in lookup/index or update/branchify pending other units.
//!
//! Mirrors `lib/workers/repository/process/lookup/bucket.ts`.

/// Port of getBucket.
/// Takes the separate* flags (from config), current/new version, and closures for the versioning api methods (to keep pure and avoid full trait dep for this unit).
pub fn get_bucket(
    separate_major_minor: bool,
    separate_multiple_major: bool,
    separate_multiple_minor: bool,
    separate_minor_patch: bool,
    current_version: &str,
    new_version: &str,
    get_major: impl Fn(&str) -> Option<u64>,
    get_minor: impl Fn(&str) -> Option<u64>,
) -> Option<String> {
    if !separate_major_minor {
        return Some("latest".to_string());
    }
    let from_major = get_major(current_version);
    let to_major = get_major(new_version);

    // istanbul ignore if: error case
    if to_major.is_none() {
        return None;
    }

    // Check for major update type first
    if from_major != to_major {
        if separate_multiple_major {
            return Some(format!("v{}", to_major.unwrap()));
        }
        // default path for major updates is not to separate them
        return Some("major".to_string());
    }

    // If we reach here then we know it's non-major
    let from_minor = get_minor(current_version);
    let to_minor = get_minor(new_version);

    // istanbul ignore if: error case
    if from_minor.is_none() || to_minor.is_none() {
        return Some("non-major".to_string());
    }

    // Check the minor update type first
    if from_minor != to_minor {
        if separate_multiple_minor {
            return Some(format!("v{}.{}", to_major.unwrap(), to_minor.unwrap()));
        }

        if separate_minor_patch {
            return Some("minor".to_string());
        }
        // default path for minor updates is not to separate them from patch
        return Some("non-major".to_string());
    }

    // If we reach here then we know it's a patch release

    /* future option
    if (separateMultiplePatch) {
      const toPatch = versioningApi.getPatch(newVersion);
      if (toPatch !== null && separateMultiplePatch) {
        return `v${toMajor}.${toMinor}.${toPatch}`;
      }
    }
    */

    if separate_minor_patch {
        return Some("patch".to_string());
    }
    // default path for patch updates is not to separate them from minor
    Some("non-major".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_multiple_updates_if_grouping_but_separate_major_minor_true() {
        // Ported: "returns multiple updates if grouping but separateMajorMinor=true" — lib/workers/repository/process/lookup/index.spec.ts line 310
        // (tests the bucketing decision for separateMajorMinor)
        let bucket = get_bucket(
            true,
            false,
            false,
            false,
            "1.0.0",
            "2.0.0",
            |v| {
                if v.starts_with("1.") {
                    Some(1)
                } else if v.starts_with("2.") {
                    Some(2)
                } else {
                    None
                }
            },
            |v| {
                if v.starts_with("1.0") {
                    Some(0)
                } else if v.starts_with("2.0") {
                    Some(0)
                } else {
                    None
                }
            },
        );
        assert_eq!(bucket, Some("major".to_string()));
    }
}
