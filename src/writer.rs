use crate::error::{TalosError, TalosResult};
use crate::model::Document;
use crate::types::OutputPath;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn write_output(doc: &Document, output: &OutputPath) -> TalosResult<()> {
    let json = serde_json::to_string_pretty(doc)?;

    match output {
        OutputPath::Stdout => {
            println!("{}", json);
            Ok(())
        }
        OutputPath::File(path) => write_to_file(&json, path),
    }
}

fn write_to_file(json: &str, output_path: &Path) -> TalosResult<()> {
    ensure_parent_directory(output_path)?;

    let temp_file = TempFile::new(output_path)?;
    temp_file.write_content(json)?;
    temp_file.commit(output_path)?;

    Ok(())
}

fn ensure_parent_directory(path: &Path) -> TalosResult<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

struct TempFile {
    path: PathBuf,
}

impl TempFile {
    fn new(target_path: &Path) -> TalosResult<Self> {
        let dir = target_path
            .parent()
            .ok_or_else(|| TalosError::InvalidInput("Invalid output path".to_string()))?;
        let base_name = target_path.file_name().unwrap_or_default();

        let temp_path = generate_temp_path(dir, base_name);
        Ok(TempFile { path: temp_path })
    }

    fn write_content(&self, content: &str) -> TalosResult<()> {
        let mut file = File::create(&self.path)?;
        file.write_all(content.as_bytes())?;
        file.flush()?;
        Ok(())
    }

    fn commit(self, target_path: &Path) -> TalosResult<()> {
        // Best-effort atomic replace
        if target_path.exists() && cfg!(windows) {
            // On Windows, remove existing file before rename
            let _ = fs::remove_file(target_path);
        }

        fs::rename(&self.path, target_path)?;
        Ok(())
    }
}

impl Drop for TempFile {
    fn drop(&mut self) {
        // Clean up temp file if commit wasn't called
        let _ = fs::remove_file(&self.path);
    }
}

fn generate_temp_path(dir: &Path, base_name: &std::ffi::OsStr) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_micros();
    let pid = std::process::id();
    let temp_name = format!(".{}.{}.{}.tmp", base_name.to_string_lossy(), pid, timestamp);
    dir.join(temp_name)
}
