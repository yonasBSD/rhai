#![cfg(all(not(feature = "no_shell"), feature = "shell"))]
//! Rhai shell command execution utilities package.

#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
use crate::{def_package, plugin::*};

def_package! {
    /// Package of shell execution utilities.
    pub BasicShellPackage(lib) {
        lib.set_standard_lib(true);
        combine_with_exported_module!(lib, "shell", shell_functions);
    }
}

#[export_module]
mod shell_functions {
    use crate::EvalAltResult;

    /// Execute a shell command and return its stdout as a string.
    /// Errors if the command fails or returns a non-zero exit code.
    ///
    /// ```rhai
    /// let output = exec("ls -a");
    /// print(output);
    /// ```
    #[rhai_fn(return_raw)]
    pub fn exec(cmd: &str) -> Result<String, Box<EvalAltResult>> {
        let output = if cfg!(windows) {
            std::process::Command::new("cmd").args(["/C", cmd]).output()
        } else {
            std::process::Command::new("sh").args(["-c", cmd]).output()
        };

        match output {
            Ok(o) if o.status.success() => Ok(String::from_utf8_lossy(&o.stdout).to_string()),
            Ok(o) => {
                let stderr = String::from_utf8_lossy(&o.stderr);
                Err(format!("Command failed: {stderr}").into())
            }
            Err(e) => Err(format!("Failed to execute command: {e}").into()),
        }
    }
}
