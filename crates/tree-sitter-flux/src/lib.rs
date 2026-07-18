#[allow(unused)]
use tree_sitter::Language;

unsafe extern "C" {
    fn tree_sitter_flux() -> Language;
}

/// Get the Tree-sitter language for Flux
pub fn language() -> Language {
    unsafe { tree_sitter_flux() }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_language_loads() {
        let lang = language();
        assert!(lang.version() > 0);
    }
}