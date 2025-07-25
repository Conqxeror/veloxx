//! Data visualization and plotting module for Velox.
//!
//! This module provides functionality to create various types of charts and plots
//! from DataFrame and Series data, including line plots, scatter plots, bar charts,
//! histograms, and heatmaps.
//!
//! # Features
//!
//! - Line plots for time series and continuous data
//! - Scatter plots for correlation analysis
//! - Bar charts for categorical data
//! - Histograms for distribution analysis
//! - Heatmaps for matrix visualization
//! - Customizable styling and formatting
//!
//! # Examples
//!
//! ```rust
//! use veloxx::dataframe::DataFrame;
//! use veloxx::series::Series;
//! use std::collections::BTreeMap;
//!
//! # #[cfg(feature = "visualization")]
//! # {
//! use veloxx::visualization::{Plot, ChartType};
//!
//! let mut columns = BTreeMap::new();
//! columns.insert(
//!     "x".to_string(),
//!     Series::new_f64("x", vec![Some(1.0), Some(2.0), Some(3.0), Some(4.0)]),
//! );
//! columns.insert(
//!     "y".to_string(),
//!     Series::new_f64("y", vec![Some(2.0), Some(4.0), Some(6.0), Some(8.0)]),
//! );
//!
//! let df = DataFrame::new(columns).unwrap();
//! let plot = Plot::new(&df, ChartType::Line).with_columns("x", "y");
//! // plot.save("output.svg").unwrap();
//! # }
//! ```

#[cfg(feature = "visualization")]
use std::borrow::BorrowMut;
use plotters::prelude::*;
#[cfg(feature = "visualization")]
use plotters_svg::SVGBackend;

use crate::dataframe::DataFrame;
use crate::error::VeloxxError;
use crate::series::Series;

#[cfg(feature = "visualization")]
use crate::types::Value;

/// Chart types supported by the visualization module
#[derive(Debug, Clone, PartialEq)]
pub enum ChartType {
    /// Line plot connecting data points
    Line,
    /// Scatter plot showing individual data points
    Scatter,
    /// Bar chart for categorical data
    Bar,
    /// Histogram for distribution analysis
    Histogram,
    /// Heatmap for matrix visualization
    Heatmap,
}

/// Configuration options for plot styling
#[derive(Debug, Clone)]
pub struct PlotConfig {
    /// Plot title
    pub title: String,
    /// X-axis label
    pub x_label: String,
    /// Y-axis label
    pub y_label: String,
    /// Plot width in pixels
    pub width: u32,
    /// Plot height in pixels
    pub height: u32,
    /// Grid visibility
    pub show_grid: bool,
    /// Legend visibility
    pub show_legend: bool,
}

impl Default for PlotConfig {
    fn default() -> Self {
        Self {
            title: "Velox Plot".to_string(),
            x_label: "X".to_string(),
            y_label: "Y".to_string(),
            width: 800,
            height: 600,
            show_grid: true,
            show_legend: true,
        }
    }
}

/// Main plotting structure for creating visualizations
#[derive(Debug)]
pub struct Plot<'a> {
    dataframe: &'a DataFrame,
    chart_type: ChartType,
    config: PlotConfig,
    x_column: Option<String>,
    y_column: Option<String>,
}

impl<'a> Plot<'a> {
    /// Create a new plot from a DataFrame
    ///
    /// # Arguments
    ///
    /// * `dataframe` - Reference to the DataFrame containing the data
    /// * `chart_type` - Type of chart to create
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::dataframe::DataFrame;
    /// use veloxx::visualization::{Plot, ChartType};
    /// use std::collections::BTreeMap;
    ///
    /// let columns = BTreeMap::new();
    /// let df = DataFrame::new(columns).unwrap();
    /// let plot = Plot::new(&df, ChartType::Line);
    /// ```
    pub fn new(dataframe: &'a DataFrame, chart_type: ChartType) -> Self {
        Self {
            dataframe,
            chart_type,
            config: PlotConfig::default(),
            x_column: None,
            y_column: None,
        }
    }

    /// Set the configuration for the plot
    ///
    /// # Arguments
    ///
    /// * `config` - Plot configuration options
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::visualization::{Plot, PlotConfig, ChartType};
    /// use veloxx::dataframe::DataFrame;
    /// use std::collections::BTreeMap;
    ///
    /// let columns = BTreeMap::new();
    /// let df = DataFrame::new(columns).unwrap();
    /// let mut config = PlotConfig::default();
    /// config.title = "My Custom Plot".to_string();
    ///
    /// let plot = Plot::new(&df, ChartType::Line).with_config(config);
    /// ```
    pub fn with_config(mut self, config: PlotConfig) -> Self {
        self.config = config;
        self
    }

    /// Set the X and Y columns for the plot
    ///
    /// # Arguments
    ///
    /// * `x_column` - Name of the column to use for X-axis
    /// * `y_column` - Name of the column to use for Y-axis
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::visualization::{Plot, ChartType};
    /// use veloxx::dataframe::DataFrame;
    /// use std::collections::BTreeMap;
    ///
    /// let columns = BTreeMap::new();
    /// let df = DataFrame::new(columns).unwrap();
    /// let plot = Plot::new(&df, ChartType::Scatter)
    ///     .with_columns("x_data", "y_data");
    /// ```
    pub fn with_columns(mut self, x_column: &str, y_column: &str) -> Self {
        self.x_column = Some(x_column.to_string());
        self.y_column = Some(y_column.to_string());
        self
    }

    /// Save the plot to a file
    ///
    /// # Arguments
    ///
    /// * `filename` - Path where the plot should be saved (SVG format)
    ///
    /// # Returns
    ///
    /// Result indicating success or failure
    ///
    /// # Examples
    ///
    /// ```rust
    /// use veloxx::visualization::{Plot, ChartType};
    /// use veloxx::dataframe::DataFrame;
    /// use std::collections::BTreeMap;
    ///
    /// let columns = BTreeMap::new();
    /// let df = DataFrame::new(columns).unwrap();
    /// let plot = Plot::new(&df, ChartType::Line);
    /// // plot.save("my_plot.svg").unwrap();
    /// ```
    #[cfg(feature = "visualization")]
    pub fn save(&self, filename: &str) -> Result<(), VeloxxError> {
        match self.chart_type {
            ChartType::Line => self.create_line_plot(filename),
            ChartType::Scatter => self.create_scatter_plot(filename),
            ChartType::Bar => self.create_bar_plot(filename),
            ChartType::Histogram => self.create_histogram(filename),
            ChartType::Heatmap => self.create_heatmap(filename),
        }
    }

    #[cfg(not(feature = "visualization"))]
    pub fn save(&self, _filename: &str) -> Result<(), VeloxxError> {
        Err(VeloxxError::InvalidOperation(
            "Visualization feature is not enabled. Enable with --features visualization"
                .to_string(),
        ))
    }

    #[cfg(feature = "visualization")]
    fn create_line_plot(&self, filename: &str) -> Result<(), VeloxxError> {
        let backend = SVGBackend::new(filename, (self.config.width, self.config.height));
        let root = backend.into_drawing_area();
        root.fill(&WHITE).map_err(|e| {
            VeloxxError::InvalidOperation(format!("Failed to initialize plot: {}", e))
        })?;

        let (x_data, y_data) = self.extract_xy_data()?;

        if x_data.is_empty() || y_data.is_empty() {
            return Err(VeloxxError::InvalidOperation(
                "No data available for plotting".to_string(),
            ));
        }

        let x_min = x_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let x_max = x_data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let y_min = y_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let y_max = y_data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let mut chart = ChartBuilder::on(&root)
            .caption(&self.config.title, ("sans-serif", 40))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .map_err(|e| VeloxxError::InvalidOperation(format!("Failed to build chart: {}", e)))?;

        if self.config.show_grid {
            chart
                .configure_mesh()
                .x_desc(&self.config.x_label)
                .y_desc(&self.config.y_label)
                .draw()
                .map_err(|e| {
                    VeloxxError::InvalidOperation(format!("Failed to draw mesh: {}", e))
                })?;
        }

        chart
            .draw_series(LineSeries::new(
                x_data.iter().zip(y_data.iter()).map(|(&x, &y)| (x, y)),
                &BLUE,
            ))
            .map_err(|e| {
                VeloxxError::InvalidOperation(format!("Failed to draw line series: {}", e))
            })?
            .label("Data")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 10, y)], BLUE));

        if self.config.show_legend {
            chart.configure_series_labels().draw().map_err(|e| {
                VeloxxError::InvalidOperation(format!("Failed to draw legend: {}", e))
            })?;
        }

        root.present()
            .map_err(|e| VeloxxError::InvalidOperation(format!("Failed to save plot: {}", e)))?;

        Ok(())
    }

    #[cfg(feature = "visualization")]
    fn create_scatter_plot(&self, filename: &str) -> Result<(), VeloxxError> {
        let backend = SVGBackend::new(filename, (self.config.width, self.config.height));
        let root = backend.into_drawing_area();
        root.fill(&WHITE).map_err(|e| {
            VeloxxError::InvalidOperation(format!("Failed to initialize plot: {}", e))
        })?;

        let (x_data, y_data) = self.extract_xy_data()?;

        if x_data.is_empty() || y_data.is_empty() {
            return Err(VeloxxError::InvalidOperation(
                "No data available for plotting".to_string(),
            ));
        }

        let x_min = x_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let x_max = x_data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        let y_min = y_data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let y_max = y_data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let mut chart = ChartBuilder::on(&root)
            .caption(&self.config.title, ("sans-serif", 40))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .map_err(|e| VeloxxError::InvalidOperation(format!("Failed to build chart: {}", e)))?;

        if self.config.show_grid {
            chart
                .configure_mesh()
                .x_desc(&self.config.x_label)
                .y_desc(&self.config.y_label)
                .draw()
                .map_err(|e| {
                    VeloxxError::InvalidOperation(format!("Failed to draw mesh: {}", e))
                })?;
        }

        chart
            .draw_series(
                x_data
                    .iter()
                    .zip(y_data.iter())
                    .map(|(&x, &y)| Circle::new((x, y), 3, BLUE.filled())),
            )
            .map_err(|e| {
                VeloxxError::InvalidOperation(format!("Failed to draw scatter series: {}", e))
            })?
            .label("Data Points")
            .legend(|(x, y)| Circle::new((x + 10, y), 3, BLUE.filled()));

        if self.config.show_legend {
            chart.configure_series_labels().draw().map_err(|e| {
                VeloxxError::InvalidOperation(format!("Failed to draw legend: {}", e))
            })?;
        }

        root.present()
            .map_err(|e| VeloxxError::InvalidOperation(format!("Failed to save plot: {}", e)))?;

        Ok(())
    }

    #[cfg(feature = "visualization")]
    fn create_bar_plot(&self, filename: &str) -> Result<(), VeloxxError> {
        let backend = SVGBackend::new(filename, (self.config.width, self.config.height));
        let root = backend.into_drawing_area();
        root.fill(&WHITE).map_err(|e| {
            VeloxxError::InvalidOperation(format!("Failed to initialize plot: {}", e))
        })?;

        let (categories, values) = self.extract_categorical_data()?;

        if categories.is_empty() || values.is_empty() {
            return Err(VeloxxError::InvalidOperation(
                "No data available for plotting".to_string(),
            ));
        }

        let y_max = values.iter().fold(0.0f64, |a, &b| a.max(b));

        let mut chart = ChartBuilder::on(&root)
            .caption(&self.config.title, ("sans-serif", 40))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d(0f64..(categories.len() as f64), 0f64..y_max * 1.1)
            .map_err(|e| VeloxxError::InvalidOperation(format!("Failed to build chart: {}", e)))?;

        if self.config.show_grid {
            chart
                .configure_mesh()
                .x_desc(&self.config.x_label)
                .y_desc(&self.config.y_label)
                .draw()
                .map_err(|e| {
                    VeloxxError::InvalidOperation(format!("Failed to draw mesh: {}", e))
                })?;
        }

        chart
            .draw_series(values.iter().enumerate().map(|(i, &value)| {
                Rectangle::new([(i as f64, 0.0), (i as f64 + 0.8, value)], BLUE.filled())
            }))
            .map_err(|e| {
                VeloxxError::InvalidOperation(format!("Failed to draw bar series: {}", e))
            })?
            .label("Values")
            .legend(|(x, y)| Rectangle::new([(x, y), (x + 10, y + 10)], BLUE.filled()));

        if self.config.show_legend {
            chart.configure_series_labels().draw().map_err(|e| {
                VeloxxError::InvalidOperation(format!("Failed to draw legend: {}", e))
            })?;
        }

        root.present()
            .map_err(|e| VeloxxError::InvalidOperation(format!("Failed to save plot: {}", e)))?;

        Ok(())
    }

    #[cfg(feature = "visualization")]
    fn create_histogram(&self, filename: &str) -> Result<(), VeloxxError> {
        let backend = SVGBackend::new(filename, (self.config.width, self.config.height));
        let root = backend.into_drawing_area();
        root.fill(&WHITE).map_err(|e| {
            VeloxxError::InvalidOperation(format!("Failed to initialize plot: {}", e))
        })?;

        let data = self.extract_histogram_data()?;

        if data.is_empty() {
            return Err(VeloxxError::InvalidOperation(
                "No data available for plotting".to_string(),
            ));
        }

        let x_min = data.iter().fold(f64::INFINITY, |a, &b| a.min(b));
        let x_max = data.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b));

        let mut chart = ChartBuilder::on(&root)
            .caption(&self.config.title, ("sans-serif", 40))
            .margin(20)
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_cartesian_2d((x_min..x_max).step(1.0), 0u32..50u32)
            .map_err(|e| VeloxxError::InvalidOperation(format!("Failed to build chart: {}", e)))?;

        chart
            .configure_mesh()
            .x_desc(&self.config.x_label)
            .y_desc(&self.config.y_label)
            .draw()
            .map_err(|e| {
                VeloxxError::InvalidOperation(format!("Failed to draw mesh: {}", e))
            })?;

        let hist_data: Vec<f64> = data.to_vec();
        let series = Histogram::vertical(chart.borrow_mut())
                .style(BLUE.filled())
                .data(hist_data.iter().map(|x| (*x, 1)));
        chart.draw_series(series).map_err(|e| {
            VeloxxError::InvalidOperation(format!("Failed to draw histogram series: {}", e))
        })?;


        root.present()
            .map_err(|e| VeloxxError::InvalidOperation(format!("Failed to save plot: {}", e)))?;

        Ok(())
    }

    #[cfg(feature = "visualization")]
    fn create_heatmap(&self, _filename: &str) -> Result<(), VeloxxError> {
        // Placeholder for heatmap implementation
        Err(VeloxxError::InvalidOperation(
            "Heatmap plotting not yet implemented".to_string(),
        ))
    }

    fn extract_histogram_data(&self) -> Result<Vec<f64>, VeloxxError> {
        let x_col_name = self
            .x_column
            .as_ref()
            .ok_or_else(|| VeloxxError::InvalidOperation("X column not specified".to_string()))?;

        let x_series = self
            .dataframe
            .get_column(x_col_name)
            .ok_or_else(|| VeloxxError::ColumnNotFound(x_col_name.clone()))?;

        self.series_to_f64_vec(x_series)
    }

    fn extract_xy_data(&self) -> Result<(Vec<f64>, Vec<f64>), VeloxxError> {
        let x_col_name = self
            .x_column
            .as_ref()
            .ok_or_else(|| VeloxxError::InvalidOperation("X column not specified".to_string()))?;
        let y_col_name = self
            .y_column
            .as_ref()
            .ok_or_else(|| VeloxxError::InvalidOperation("Y column not specified".to_string()))?;

        let x_series = self
            .dataframe
            .get_column(x_col_name)
            .ok_or_else(|| VeloxxError::ColumnNotFound(x_col_name.clone()))?;
        let y_series = self
            .dataframe
            .get_column(y_col_name)
            .ok_or_else(|| VeloxxError::ColumnNotFound(y_col_name.clone()))?;

        let x_data = self.series_to_f64_vec(x_series)?;
        let y_data = self.series_to_f64_vec(y_series)?;

        Ok((x_data, y_data))
    }

    fn extract_categorical_data(&self) -> Result<(Vec<String>, Vec<f64>), VeloxxError> {
        let x_col_name = self
            .x_column
            .as_ref()
            .ok_or_else(|| VeloxxError::InvalidOperation("X column not specified".to_string()))?;
        let y_col_name = self
            .y_column
            .as_ref()
            .ok_or_else(|| VeloxxError::InvalidOperation("Y column not specified".to_string()))?;

        let x_series = self
            .dataframe
            .get_column(x_col_name)
            .ok_or_else(|| VeloxxError::ColumnNotFound(x_col_name.clone()))?;
        let y_series = self
            .dataframe
            .get_column(y_col_name)
            .ok_or_else(|| VeloxxError::ColumnNotFound(y_col_name.clone()))?;

        let categories = self.series_to_string_vec(x_series)?;
        let values = self.series_to_f64_vec(y_series)?;

        Ok((categories, values))
    }

    fn series_to_f64_vec(&self, series: &Series) -> Result<Vec<f64>, VeloxxError> {
        let mut result = Vec::new();
        for i in 0..series.len() {
            if let Some(value) = series.get_value(i) {
                match value {
                    Value::F64(f) => result.push(f),
                    Value::I32(i) => result.push(i as f64),
                    _ => {
                        return Err(VeloxxError::InvalidOperation(
                            "Cannot convert non-numeric data to f64".to_string(),
                        ));
                    }
                }
            }
        }
        Ok(result)
    }

    fn series_to_string_vec(&self, series: &Series) -> Result<Vec<String>, VeloxxError> {
        let mut result = Vec::new();
        for i in 0..series.len() {
            if let Some(value) = series.get_value(i) {
                result.push(value.to_string());
            }
        }
        Ok(result)
    }
}
