//! Parity harness.
//!
//! Walks `ts/integration/fixtures/**` for directories that contain
//! `input.{ts,tsx,js,jsx,mjs,cjs}` plus one of the `expected.*`
//! sibling baselines and dynamically generates one `libtest-mimic`
//! test per (fixture, baseline) pair.
//!
//! Each test feeds the input through the matching in-process
//! pipeline helper (`emit_ir_text` for `expected.ir.json`,
//! `emit_json_text` for `expected.json`) and compares the rendered
//! text to the on-disk baseline via `pretty_assertions::StrComparison`.
//! The TS baselines are treated as the source of truth; the
//! `expected.*` files are never written back from Rust.

use std::fs;
use std::path::{Path, PathBuf};

use libtest_mimic::{Arguments, Failed, Trial};
use pretty_assertions::StrComparison;
use unsnarl_emitter_mermaid::strategy::MermaidStrategy;
use unsnarl_emitter_mermaid::theme::DARK_THEME;

use unsnarl::pipeline::{emit_ir_text, emit_json_text, emit_mermaid_text, language_for_path};

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

/// Which baseline a [`FixtureCase`] checks against.
#[derive(Clone, Copy)]
enum Baseline {
    /// `expected.ir.json` (Step 12 baseline).
    Ir,
    /// `expected.json` (Step 13 visual-graph JSON baseline).
    Json,
    /// `expected.mermaid` (Step 14 mermaid baseline). The harness
    /// renders with the CLI defaults (elk strategy + dark theme,
    /// `--debug` off) so the test bytes match the same defaults
    /// the TS port records on disk.
    Mermaid,
}

impl Baseline {
    fn file_name(self) -> &'static str {
        match self {
            Self::Ir => "expected.ir.json",
            Self::Json => "expected.json",
            Self::Mermaid => "expected.mermaid",
        }
    }

    fn test_suffix(self) -> &'static str {
        match self {
            Self::Ir => "ir",
            Self::Json => "json",
            Self::Mermaid => "mermaid",
        }
    }
}

/// One fixture × one baseline test case (fixture root with
/// `input.*` + the matching `expected.*` baseline).
struct FixtureCase {
    name: String,
    input: PathBuf,
    expected: PathBuf,
    rel_source_path: String,
    baseline: Baseline,
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
        let rel_name = dir
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
        let rel_source = input
            .strip_prefix(ts_root)
            .unwrap_or(&input)
            .to_string_lossy()
            .replace('\\', "/")
            .to_string();
        for baseline in [Baseline::Ir, Baseline::Json, Baseline::Mermaid] {
            let expected = dir.join(baseline.file_name());
            if !expected.is_file() {
                continue;
            }
            out.push(FixtureCase {
                name: format!("{rel_name}::{}", baseline.test_suffix()),
                input: input.clone(),
                expected,
                rel_source_path: rel_source.clone(),
                baseline,
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
    let actual = match case.baseline {
        Baseline::Ir => emit_ir_text(&code, &case.rel_source_path, language, true)
            .map_err(|e| Failed::from(format!("emit_ir_text failed: {e:?}")))?,
        Baseline::Json => emit_json_text(&code, &case.rel_source_path, language, true)
            .map_err(|e| Failed::from(format!("emit_json_text failed: {e:?}")))?,
        Baseline::Mermaid => emit_mermaid_text(
            &code,
            &case.rel_source_path,
            language,
            MermaidStrategy::Elk,
            &DARK_THEME,
            false,
        )
        .map_err(|e| Failed::from(format!("emit_mermaid_text failed: {e:?}")))?,
    };
    if actual != expected {
        return Err(Failed::from(format!(
            "{} mismatch for {}\n{}",
            case.baseline.test_suffix(),
            case.name,
            StrComparison::new(&expected, &actual)
        )));
    }
    Ok(())
}

fn main() {
    let args = Arguments::from_args();
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
