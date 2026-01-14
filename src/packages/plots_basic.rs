#![cfg(all(not(feature = "no_plots"), feature = "plots"))]
#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

//! Rhai scripting engine integration for plotting utilities.

use crate::def_package;
use crate::plugin::*;
use crate::types::plots::RhaiPlot;
use crate::Array;

// -------------------------------------------------------------------------
// Helper conversion utilities
// -------------------------------------------------------------------------

fn array_to_f64_vec(arr: &Array, name: &str) -> Result<Vec<f64>, String> {
    arr.iter()
        .map(|v| {
            v.as_float()
                .map_err(|typ| format!("{name} must be numbers, got {typ}"))
        })
        .collect()
}

fn array_to_string_vec(arr: &Array, name: &str) -> Result<Vec<String>, String> {
    arr.iter()
        .map(|v| {
            v.clone()
                .into_immutable_string()
                .map(|s: ImmutableString| s.to_string())
                .map_err(|typ| format!("{name} must be strings, got {typ}"))
        })
        .collect()
}

#[export_module]
mod plots_functions {
    use crate::types::plots::RhaiPlot;
    use crate::ImmutableString;
    use crate::{Array, Dynamic};

    // -------------------------------------------------------------------------
    // Constructors
    // -------------------------------------------------------------------------

    /// Create a new empty plot.
    ///
    /// # Example
    /// ```rhai
    /// import plots as p;
    ///
    /// let plot = p::plot();
    /// plot.title("My Plot");
    /// plot.save();   // writes "output.png"
    /// ```
    #[rhai_fn(name = "plot", return_raw)]
    pub fn plot() -> RhaiResult {
        Ok(Dynamic::from(RhaiPlot::new()))
    }

    // -------------------------------------------------------------------------
    // Method-style plot operations
    // -------------------------------------------------------------------------

    /// Add a line plot.
    ///
    /// # Example
    /// ```rhai
    /// import plots as p;
    ///
    /// let plot = p::new();
    /// plot.line([0,1,2], [0,1,4]);
    /// plot.save();
    /// ```
    #[rhai_fn(name = "line", return_raw)]
    pub fn line_method(plot: &mut RhaiPlot, x: Array, y: Array) -> RhaiResult {
        let x_vec = array_to_f64_vec(&x, "X values")?;
        let y_vec = array_to_f64_vec(&y, "Y values")?;
        Ok(Dynamic::from(plot.line_internal(x_vec, y_vec)?))
    }

    /// Add a scatter plot.
    ///
    /// # Example
    /// ```rhai
    /// import plots as p;
    ///
    /// let plot = p::new();
    /// plot.scatter([1,2,3], [2.1, 1.8, 3.0]);
    /// plot.save();
    /// ```
    #[rhai_fn(name = "scatter", return_raw)]
    pub fn scatter_method(plot: &mut RhaiPlot, x: Array, y: Array) -> RhaiResult {
        let x_vec = array_to_f64_vec(&x, "X values")?;
        let y_vec = array_to_f64_vec(&y, "Y values")?;
        Ok(Dynamic::from(plot.scatter_internal(x_vec, y_vec)?))
    }

    /// Add a bar chart.
    ///
    /// # Example
    /// ```rhai
    /// import plots as p;
    ///
    /// let plot = p::new();
    /// plot.bar(["A","B","C"], [1.0, 2.5, 1.8]);
    /// plot.show();
    /// ```
    #[rhai_fn(name = "bar", return_raw)]
    pub fn bar_method(plot: &mut RhaiPlot, labels: Array, values: Array) -> RhaiResult {
        let label_vec = array_to_string_vec(&labels, "Labels")?;
        let value_vec = array_to_f64_vec(&values, "Values")?;
        Ok(Dynamic::from(plot.bar_internal(label_vec, value_vec)?))
    }

    /// Set the plot title.
    ///
    /// # Example
    /// ```rhai
    /// import plots as p;
    ///
    /// let plot = p::new();
    /// plot.title("Experiment Results");
    /// plot.save();
    /// ```
    #[rhai_fn(name = "title", return_raw)]
    pub fn title_method(plot: &mut RhaiPlot, title: &str) -> RhaiResult {
        Ok(Dynamic::from(plot.title_internal(title)?))
    }

    /// Set the x-axis label.
    ///
    /// # Example
    /// ```rhai
    /// import plots as p;
    ///
    /// let plot = p::new();
    /// plot.xlabel("Time (s)");
    /// plot.save();
    /// ```
    #[rhai_fn(name = "xlabel", return_raw)]
    pub fn xlabel_method(plot: &mut RhaiPlot, label: &str) -> RhaiResult {
        Ok(Dynamic::from(plot.xlabel_internal(label)?))
    }

    /// Set the y-axis label.
    ///
    /// # Example
    /// ```rhai
    /// import plots as p;
    ///
    /// let plot = p::new();
    /// plot.ylabel("Amplitude");
    /// plot.show();
    /// ```
    #[rhai_fn(name = "ylabel", return_raw)]
    pub fn ylabel_method(plot: &mut RhaiPlot, label: &str) -> RhaiResult {
        Ok(Dynamic::from(plot.ylabel_internal(label)?))
    }

    // -------------------------------------------------------------------------
    // Save / Show
    // -------------------------------------------------------------------------

    /// Save the plot to a file.
    /// Defaults to `"output.png"` if no path is provided.
    ///
    /// # Example
    /// ```rhai
    /// import plots as p;
    ///
    /// let plot = p::new();
    /// plot.line([1,2,3], [4,5,6]);
    ///
    /// plot.save();              // writes "output.png"
    /// plot.save("custom.png");  // writes "custom.png"
    /// ```
    #[rhai_fn(name = "save", return_raw)]
    pub fn save_method(plot: &mut RhaiPlot, path: Option<&str>) -> RhaiResult {
        let path = path.unwrap_or("output.png");
        plot.save_internal(path)?;
        Ok(Dynamic::UNIT)
    }

    /// Alias for `save()`.
    ///
    /// # Example
    /// ```rhai
    /// import plots as p;
    ///
    /// let plot = p::new();
    /// plot.scatter([1,2,3], [3,2,1]);
    ///
    /// plot.show();              // writes "output.png"
    /// plot.show("preview.png"); // writes "preview.png"
    /// ```
    #[rhai_fn(name = "show", return_raw)]
    pub fn show_method(plot: &mut RhaiPlot, path: Option<&str>) -> RhaiResult {
        let path = path.unwrap_or("output.png");
        plot.save_internal(path)?;
        Ok(Dynamic::UNIT)
    }
}

def_package! {
    /// Package of plotting utilities using ruviz.
    pub BasicPlotsPackage(lib) {
        lib.set_standard_lib(false);

        // Register the custom Rhai type for plots.
        lib.set_custom_type::<RhaiPlot>("Plot");

        // Build the actual 'plots' submodule.
        let mut plots_mod = Module::new();

        // Export all functions from your Rust module.
        plots_mod.combine(exported_module!(plots_functions));

        // Attach the submodule under the name 'plots'.
        lib.set_sub_module("plots", plots_mod);
    }
}
