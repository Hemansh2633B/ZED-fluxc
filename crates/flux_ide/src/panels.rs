//! Flux IDE Panels
//! 
//! This module provides the right-side panels for Flux-specific functionality:
//! - FluxInspectorPanel: Data type inspector, bit slice explorer, struct visualizer
//! - FluxBinaryViewerPanel: Binary/ELF file viewer with Flux struct overlay
//! - FluxBuildPanel: Build dashboard and project analyzer

mod flux_inspector_panel;
mod flux_binary_viewer_panel;
mod flux_build_panel;

pub use flux_inspector_panel::FluxInspectorPanel;
pub use flux_binary_viewer_panel::FluxBinaryViewerPanel;
pub use flux_build_panel::FluxBuildPanel;