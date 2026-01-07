#![cfg(all(not(feature = "no_env"), feature = "env"))]
//! Rhai scripting engine integration
//!
//! Provides a sandboxed scripting environment for complex task logic.
//! Rhai was chosen for its fast startup time, Rust-native integration,
//! and familiar syntax.

#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
use crate::def_package;
use crate::plugin::*;

def_package! {
    /// Package of basic filesystem utilities.
    pub BasicEnvironmentPackage(lib) {
        lib.set_standard_lib(true);

        // Register filesystem functions
        combine_with_exported_module!(lib, "env", env_functions);
    }
}

// File operations
#[export_module]
mod env_functions {
    use crate::Dynamic;

    /// Get the value of an environment variable. Returns an empty string if the variable is not set.
    ///
    /// ```rhai
    /// let path = get_env("PATH");
    /// ```
    #[rhai_fn()]
    pub fn get_env(key: &str) -> String {
        std::env::var(key).unwrap_or_default()
    }

    /// Set the value of an environment variable.
    ///
    /// ```rhai
    /// set_env("MY_VAR", "my_value");
    /// ```
    #[rhai_fn()]
    pub fn set_env(key: &str, value: &str) {
        std::env::set_var(key, value);
    }
}
