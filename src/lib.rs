//! The shared look: `ui/theme.slint` and `ui/widgets.slint`. An app's `build.rs` passes
//! [`library_paths`] to `slint_build`, and its screens import `@kitka/theme.slint` and
//! `@kitka/widgets.slint`.

use std::collections::HashMap;
use std::path::PathBuf;

/// The `@kitka` import path for `slint_build::CompilerConfiguration::with_library_paths`.
pub fn library_paths() -> HashMap<String, PathBuf> {
    HashMap::from([("kitka".into(), PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/ui")))])
}
