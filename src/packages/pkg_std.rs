#[cfg(feature = "no_std")]
use std::prelude::v1::*;

use super::*;
use crate::def_package;
use crate::types::plots::RhaiPlot;

def_package! {
    /// Standard package containing all built-in features.
    ///
    /// # Contents
    ///
    /// * [`CorePackage`][super::CorePackage]
    /// * [`BitFieldPackage`][super::BitFieldPackage]
    /// * [`LogicPackage`][super::LogicPackage]
    /// * [`BasicMathPackage`][super::BasicMathPackage]
    /// * [`BasicArrayPackage`][super::BasicArrayPackage]
    /// * [`BasicBlobPackage`][super::BasicBlobPackage]
    /// * [`BasicMapPackage`][super::BasicMapPackage]
    /// * [`BasicTimePackage`][super::BasicTimePackage]
    /// * [`MoreStringPackage`][super::MoreStringPackage]
    pub StandardPackage(lib) :
            CorePackage,
            BitFieldPackage,
            LogicPackage,
            BasicMathPackage,
            #[cfg(not(feature = "no_index"))] BasicArrayPackage,
            #[cfg(not(feature = "no_index"))] BasicBlobPackage,
            #[cfg(not(feature = "no_object"))] BasicMapPackage,
            #[cfg(not(feature = "no_time"))] BasicTimePackage,
            MoreStringPackage
    {
        lib.set_standard_lib(true);

        // Create plots package
        let plots_package = BasicPlotsPackage::new();
        let plots_shared = plots_package.as_shared_module();

        // Extract and add the plots submodule
        if let Some(plots_submodule) = plots_shared.get_sub_module("plots") {
            println!("Adding plots submodule to StandardPackage");
            lib.set_sub_module("plots", plots_submodule.clone());
            lib.set_custom_type::<RhaiPlot>("Plot");
        }
    }
}
