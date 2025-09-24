# ![Talos Reader logo](logo.png)

# Talos Reader

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Open Source Love](https://badges.frapsoft.com/os/v2/open-source.svg?v=103)](https://github.com/rovocms/rovocms)

Talos Reader is a Rust-based CLI that scans codebases and generates structured JSON summaries of directories, files, and function signatures. It helps tame large projects and makes them easier to feed into Large Language Models (LLMs).

## Features
- Extracts JS/TS/CSS signatures via tree-sitter
- Honors .gitignore (using ignore crate)
- Terse mode: skip files with zero signatures
- Deterministic ordering + atomic file writes
- Extensible extractor system (support for more languages planned)

## Use Cases
- **LLM Context Preparation**: Summarize large codebases for LLM analysis  
- **Code Documentation**: Automatically extract API signatures for docs  
- **Codebase Analysis**: Get an overview of structure and functionality  
- **Code Review Assistance**: Understand changes and their impact quickly  


## Install/Build
```bash
cd talos
cargo build
```

## Usage

### Basic:
```bash
# Default run (writes talos.json in input dir)
talos /path/to/project
```

### Custom output:
```bash
# Print to stdout
talos /path/to/project --output -

# Write to a custom file
talos /path/to/project --output my-signatures.json
```
### Filtering:
```bash
# Limit to JS/TS only
talos /path/to/project --ext js,ts,tsx

# Exclude tests
talos /path/to/project --exclude "**/*.test.*" --exclude "**/__tests__/**"

# Include only specific dirs
talos /path/to/project --include "src/**" --include "lib/**"
```


## Output Schema
```json
{
  "schema_version": "1.0",
  "last_updated": "ISO8601",
  "directories": [
    {
      "directory_path": "string",
      "files": [
        {
          "file_name": "string",
          "relative_file_path": "string",
          "last_scanned": "ISO8601",
          "signatures": []
        }
      ]
    }
  ],
  "errors": [
    { "path": "string", "error": "string" }
  ]
}
```

## Contributing
See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.


--- 

Built with ❤️ by [1337Hero](https://github.com/1337hero)
