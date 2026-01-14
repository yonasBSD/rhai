use crate::EvalAltResult;
use ruviz::prelude::*;
use std::sync::{Arc, Mutex};

/// Wrapper for ruviz Plot with interior mutability for Rhai
#[derive(Clone)]
pub struct RhaiPlot(Arc<Mutex<Plot>>);

impl RhaiPlot {
    pub fn new() -> Self {
        RhaiPlot(Arc::new(Mutex::new(Plot::new())))
    }

    pub fn line_internal(
        &self,
        x: Vec<f64>,
        y: Vec<f64>,
    ) -> std::result::Result<Self, Box<EvalAltResult>> {
        let mut plot = self.0.lock().unwrap();
        *plot = plot.clone().line(&x, &y).into();
        Ok(self.clone())
    }

    pub fn scatter_internal(
        &self,
        x: Vec<f64>,
        y: Vec<f64>,
    ) -> std::result::Result<Self, Box<EvalAltResult>> {
        let mut plot = self.0.lock().unwrap();
        *plot = plot.clone().scatter(&x, &y).into();
        Ok(self.clone())
    }

    pub fn bar_internal(
        &self,
        labels: Vec<String>,
        values: Vec<f64>,
    ) -> std::result::Result<Self, Box<EvalAltResult>> {
        let mut plot = self.0.lock().unwrap();
        let label_refs: Vec<&str> = labels.iter().map(|s| s.as_str()).collect();
        *plot = plot.clone().bar(&label_refs, &values).into();
        Ok(self.clone())
    }

    pub fn title_internal(&self, title: &str) -> std::result::Result<Self, Box<EvalAltResult>> {
        let mut plot = self.0.lock().unwrap();
        *plot = plot.clone().title(title);
        Ok(self.clone())
    }

    pub fn xlabel_internal(&self, label: &str) -> std::result::Result<Self, Box<EvalAltResult>> {
        let mut plot = self.0.lock().unwrap();
        *plot = plot.clone().xlabel(label);
        Ok(self.clone())
    }

    pub fn ylabel_internal(&self, label: &str) -> std::result::Result<Self, Box<EvalAltResult>> {
        let mut plot = self.0.lock().unwrap();
        *plot = plot.clone().ylabel(label);
        Ok(self.clone())
    }

    pub fn save_internal(&self, path: &str) -> std::result::Result<(), Box<EvalAltResult>> {
        let plot = self.0.lock().unwrap();
        plot.clone()
            .save(path)
            .map_err(|e| format!("Failed to save plot: {}", e).into())
    }
}
