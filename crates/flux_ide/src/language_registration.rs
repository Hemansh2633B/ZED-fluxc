//! Language registration for Flux
//! 
//! This module registers the Flux language with Zed's language system,
//! including syntax highlighting, LSP, and language configuration.

use flux_parser::{flux_language_config, load_flux_queries};
use language::{LanguageConfig, LanguageQueries, LanguageRegistry};
use lsp::LanguageServerId;
use gpui::App;

/// Register Flux language with Zed
pub fn register_flux_language(cx: &mut App) {
    let registry = LanguageRegistry::global();
    
    // Register the language configuration
    let config = flux_language_config();
    registry.add_language_config(config);
    
    // Register Tree-sitter queries
    let queries = load_flux_queries();
    registry.add_language_queries("flux", queries);
    
    // Note: LSP server registration would be done through Zed's LSP system
    // This typically involves adding a language server adapter
    tracing::info!("Flux language registered");
}

/// Get the Flux language configuration
pub fn flux_language_config() -> LanguageConfig {
    flux_parser::flux_language_config()
}

/// Get the Flux Tree-sitter queries
pub fn flux_language_queries() -> LanguageQueries {
    flux_parser::load_flux_queries()
}