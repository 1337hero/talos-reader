use crate::error::{TalosError, TalosResult};
use crate::extractor::extract_signatures_for_file;
use crate::model::{DirectoryEntry, Document, ErrorEntry, FileEntry};
use crate::types::{Extensions, GlobPatterns, MaxFileSize};
use globset::{Glob, GlobSet, GlobSetBuilder};
use ignore::WalkBuilder;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use time::OffsetDateTime;

pub struct ScanOptions {
    pub allowed_exts: Extensions,
    pub include_globs: GlobPatterns,
    pub exclude_globs: GlobPatterns,
    pub max_file_size: Option<MaxFileSize>,
    pub terse_output: bool,
}

pub fn scan_project(root: &Path, opts: &ScanOptions) -> TalosResult<(Document, Vec<ErrorEntry>)> {
    let root = root.canonicalize()?;
    let ts = format_timestamp()?;

    let (include_set, exclude_set) = build_globsets(&opts.include_globs, &opts.exclude_globs)?;
    let allowed_extensions = &opts.allowed_exts;

    // Walk and group files by directory
    let mut by_dir: BTreeMap<PathBuf, Vec<PathBuf>> = BTreeMap::new();

    let mut walker = WalkBuilder::new(&root);
    walker
        .hidden(false)
        .follow_links(false)
        .git_ignore(true)
        .parents(true);
    let walker = walker.build();

    for dent in walker {
        let dent = match dent {
            Ok(d) => d,
            Err(_) => continue,
        };
        let p = dent.path();
        if dent.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
            continue;
        }
        if !is_allowed_file(p, allowed_extensions, opts.max_file_size.as_ref()) {
            continue;
        }
        if !included(p, include_set.as_ref(), &exclude_set) {
            continue;
        }

        let dir = p.parent().unwrap_or(&root).to_path_buf();
        by_dir.entry(dir).or_default().push(p.to_path_buf());
    }

    let mut directories: Vec<DirectoryEntry> = Vec::new();
    let mut errors: Vec<ErrorEntry> = Vec::new();

    for (dir_abs, mut files) in by_dir.into_iter() {
        files.sort();
        let mut file_entries: Vec<FileEntry> = Vec::new();

        for file_path in files {
            let rel_file = path_relative_to(&file_path, &root);
            let file_name = extract_file_name(&file_path, &rel_file);

            match extract_signatures_for_file(&file_path) {
                Ok(sigs) => {
                    if opts.terse_output && sigs.is_empty() {
                        continue;
                    }
                    file_entries.push(FileEntry {
                        file_name,
                        relative_file_path: rel_file,
                        last_scanned: ts.clone(),
                        signatures: sigs,
                        summary: None,
                    });
                }
                Err(e) => errors.push(ErrorEntry {
                    path: rel_file,
                    error: e.to_string(),
                }),
            }
        }

        if !file_entries.is_empty() {
            directories.push(DirectoryEntry {
                directory_path: path_relative_to(&dir_abs, &root),
                files: file_entries,
            });
        }
    }

    // Deterministic ordering
    directories.sort_by(|a, b| a.directory_path.cmp(&b.directory_path));

    let doc = crate::model::Document {
        schema_version: "1.0".to_string(),
        last_updated: ts,
        directories,
        errors: Vec::new(),
    };

    Ok((doc, errors))
}

fn format_timestamp() -> TalosResult<String> {
    OffsetDateTime::now_utc()
        .format(&time::format_description::well_known::Rfc3339)
        .map_err(|_| TalosError::ScanError("Failed to format timestamp".to_string()))
}

fn path_relative_to(path: &Path, root: &Path) -> String {
    path.strip_prefix(root)
        .map(|p| {
            if p.as_os_str().is_empty() {
                ".".into()
            } else {
                p.to_string_lossy().to_string()
            }
        })
        .unwrap_or_else(|_| path.to_string_lossy().to_string())
}

fn extract_file_name(file_path: &Path, rel_file: &str) -> String {
    file_path
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| rel_file.to_string())
}

fn is_allowed_file(path: &Path, extensions: &Extensions, max_size: Option<&MaxFileSize>) -> bool {
    if !matches_allowed_ext(path, extensions) {
        return false;
    }

    if let Some(max) = max_size {
        if let Ok(metadata) = fs::metadata(path) {
            if max.exceeds(metadata.len()) {
                return false;
            }
        }
    }

    true
}

fn matches_allowed_ext(path: &Path, extensions: &Extensions) -> bool {
    let ext = path
        .extension()
        .map(|e| e.to_string_lossy().to_ascii_lowercase())
        .unwrap_or_default();
    extensions.contains(&ext)
}

fn build_globsets(
    includes: &GlobPatterns,
    excludes: &GlobPatterns,
) -> TalosResult<(Option<GlobSet>, GlobSet)> {
    let include_set = build_include_globset(includes)?;
    let exclude_set = build_exclude_globset(excludes)?;
    Ok((include_set, exclude_set))
}

fn build_include_globset(includes: &GlobPatterns) -> TalosResult<Option<GlobSet>> {
    if includes.is_empty() {
        return Ok(None);
    }

    let mut builder = GlobSetBuilder::new();
    for pattern in includes.iter() {
        if pattern.trim().is_empty() {
            continue;
        }
        let glob = Glob::new(pattern).map_err(|e| {
            TalosError::ScanError(format!("Invalid include glob '{}': {}", pattern, e))
        })?;
        builder.add(glob);
    }

    let globset = builder
        .build()
        .map_err(|e| TalosError::ScanError(format!("Failed to build include globset: {}", e)))?;
    Ok(Some(globset))
}

fn build_exclude_globset(excludes: &GlobPatterns) -> TalosResult<GlobSet> {
    let mut builder = GlobSetBuilder::new();

    // Add sensible defaults
    for pattern in DEFAULT_EXCLUDES {
        let glob = Glob::new(pattern).map_err(|e| {
            TalosError::ScanError(format!("Invalid default exclude pattern: {}", e))
        })?;
        builder.add(glob);
    }

    // Add user-specified excludes
    for pattern in excludes.iter() {
        if pattern.trim().is_empty() {
            continue;
        }
        let glob = Glob::new(pattern).map_err(|e| {
            TalosError::ScanError(format!("Invalid exclude glob '{}': {}", pattern, e))
        })?;
        builder.add(glob);
    }

    builder
        .build()
        .map_err(|e| TalosError::ScanError(format!("Failed to build exclude globset: {}", e)))
}

const DEFAULT_EXCLUDES: &[&str] = &["node_modules/**", "dist/**", ".git/**", "coverage/**"];

fn included(path: &Path, includes: Option<&GlobSet>, excludes: &GlobSet) -> bool {
    if excludes.is_match(path) {
        return false;
    }
    if let Some(gs) = includes {
        return gs.is_match(path);
    }
    true
}
