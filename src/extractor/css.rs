use crate::error::{TalosError, TalosResult};
use std::collections::BTreeSet;
use tree_sitter::{Language, Node, Parser, Query, QueryCapture, QueryCursor};

fn lang_css() -> Language {
    tree_sitter_css::language()
}

const CSS_QUERY: &str = include_str!("../../queries/css.scm");

pub fn extract_css(code: &str) -> TalosResult<Vec<String>> {
    extract_with_query(code, lang_css(), CSS_QUERY)
}

fn extract_with_query(code: &str, lang: Language, query_str: &str) -> TalosResult<Vec<String>> {
    let mut parser = Parser::new();
    parser
        .set_language(lang)
        .map_err(|e| TalosError::ScanError(format!("Failed to set CSS parser language: {}", e)))?;

    let tree = parser
        .parse(code, None)
        .ok_or_else(|| TalosError::ScanError("Failed to parse CSS code".to_string()))?;

    let query = Query::new(lang, query_str)?;
    let mut cursor = QueryCursor::new();
    let mut signatures: BTreeSet<String> = BTreeSet::new();

    for query_match in cursor.matches(&query, tree.root_node(), code.as_bytes()) {
        if let Some(signature) = create_css_signature(&query, query_match.captures, code) {
            signatures.insert(signature);
        }
    }

    Ok(signatures.into_iter().collect())
}

#[derive(Debug)]
enum CssSignatureType {
    Class(String),
    Id(String),
    Element(String),
    Keyframe(String),
    CustomProperty(String),
    AtRule(String),
}

impl CssSignatureType {
    fn render(self) -> String {
        match self {
            Self::Class(name) => format!(".{}", name.trim_start_matches('.')),
            Self::Id(name) => format!("#{}", name.trim_start_matches('#')),
            Self::Element(name) => name,
            Self::Keyframe(name) => format!("@keyframes {}", name),
            Self::CustomProperty(name) => name,
            Self::AtRule(name) => name,
        }
    }
}

struct CssCaptureMap<'a> {
    query: &'a Query,
    captures: &'a [QueryCapture<'a>],
}

impl<'a> CssCaptureMap<'a> {
    fn new(query: &'a Query, captures: &'a [QueryCapture<'a>]) -> Self {
        Self { query, captures }
    }

    fn get_node(&self, name: &str) -> Option<Node<'a>> {
        self.captures
            .iter()
            .find(|c| self.query.capture_names()[c.index as usize] == name)
            .map(|c| c.node)
    }
}

fn create_css_signature(query: &Query, captures: &[QueryCapture], code: &str) -> Option<String> {
    let capture_map = CssCaptureMap::new(query, captures);
    let signature_type = detect_css_signature_type(&capture_map, code)?;
    Some(signature_type.render())
}

fn detect_css_signature_type(captures: &CssCaptureMap, code: &str) -> Option<CssSignatureType> {
    // CSS Class
    if let Some(node) = captures.get_node("css_class") {
        let name = extract_text(code, node.byte_range());
        return Some(CssSignatureType::Class(name.into_owned()));
    }

    // CSS ID
    if let Some(node) = captures.get_node("css_id") {
        let name = extract_text(code, node.byte_range());
        return Some(CssSignatureType::Id(name.into_owned()));
    }

    // CSS Element
    if let Some(node) = captures.get_node("css_element") {
        let name = extract_text(code, node.byte_range());
        return Some(CssSignatureType::Element(name.into_owned()));
    }

    // Keyframes
    if let Some(node) = captures.get_node("keyframe_name") {
        let name = extract_text(code, node.byte_range());
        return Some(CssSignatureType::Keyframe(name.into_owned()));
    }

    // Custom Properties (CSS Variables)
    if let Some(node) = captures.get_node("css_property") {
        let name = extract_text(code, node.byte_range());
        return Some(CssSignatureType::CustomProperty(name.into_owned()));
    }

    // At-rules
    if let Some(node) = captures.get_node("at_rule_name") {
        let name = extract_text(code, node.byte_range());
        return Some(CssSignatureType::AtRule(name.into_owned()));
    }

    None
}

use std::borrow::Cow;

fn extract_text(code: &str, range: std::ops::Range<usize>) -> Cow<'_, str> {
    match code.get(range) {
        Some(s) => {
            let trimmed = s.trim();
            if trimmed.len() == s.len() {
                Cow::Borrowed(trimmed)
            } else {
                Cow::Owned(trimmed.to_string())
            }
        }
        None => Cow::Borrowed(""),
    }
}
