use std::fs;
use std::path::{Path, PathBuf};

pub struct Violation {
    pub path: PathBuf,
    pub line_number: usize,
    pub line: String,
}

pub fn find_violations(root: &Path, skip_dir_names: &[&str]) -> Vec<Violation> {
    let mut out = Vec::new();
    walk(root, skip_dir_names, &mut out);
    out
}

fn walk(dir: &Path, skip_dir_names: &[&str], out: &mut Vec<Violation>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let Ok(file_type) = entry.file_type() else {
            continue;
        };
        let path = entry.path();
        if file_type.is_dir() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name == "target" || skip_dir_names.contains(&name) {
                continue;
            }
            walk(&path, skip_dir_names, out);
        } else if file_type.is_file() && path.extension().and_then(|e| e.to_str()) == Some("rs") {
            scan_file(&path, out);
        }
    }
}

fn scan_file(path: &Path, out: &mut Vec<Violation>) {
    let Ok(contents) = fs::read_to_string(path) else {
        return;
    };
    for (idx, line) in contents.lines().enumerate() {
        if line_has_forbidden_dead_code(line) {
            out.push(Violation {
                path: path.to_path_buf(),
                line_number: idx + 1,
                line: line.to_string(),
            });
        }
    }
}

pub(crate) fn line_has_forbidden_dead_code(line: &str) -> bool {
    const PREFIXES: &[&str] = &["#[allow(", "#![allow(", "#[expect(", "#![expect("];
    for prefix in PREFIXES {
        let mut rest = line;
        while let Some(idx) = rest.find(prefix) {
            let after = &rest[idx + prefix.len()..];
            let Some(close) = after.find(')') else {
                break;
            };
            let inner = &after[..close];
            // Tokenize the allow/expect list by comma and whitespace so that
            // `dead_code` matches as a whole token, never as a substring of
            // e.g. `dead_code_extra`.
            if inner
                .split(|c: char| c == ',' || c.is_whitespace())
                .any(|tok| tok == "dead_code")
            {
                return true;
            }
            rest = &after[close..];
        }
    }
    false
}

#[cfg(test)]
#[path = "no_allow_dead_code_test.rs"]
mod no_allow_dead_code_test;
