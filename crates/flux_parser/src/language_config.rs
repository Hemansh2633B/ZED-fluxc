use language_core::{LanguageConfig, LanguageMatcher, BracketPairConfig, BracketPair, SoftWrap};

/// Get the Flux language configuration
pub fn flux_config() -> LanguageConfig {
    LanguageConfig {
        name: "Flux".into(),
        soft_wrap: Some(SoftWrap::PreferLine),
        autoclose_before: ")]}".into(),
        matcher: LanguageMatcher {
            path_suffixes: vec!["flux".to_owned(), "flx".to_owned()],
            first_line_pattern: Some("^#!.*flux".to_owned()),
            modeline_aliases: vec!["flux".to_owned()],
        },
        brackets: BracketPairConfig {
            pairs: vec![
                BracketPair { start: "(".to_string(), end: ")".to_string(), close: true, surround: true, newline: false },
                BracketPair { start: "[".to_string(), end: "]".to_string(), close: true, surround: true, newline: false },
                BracketPair { start: "{".to_string(), end: "}".to_string(), close: true, surround: true, newline: false },
                BracketPair { start: "\"".to_string(), end: "\"".to_string(), close: true, surround: true, newline: false },
                BracketPair { start: "'".to_string(), end: "'".to_string(), close: true, surround: true, newline: false },
            ],
            disabled_scopes_by_bracket_ix: Default::default(),
        },
        ..Default::default()
    }
}