use crate::error::{TalosError, TalosResult};
use std::collections::BTreeSet;
use tree_sitter::{Language, Node, Parser, Query, QueryCapture, QueryCursor};

fn lang_js() -> Language {
    tree_sitter_javascript::language()
}
fn lang_ts() -> Language {
    tree_sitter_typescript::language_typescript()
}
#[allow(dead_code)]
fn lang_tsx() -> Language {
    tree_sitter_typescript::language_tsx()
}

const UNIFIED_QUERY: &str = include_str!("../../queries/unified.scm");

pub fn extract_js(code: &str) -> TalosResult<Vec<String>> {
    extract_with_query(code, lang_js(), UNIFIED_QUERY)
}

pub fn extract_ts(code: &str) -> TalosResult<Vec<String>> {
    extract_with_query(code, lang_ts(), UNIFIED_QUERY)
}

pub fn extract_tsx(code: &str) -> TalosResult<Vec<String>> {
    extract_with_query(code, lang_tsx(), UNIFIED_QUERY)
}

fn extract_with_query(code: &str, lang: Language, query_str: &str) -> TalosResult<Vec<String>> {
    let mut parser = Parser::new();
    parser
        .set_language(lang)
        .map_err(|e| TalosError::ScanError(format!("Failed to set parser language: {}", e)))?;

    let tree = parser
        .parse(code, None)
        .ok_or_else(|| TalosError::ScanError("Failed to parse code".to_string()))?;

    let query = Query::new(lang, query_str)?;
    let mut cursor = QueryCursor::new();
    let mut signatures: BTreeSet<String> = BTreeSet::new();

    for query_match in cursor.matches(&query, tree.root_node(), code.as_bytes()) {
        if let Some(signature) = create_signature(&query, query_match.captures, code) {
            signatures.insert(signature);
        }
    }

    Ok(signatures.into_iter().collect())
}

#[derive(Debug)]
enum SignatureType {
    Class(String),
    Function(String, String),
    Method(String, String),
    ArrowFunction(String, String),
    FunctionExpression(String, String),
}

impl SignatureType {
    fn render(self) -> String {
        match self {
            Self::Class(name) => format!("class {}", name),
            Self::Function(name, params) => format!("function {}{}", name, params),
            Self::Method(name, params) => format!("method {}{}", name, params),
            Self::ArrowFunction(name, params) => format!("const {} = {} =>", name, params),
            Self::FunctionExpression(name, params) => {
                format!("const {} = function {}", name, params)
            }
        }
    }
}

struct CaptureMap<'a> {
    query: &'a Query,
    captures: &'a [QueryCapture<'a>],
}

impl<'a> CaptureMap<'a> {
    fn new(query: &'a Query, captures: &'a [QueryCapture<'a>]) -> Self {
        Self { query, captures }
    }

    fn get_node(&self, name: &str) -> Option<Node<'a>> {
        self.captures
            .iter()
            .find(|c| self.query.capture_names()[c.index as usize] == name)
            .map(|c| c.node)
    }

    fn has_capture(&self, name: &str) -> bool {
        self.get_node(name).is_some()
    }
}

fn create_signature(query: &Query, captures: &[QueryCapture], code: &str) -> Option<String> {
    let capture_map = CaptureMap::new(query, captures);

    let signature_type = detect_signature_type(&capture_map, code)?;
    Some(signature_type.render())
}

fn detect_signature_type(captures: &CaptureMap, code: &str) -> Option<SignatureType> {
    // Class declaration
    if let Some(node) = captures.get_node("cname") {
        let name = extract_text(code, node.byte_range());
        return Some(SignatureType::Class(name.into_owned()));
    }

    // Function declaration
    if let (Some(name_node), Some(params_node)) =
        (captures.get_node("fname"), captures.get_node("fparams"))
    {
        let name = extract_text(code, name_node.byte_range());
        let params = extract_text(code, params_node.byte_range());
        return Some(SignatureType::Function(
            name.into_owned(),
            params.into_owned(),
        ));
    }

    // Method definition
    if let (Some(name_node), Some(params_node)) =
        (captures.get_node("mname"), captures.get_node("mparams"))
    {
        let name = extract_text(code, name_node.byte_range());
        let params = extract_text(code, params_node.byte_range());
        return Some(SignatureType::Method(
            name.into_owned(),
            params.into_owned(),
        ));
    }

    // Variable assigned function or arrow function
    if let (Some(name_node), Some(params_node)) =
        (captures.get_node("vname"), captures.get_node("vparams"))
    {
        let name = extract_text(code, name_node.byte_range());
        let params = extract_text(code, params_node.byte_range());

        if captures.has_capture("is_arrow") {
            return Some(SignatureType::ArrowFunction(
                name.into_owned(),
                params.into_owned(),
            ));
        } else {
            return Some(SignatureType::FunctionExpression(
                name.into_owned(),
                params.into_owned(),
            ));
        }
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
