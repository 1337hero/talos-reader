use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Document {
    pub schema_version: String,
    pub last_updated: String,
    pub directories: Vec<DirectoryEntry>,
    #[serde(default)]
    pub errors: Vec<ErrorEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DirectoryEntry {
    pub directory_path: String, // relative to root
    pub files: Vec<FileEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntry {
    pub file_name: String,
    pub relative_file_path: String,
    pub last_scanned: String,
    pub signatures: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorEntry {
    pub path: String,
    pub error: String,
}
