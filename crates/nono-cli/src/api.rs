//! Programmatic embedding surface for nono.
//!
//! This module exposes the profile parsing/validation and profile → capability
//! manifest compilation paths that the `nono` CLI uses internally, so other
//! tools can reuse them without depending on the CLI binary.
//!
//! # Example
//!
//! ```
//! use nono_cli::api::{compile_manifest, validate_profile};
//!
//! let json = br#"{ "meta": { "name": "demo" }, "filesystem": { "read": ["/tmp"] } }"#;
//! validate_profile(json).expect("profile should be valid");
//! let manifest = compile_manifest(json, std::path::Path::new("/tmp"))
//!     .expect("manifest should compile");
//! println!("{}", manifest.to_json().expect("serialize"));
//! ```

use std::path::Path;

use nono::manifest::CapabilityManifest;
use nono::{NonoError, Result};

use crate::profile;
use crate::profile_cmd;

/// Parse and validate a profile from raw bytes.
///
/// This performs the same parse and inheritance-resolution steps the CLI uses
/// before a profile is accepted: JSON parse via
/// [`profile::parse_profile_bytes`] followed by
/// [`profile::resolve_and_finalize_profile`] (which resolves `extends`, applies
/// defaults, and re-validates merged values).
///
/// Returns `Ok(())` when the profile is structurally valid and resolvable.
pub fn validate_profile(bytes: &[u8]) -> Result<()> {
    let raw = profile::parse_profile_bytes(bytes)?;
    profile::resolve_and_finalize_profile(raw)?;
    Ok(())
}

/// Compile a profile into a fully-resolved capability manifest.
///
/// `bytes` is parsed and resolved the same way as [`validate_profile`], then
/// compiled into a portable [`CapabilityManifest`] with absolute paths.
/// Environment-variable templates (`~`, `$HOME`, `$TMPDIR`, ...) are expanded
/// relative to `workdir`.
///
/// `workdir` is the base directory used to expand relative and templated
/// paths. Callers embedding nono should pass the directory the sandboxed
/// workload will run in; the CLI uses the process working directory.
pub fn compile_manifest(bytes: &[u8], workdir: &Path) -> Result<CapabilityManifest> {
    let raw = profile::parse_profile_bytes(bytes)?;
    let resolved = profile::resolve_and_finalize_profile(raw)?;

    if resolved.meta.name.is_empty() {
        return Err(NonoError::ProfileParse(
            "profile meta.name must not be empty".to_string(),
        ));
    }

    profile_cmd::resolve_to_manifest(&resolved, workdir)
}
