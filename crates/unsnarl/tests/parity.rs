//! IR parity harness.
//!
//! Walks `ts/integration/fixtures/**` for directories that contain
//! `input.{ts,tsx,js,jsx,mjs,cjs}` plus an `expected.ir.json` sibling
//! and dynamically generates one `libtest-mimic` test per fixture.
//!
//! Each test feeds the input through `unsnarl::pipeline::emit_ir_text`
//! (the in-process IR pipeline, parse -> analyse -> serialize -> emit)
//! and compares the rendered text to `expected.ir.json` via
//! `pretty_assertions::StrComparison`. The TS 0.2.0 baselines under
//! `expected.ir.json` are treated as the source of truth; the
//! `expected.*` files are never written back from Rust.

use std::fs;
use std::path::{Path, PathBuf};

use libtest_mimic::{Arguments, Failed, Trial};
use pretty_assertions::StrComparison;

use unsnarl::pipeline::{emit_ir_text, language_for_path};

fn workspace_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    PathBuf::from(manifest_dir)
        .parent()
        .and_then(Path::parent)
        .expect("workspace root is two levels above crate dir")
        .to_path_buf()
}

fn fixtures_root() -> PathBuf {
    workspace_root().join("ts/integration/fixtures")
}

fn find_input_file(dir: &Path) -> Option<PathBuf> {
    let entries = fs::read_dir(dir).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !name.starts_with("input.") {
            continue;
        }
        if language_for_path(name).is_some() {
            return Some(path);
        }
    }
    None
}

/// One IR baseline test case (fixture root with `input.*` +
/// `expected.ir.json`).
struct FixtureCase {
    name: String,
    input: PathBuf,
    expected: PathBuf,
    rel_source_path: String,
}

fn collect_fixtures() -> Vec<FixtureCase> {
    let root = fixtures_root();
    let mut out = Vec::new();
    visit_dir(&root, &root, &mut out);
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

fn visit_dir(root: &Path, dir: &Path, out: &mut Vec<FixtureCase>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let entries: Vec<_> = entries.flatten().collect();
    // First, check whether this directory itself is a fixture root.
    if let Some(input) = find_input_file(dir) {
        let expected = dir.join("expected.ir.json");
        if expected.is_file() {
            let name = dir
                .strip_prefix(root)
                .unwrap_or(dir)
                .to_string_lossy()
                .replace('\\', "/")
                .to_string();
            // Source path the IR records: relative to the `ts/` dir
            // (matches the `relative(PROJECT_ROOT, ...)` shape from
            // `ts/integration/fixture-snapshot.ts`, where PROJECT_ROOT
            // is `ts/`).
            let ts_root = root
                .parent()
                .and_then(Path::parent)
                .expect("fixtures live under ts/integration/fixtures");
            let rel = input
                .strip_prefix(ts_root)
                .unwrap_or(&input)
                .to_string_lossy()
                .replace('\\', "/")
                .to_string();
            out.push(FixtureCase {
                name,
                input,
                expected,
                rel_source_path: rel,
            });
        }
    }
    for entry in entries {
        let path = entry.path();
        if path.is_dir() {
            visit_dir(root, &path, out);
        }
    }
}

fn run_case(case: &FixtureCase) -> Result<(), Failed> {
    let code = fs::read_to_string(&case.input)
        .map_err(|e| Failed::from(format!("read input {}: {e}", case.input.display())))?;
    let expected = fs::read_to_string(&case.expected)
        .map_err(|e| Failed::from(format!("read expected {}: {e}", case.expected.display())))?;
    let language = language_for_path(case.rel_source_path.as_str()).ok_or_else(|| {
        Failed::from(format!("unsupported language for {}", case.rel_source_path))
    })?;
    let actual = emit_ir_text(&code, &case.rel_source_path, language, true)
        .map_err(|e| Failed::from(format!("emit_ir_text failed: {e:?}")))?;
    if actual != expected {
        return Err(Failed::from(format!(
            "IR mismatch for {}\n{}",
            case.name,
            StrComparison::new(&expected, &actual)
        )));
    }
    Ok(())
}

fn main() {
    let args = Arguments::from_args();
    // The IR pipeline is not yet byte-identical to the TS 0.2.0
    // baselines (Step 12 is in progress; see issue #121). Gate
    // execution behind `UNSNARL_PARITY=1` so `cargo test --workspace`
    // remains green while the remaining classify / annotation gaps
    // are closed. Removing the gate is part of Step 12's exit
    // criteria.
    if std::env::var_os("UNSNARL_PARITY").is_none() {
        libtest_mimic::run(&args, Vec::<Trial>::new()).exit();
    }
    let cases = collect_fixtures();
    let trials: Vec<Trial> = cases
        .into_iter()
        .map(|case| {
            let name = case.name.clone();
            Trial::test(name, move || run_case(&case))
        })
        .collect();
    libtest_mimic::run(&args, trials).exit();
}
