use std::fmt;

/// Unified error type for the entire application
#[derive(Debug)]
pub enum TalosError {
    Io(std::io::Error),
    Serialization(serde_json::Error),
    TreeSitter(tree_sitter::QueryError),
    InvalidInput(String),
    ScanError(String),
}

impl fmt::Display for TalosError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TalosError::Io(err) => write!(f, "IO error: {}", err),
            TalosError::Serialization(err) => write!(f, "Serialization error: {}", err),
            TalosError::TreeSitter(err) => write!(f, "Tree-sitter query error: {}", err),
            TalosError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            TalosError::ScanError(msg) => write!(f, "Scan error: {}", msg),
        }
    }
}

impl std::error::Error for TalosError {}

impl From<std::io::Error> for TalosError {
    fn from(err: std::io::Error) -> Self {
        TalosError::Io(err)
    }
}

impl From<serde_json::Error> for TalosError {
    fn from(err: serde_json::Error) -> Self {
        TalosError::Serialization(err)
    }
}

impl From<tree_sitter::QueryError> for TalosError {
    fn from(err: tree_sitter::QueryError) -> Self {
        TalosError::TreeSitter(err)
    }
}

/// Result type alias for convenience
pub type TalosResult<T> = Result<T, TalosError>;
