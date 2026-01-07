#![cfg(all(not(feature = "no_semver"), feature = "semver"))]
//! Rhai semantic versioning utilities package.

#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
use crate::{def_package, plugin::*};

def_package! {
    /// Package of semantic versioning utilities.
    pub BasicSemverPackage(lib) {
        lib.set_standard_lib(true);
        combine_with_exported_module!(lib, "semver", semver_functions);
    }
}

#[export_module]
mod semver_functions {
    use crate::EvalAltResult;

    /// Increment a semantic version string (e.g., "1.2.3") by the specified part ("major", "minor", "patch").
    ///
    /// ```rhai
    /// let next_ver = semver_bump("1.2.3", "minor"); // "1.3.0"
    /// ```
    #[rhai_fn(return_raw)]
    pub fn semver_bump(version: &str, part: &str) -> Result<String, Box<EvalAltResult>> {
        let parts: Vec<u32> = version.split('.').map(|s| s.parse().unwrap_or(0)).collect();

        if parts.len() != 3 {
            return Err("Invalid semver format (must be X.Y.Z)".into());
        }

        let (major, minor, patch) = (parts[0], parts[1], parts[2]);

        let new_version = match part {
            "major" => format!(
                "{}.0.0",
                major.checked_add(1).ok_or("Major version overflow")?
            ),
            "minor" => format!(
                "{}.{}.0",
                major,
                minor.checked_add(1).ok_or("Minor version overflow")?
            ),
            "patch" => format!(
                "{}.{}.{}",
                major,
                minor,
                patch.checked_add(1).ok_or("Patch version overflow")?
            ),
            _ => {
                return Err(format!(
                    "Unknown version part: {part}. Expected 'major', 'minor', or 'patch'"
                )
                .into())
            }
        };

        Ok(new_version)
    }
}
