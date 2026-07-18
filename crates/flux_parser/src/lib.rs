//! Flux Parser Integration for Zed
//! 
//! This crate provides Tree-sitter based parsing for the Flux language,
//! including syntax highlighting, semantic queries, and AST navigation.

pub mod language_config;
pub mod query_strings;

use anyhow::{Context, Result};
use language_core::{LanguageConfig, LanguageQueries};
use std::borrow::Cow;
use tree_sitter::{Language, Node, Parser, Query, QueryCursor, StreamingIterator, Tree};

/// The Flux language identifier
pub const FLUX_LANGUAGE_ID: &str = "flux";

/// File extensions associated with Flux
pub const FLUX_EXTENSIONS: &[&str] = &["flux", "flx"];

/// Get the Tree-sitter language for Flux
pub fn flux_language() -> Language {
    tree_sitter_flux::language()
}

/// Create a new parser configured for Flux
pub fn create_parser() -> Parser {
    let mut parser = Parser::new();
    parser.set_language(&flux_language()).expect("Failed to set Flux language");
    parser
}

/// Parse Flux source code into a syntax tree
#[ztracing::instrument(skip(source))]
pub fn parse_flux(source: &str) -> Result<Tree> {
    let mut parser = create_parser();
    parser.parse(source, None).context("Failed to parse Flux source")
}

/// Language configuration for Flux
pub fn flux_language_config() -> LanguageConfig {
    language_config::flux_config()
}

/// Load all Tree-sitter queries for Flux
pub fn load_flux_queries() -> LanguageQueries {
    let mut result = LanguageQueries::default();
    
    // Load highlights query
    result.highlights = Some(Cow::Borrowed(query_strings::HIGHLIGHTS));
    
    // Load indents query
    result.indents = Some(Cow::Borrowed(query_strings::INDENTS));
    
    // Load folds query
    result.folds = Some(Cow::Borrowed(query_strings::FOLDS));
    
    // Load locals query
    result.locals = Some(Cow::Borrowed(query_strings::LOCALS));
    
    // Load injections query
    result.injections = Some(Cow::Borrowed(query_strings::INJECTIONS));
    
    result
}

/// Query the Flux AST with a Tree-sitter query
pub fn query_flux_ast(tree: &Tree, source: &str, query_string: &str) -> Result<Vec<QueryMatch>> {
    let query = Query::new(&flux_language(), query_string)?;
    let mut cursor = QueryCursor::new();
    let matches = cursor.matches(&query, tree.root_node(), source.as_bytes());
    
    let mut results = Vec::new();
    let mut matches_iter = matches;
    while let Some(m) = matches_iter.next() {
        results.push(QueryMatch {
            pattern_index: m.pattern_index,
            captures: m.captures.iter().map(|c| QueryCapture {
                index: c.index as usize,
                node: c.node,
            }).collect(),
        });
    }
    
    Ok(results)
}

/// A match from a Tree-sitter query
#[derive(Debug, Clone)]
pub struct QueryMatch {
    pub pattern_index: usize,
    pub captures: Vec<QueryCapture>,
}

/// A capture from a Tree-sitter query match
#[derive(Debug, Clone)]
pub struct QueryCapture {
    pub index: usize,
    pub node: Node,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flux_language_loads() {
        let lang = flux_language();
        assert!(lang.version() > 0);
    }

    #[test]
    fn test_parse_simple_flux() {
        let source = r#"
def main() -> int {
    return 0;
}
"#;
        let tree = parse_flux(source).expect("Failed to parse");
        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn test_parse_data_type() {
        let source = r#"
data{13:16} as my_type;
"#;
        let tree = parse_flux(source).expect("Failed to parse");
        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn test_parse_bit_slice() {
        let source = r#"
u32 value = 0x12345678;
u32 slice = value[0``7];
"#;
        let tree = parse_flux(source).expect("Failed to parse");
        assert!(!tree.root_node().has_error());
    }

    #[test]
    fn test_parse_from_expression() {
        let source = r#"
struct Packet {
    data{8} magic;
    data{16} length;
}

def parse(byte* buf) -> void {
    Packet pkt from buf;
}
"#;
        let tree = parse_flux(source).expect("Failed to parse");
        assert!(!tree.root_node().has_error());
    }
}
