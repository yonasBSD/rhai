#[cfg(feature = "no_std")]
use std::prelude::v1::*;

use super::*;
use crate::def_package;

def_package! {
    /// yonasBSD package containing additional features.
    ///
    /// # Contents
    ///
    /// * [`BasicFilesystemPackage`][super::BasicFilesystemPackage]
    /// * [`BasicEnvironmentPackage`][super::BasicEnvironmentPackage]
    /// * [`BasicShellPackage`][super::BasicShellPackage]
    /// * [`BasicJsonPackage`][super::BasicJsonPackage]
    /// * [`BasicSemverPackage`][super::BasicSemverPackage]
    pub YonasBSDPackage(lib) :
            #[cfg(not(feature = "no_fs"))] BasicFilesystemPackage,
            #[cfg(not(feature = "no_env"))] BasicEnvironmentPackage,
            #[cfg(not(feature = "no_shell"))] BasicShellPackage,
            #[cfg(not(feature = "no_json"))] BasicJsonPackage,
            //#[cfg(not(feature = "no_yaml"))] BasicYamlPackage,
            #[cfg(not(feature = "no_semver"))] BasicSemverPackage
    {
        lib.set_standard_lib(true);
    }
}
