//! Flux IDE Main Crate
//! 
//! This crate integrates Flux-specific functionality into Zed, providing:
//! - Flux language support (syntax highlighting, LSP)
//! - Flux Inspector Panel (right sidebar)
//! - Flux Binary Viewer Panel (right sidebar)
//! - Flux Build Dashboard (bottom panel)

mod language_registration;
mod panels;
mod flux_inspector_panel;
mod flux_binary_viewer_panel;
mod flux_build_panel;

pub use language_registration::{register_flux_language, flux_language_config, flux_language_queries};
pub use panels::{FluxInspectorPanel, FluxBinaryViewerPanel, FluxBuildPanel};

#[cfg(test)]
mod tests {
    #[test]
    fn test_flux_ide_compiles() {
        // Just verify the crate compiles
    }
}