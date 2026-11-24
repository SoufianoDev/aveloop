//! Architecture checks: `domain` must stay independent of UI, debugger, filesystem, and OS process APIs.
//!
//! Source-level scanning is used because Rust has no lint for module dependency direction.
//!
//! shortcut: grouped imports (`use std::{fs, process}`) are not expanded, so only contiguous
//! `std::fs`-style paths are caught; replace with a `syn`-based AST check if the domain starts
//! grouping std imports.

use std::fs;
use std::path::{Path, PathBuf};

const FORBIDDEN_IDENTIFIERS: &[&str] =
    &["gpui", "gpui_kit", "dap", "tree_sitter", "tokio", "reqwest"];

const FORBIDDEN_PATHS: &[&str] = &[
    "std::fs",
    "std::process",
    "std::net",
    "std::env",
    "crate::presentation",
    "crate::infrastructure",
    "crate::platform",
    "crate::app",
];

#[test]
fn domain_dependencies_respect_architecture_boundaries() {
    let domain = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/domain");
    let mut files = Vec::new();
    collect_rust_files(&domain, &mut files);
    assert!(
        !files.is_empty(),
        "no domain sources found under {domain:?}"
    );

    let mut violations = Vec::new();
    for path in files {
        let source = fs::read_to_string(&path).expect("read domain source");
        let rel = path
            .strip_prefix(env!("CARGO_MANIFEST_DIR"))
            .unwrap_or(&path);
        violations.extend(check_source(&rel.display().to_string(), &source));
    }

    assert!(
        violations.is_empty(),
        "domain leaks outside its boundary:\n{}",
        violations.join("\n")
    );
}

#[test]
fn checker_flags_forbidden_dependencies() {
    for source in [
        "use gpui_kit::Application;\n",
        "use gpui::*;\n",
        "use dap::Message;\n",
        "use tree_sitter::Parser;\n",
        "use std::process::Command;\n",
        "use std::fs::File;\n",
        "use crate::presentation::Workspace;\n",
        "use crate::infrastructure::debugger::Adapter;\n",
    ] {
        assert!(
            !check_source("fixture.rs", source).is_empty(),
            "checker missed: {source}"
        );
    }
}

#[test]
fn checker_accepts_domain_style_code() {
    for source in [
        "use serde::{Deserialize, Serialize};\n",
        "use crate::domain::execution::ExecutionTrace;\n",
        "// this module must not import gpui or std::process\n",
        "pub fn label(&self) -> &str { \"gpui\" }\n",
        "use crate::application::ports::DebuggerPort;\n",
    ] {
        assert!(
            check_source("fixture.rs", source).is_empty(),
            "false positive on: {source}"
        );
    }
}

fn check_source(name: &str, source: &str) -> Vec<String> {
    let code = strip_comments_and_literals(source);
    let identifiers: Vec<&str> = code
        .split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .filter(|s| !s.is_empty())
        .collect();
    let compact: String = code
        .chars()
        .map(|c| if c.is_whitespace() { '\u{1}' } else { c })
        .collect();

    let mut violations = Vec::new();
    for forbidden in FORBIDDEN_IDENTIFIERS {
        if identifiers.contains(forbidden) {
            violations.push(format!("{name}: imports `{forbidden}`"));
        }
    }
    for forbidden in FORBIDDEN_PATHS {
        if has_path(&compact, forbidden) {
            violations.push(format!("{name}: imports `{forbidden}`"));
        }
    }
    violations
}

fn has_path(compact: &str, path: &str) -> bool {
    let mut from = 0;
    while let Some(found) = compact[from..].find(path) {
        let start = from + found;
        let end = start + path.len();
        let boundary_before =
            start == 0 || !compact[..start].ends_with(|c: char| c.is_alphanumeric() || c == '_');
        let boundary_after = compact[end..]
            .chars()
            .next()
            .is_none_or(|c| matches!(c, ':' | ';' | ',' | '{' | '}' | '\u{1}'));
        if boundary_before && boundary_after {
            return true;
        }
        from = end;
    }
    false
}

fn collect_rust_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("read domain directory") {
        let path = entry.expect("directory entry").path();
        if path.is_dir() {
            collect_rust_files(&path, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(path);
        }
    }
}

fn strip_comments_and_literals(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '/' => match chars.peek() {
                Some('/') => {
                    chars.next();
                    for c in chars.by_ref() {
                        if c == '\n' {
                            out.push('\n');
                            break;
                        }
                    }
                }
                Some('*') => {
                    chars.next();
                    let mut prev = '\0';
                    for c in chars.by_ref() {
                        if prev == '*' && c == '/' {
                            break;
                        }
                        prev = c;
                    }
                }
                _ => out.push('/'),
            },
            '"' => {
                while let Some(c) = chars.next() {
                    if c == '\\' {
                        chars.next();
                    } else if c == '"' {
                        break;
                    }
                }
            }
            '\'' => {
                let mut lookahead = chars.clone();
                let is_char_literal = match lookahead.next() {
                    Some('\\') => lookahead.next().is_some() && lookahead.next() == Some('\''),
                    Some(_) => lookahead.next() == Some('\''),
                    None => false,
                };
                if is_char_literal {
                    while let Some(c) = chars.next() {
                        if c == '\\' {
                            chars.next();
                        } else if c == '\'' {
                            break;
                        }
                    }
                } else {
                    out.push('\'');
                }
            }
            _ => out.push(c),
        }
    }
    out
}
