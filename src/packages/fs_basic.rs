#![cfg(all(not(feature = "no_fs"), feature = "fs"))]
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
    pub BasicFilesystemPackage(lib) {
        lib.set_standard_lib(true);

        // Register filesystem functions
        combine_with_exported_module!(lib, "fs", fs_functions);
    }
}

// File operations
#[export_module]
mod fs_functions {
    use crate::Array;
    use crate::Dynamic;
    use crate::EvalAltResult;
    use std::io;
    use std::path::Path;

    // Helper to standardize Rhai error reporting for IO failures
    fn io_error_to_rhai_error(action: &str, path: &str, e: io::Error) -> Box<EvalAltResult> {
        Box::new(EvalAltResult::ErrorSystem(
            format!("Failed to {action} '{path}'"),
            e.to_string().into(),
        ))
    }

    /// Read a file
    ///
    /// ```rhai
    /// let content = read_file("/path/to/file.txt");
    /// ```
    #[rhai_fn(return_raw)]
    pub fn read_file(path: &str) -> Result<String, Box<EvalAltResult>> {
        std::fs::read_to_string(path).map_err(|e| io_error_to_rhai_error("read file", path, e))
    }

    /// Write content to a file. Overwrites the file if it exists.
    ///
    /// ```rhai
    /// write_file("/path/to/file.txt", "new content");
    /// ```
    #[rhai_fn(return_raw)]
    pub fn write_file(path: &str, contents: &str) -> Result<(), Box<EvalAltResult>> {
        std::fs::write(path, contents).map_err(|e| io_error_to_rhai_error("write file", path, e))
    }

    /// Check if a file or directory exists.
    ///
    /// ```rhai
    /// let exists = file_exists("/path/to/file.txt");
    /// ```
    #[rhai_fn()]
    pub fn file_exists(path: &str) -> bool {
        Path::new(path).exists()
    }

    /// Check if the path points to a file.
    ///
    /// ```rhai
    /// let is_f = is_file("/path/to/file.txt");
    /// ```
    #[rhai_fn()]
    pub fn is_file(path: &str) -> bool {
        Path::new(path).is_file()
    }

    /// Check if the path points to a directory.
    ///
    /// ```rhai
    /// let is_d = is_dir("/path/to/dir");
    /// ```
    #[rhai_fn()]
    pub fn is_dir(path: &str) -> bool {
        Path::new(path).is_dir()
    }

    // Directory operations

    /// Create a directory and its parents if they don't exist.
    ///
    /// ```rhai
    /// mkdir("new/directory/path");
    /// ```
    #[rhai_fn(return_raw)]
    pub fn mkdir(path: &str) -> Result<(), Box<EvalAltResult>> {
        std::fs::create_dir_all(path)
            .map_err(|e| io_error_to_rhai_error("create directory", path, e))
    }

    /// Remove a directory and all its contents (equivalent to `rm -rf`).
    ///
    /// ```rhai
    /// rmdir("old/directory/path");
    /// ```
    #[rhai_fn(return_raw)]
    pub fn rmdir(path: &str) -> Result<(), Box<EvalAltResult>> {
        std::fs::remove_dir_all(path)
            .map_err(|e| io_error_to_rhai_error("remove directory", path, e))
    }

    /// List the contents of a directory, returning an array of paths (strings).
    ///
    /// ```rhai
    /// let files = list_dir("/path/to/dir");
    /// ```
    #[rhai_fn(return_raw)]
    pub fn list_dir(path: &str) -> Result<Array, Box<EvalAltResult>> {
        let entries: Result<Vec<_>, io::Error> = std::fs::read_dir(path)
            .map_err(|e| io_error_to_rhai_error("read directory", path, e))?
            .map(|e| e.map(|entry| Dynamic::from(entry.path().to_string_lossy().to_string())))
            .collect();

        entries
            .map(|vec| vec.into())
            .map_err(|e| io_error_to_rhai_error("process directory entry", path, e))
    }

    // Path operations

    /// Join two path segments.
    ///
    /// ```rhai
    /// let full_path = join_path("home/user", "file.txt"); // returns "home/user/file.txt"
    /// ```
    #[rhai_fn()]
    pub fn join_path(a: &str, b: &str) -> String {
        Path::new(a).join(b).to_string_lossy().to_string()
    }

    /// Get the parent path. Returns an empty string if the path is at the root.
    ///
    /// ```rhai
    /// let parent = parent_path("/a/b/c"); // returns "/a/b"
    /// ```
    #[rhai_fn()]
    pub fn parent_path(path: &str) -> String {
        Path::new(path)
            .parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    /// Get the file name component of a path. Returns an empty string if it's a root or path ends in `..`.
    ///
    /// ```rhai
    /// let name = file_name("/a/b/file.txt"); // returns "file.txt"
    /// ```
    #[rhai_fn()]
    pub fn file_name(path: &str) -> String {
        Path::new(path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    /// Get the file extension. Returns an empty string if no extension is found.
    ///
    /// ```rhai
    /// let ext = extension("/a/b/file.txt"); // returns "txt"
    /// ```
    #[rhai_fn()]
    pub fn extension(path: &str) -> String {
        Path::new(path)
            .extension()
            .map(|e| e.to_string_lossy().to_string())
            .unwrap_or_default()
    }

    /// Perform glob matching and return an array of matching paths.
    ///
    /// ```rhai
    /// let files = glob("./src/*.rs");
    /// ```
    #[rhai_fn(return_raw)]
    pub fn glob(pattern: &str) -> Result<Array, Box<EvalAltResult>> {
        let paths: Vec<_> = glob::glob(pattern)
            .map_err(|e| {
                Box::new(EvalAltResult::ErrorSystem(
                    format!("Invalid glob pattern: {pattern}"),
                    format!("{e}").into(),
                ))
            })?
            .filter_map(std::result::Result::ok)
            .map(|p| Dynamic::from(p.to_string_lossy().to_string()))
            .collect();
        Ok(paths)
    }
}
