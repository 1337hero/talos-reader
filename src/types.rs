use std::path::{Path, PathBuf};
use std::str::FromStr;

/// File extensions with validation and normalization
#[derive(Debug, Clone)]
pub struct Extensions(Vec<String>);

impl Default for Extensions {
    fn default() -> Self {
        Extensions(vec![
            "js".to_string(),
            "jsx".to_string(),
            "ts".to_string(),
            "tsx".to_string(),
            "css".to_string(),
        ])
    }
}

impl Extensions {
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.iter().map(|s| s.as_str())
    }

    pub fn contains(&self, ext: &str) -> bool {
        self.0.contains(&ext.to_ascii_lowercase())
    }
}

impl FromStr for Extensions {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let exts: Vec<String> = s
            .split(',')
            .map(|e| e.trim().trim_start_matches('.').to_ascii_lowercase())
            .filter(|e| !e.is_empty())
            .collect();

        if exts.is_empty() {
            Err("No valid extensions provided")
        } else {
            Ok(Extensions(exts))
        }
    }
}

impl From<Extensions> for Vec<String> {
    fn from(extensions: Extensions) -> Self {
        extensions.0
    }
}

/// Output path with proper handling of stdout and file paths
#[derive(Debug, Clone)]
pub enum OutputPath {
    Stdout,
    File(PathBuf),
}

impl OutputPath {
    pub fn new(input: Option<&str>, default_dir: &Path) -> Self {
        match input {
            Some("-") => OutputPath::Stdout,
            Some(path) => OutputPath::File(PathBuf::from(path)),
            None => {
                let mut default_path = default_dir.to_path_buf();
                default_path.push("talos.json");
                OutputPath::File(default_path)
            }
        }
    }

    pub fn is_stdout(&self) -> bool {
        matches!(self, OutputPath::Stdout)
    }

    pub fn as_str(&self) -> String {
        match self {
            OutputPath::Stdout => "-".to_string(),
            OutputPath::File(path) => path.to_string_lossy().to_string(),
        }
    }
}

/// Maximum file size with proper validation
#[derive(Debug, Clone, Copy)]
pub struct MaxFileSize(u64);

impl MaxFileSize {
    pub fn new(size: u64) -> Self {
        MaxFileSize(size)
    }

    pub fn bytes(&self) -> u64 {
        self.0
    }

    pub fn exceeds(&self, size: u64) -> bool {
        size > self.0
    }
}

impl Default for MaxFileSize {
    fn default() -> Self {
        MaxFileSize(10 * 1024 * 1024) // 10MB default
    }
}

/// Glob patterns with validation
#[derive(Debug, Clone)]
pub struct GlobPatterns(Vec<String>);

impl GlobPatterns {
    pub fn new(patterns: Vec<String>) -> Self {
        GlobPatterns(patterns)
    }

    pub fn empty() -> Self {
        GlobPatterns(Vec::new())
    }

    pub fn iter(&self) -> impl Iterator<Item = &String> {
        self.0.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl From<Vec<String>> for GlobPatterns {
    fn from(patterns: Vec<String>) -> Self {
        GlobPatterns(patterns)
    }
}

impl From<GlobPatterns> for Vec<String> {
    fn from(patterns: GlobPatterns) -> Self {
        patterns.0
    }
}
