use crate::error::{TalosError, TalosResult};
use std::fs;
use std::path::Path;

mod css;
mod js_ts;

#[derive(Debug, Clone, Copy)]
pub enum SupportedLang {
    JavaScript,
    TypeScript,
    TypeScriptReact,
    CSS,
}

pub trait LanguageExtractor {
    fn extract(&self, code: &str) -> TalosResult<Vec<String>>;
}

pub fn infer_lang_from_ext(path: &Path) -> Option<SupportedLang> {
    let ext = path
        .extension()
        .map(|e| e.to_string_lossy().to_ascii_lowercase())?;
    match ext.as_str() {
        "js" | "jsx" => Some(SupportedLang::JavaScript),
        "ts" => Some(SupportedLang::TypeScript),
        "tsx" => Some(SupportedLang::TypeScriptReact),
        "css" => Some(SupportedLang::CSS),
        _ => None,
    }
}

pub fn extract_signatures_for_file(path: &Path) -> TalosResult<Vec<String>> {
    let lang = match infer_lang_from_ext(path) {
        Some(l) => l,
        None => return Ok(vec![]),
    };

    let code = read_file_safely(path)?;
    extract_signatures_for_language(&code, lang)
}

fn read_file_safely(path: &Path) -> TalosResult<String> {
    fs::read_to_string(path).map_err(TalosError::Io)
}

fn extract_signatures_for_language(code: &str, lang: SupportedLang) -> TalosResult<Vec<String>> {
    match lang {
        SupportedLang::JavaScript => js_ts::extract_js(code),
        SupportedLang::TypeScript => js_ts::extract_ts(code),
        SupportedLang::TypeScriptReact => js_ts::extract_tsx(code),
        SupportedLang::CSS => css::extract_css(code),
    }
}
