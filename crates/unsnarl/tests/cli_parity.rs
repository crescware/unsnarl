//! End-to-end CLI parity harness.
//!
//! Companion to `tests/parity.rs`. Whereas the in-process parity
//! harness drives the pipeline through `emit_*_text` helpers — which
//! bypass the `crates/unsnarl/src/cli/` argument layer, the
//! `crates/unsnarl/src/run.rs` orchestration, and the
//! `Box<dyn Emitter>` trait-dispatch path that the production
//! binary actually takes — this harness invokes the `uns` binary
//! itself as a subprocess for each baseline. The coverage report
//! therefore lights up the CLI-only code paths (args parsing,
//! emitter selection, stdout / output-path handling) that the
//! in-process harness cannot reach.
//!
//! Scope: baseline-only. Each fixture directory with `input.*` plus
//! one of the five `expected.*` siblings yields five subprocess
//! invocations of `uns -f <format> <input>` whose stdout is
//! compared byte-for-byte to the on-disk baseline. The variant
//! cases (`pruned-*/`, `depth-*/`, `highlight-*/`, `plugin-*/`,
//! `pruned-depth-*/`, `pruned-highlight-*/`) still go through the
//! in-process harness for now; folding them into the CLI sweep
//! requires translating each `variants.json` entry into the
//! equivalent `-r` / `-A` / `-B` / `--depth*` / `-H` / `--plugin`
//! flags and is handled in a follow-up.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use libtest_mimic::{Arguments, Failed, Trial};
use pretty_assertions::StrComparison;

const UNS_BIN: &str = env!("CARGO_BIN_EXE_uns");

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

fn ts_root() -> PathBuf {
    workspace_root().join("ts")
}

#[derive(Clone, Copy)]
enum Baseline {
    Ir,
    Json,
    Mermaid,
    Markdown,
    Stats,
}

impl Baseline {
    fn file_name(self) -> &'static str {
        match self {
            Self::Ir => "expected.ir.json",
            Self::Json => "expected.json",
            Self::Mermaid => "expected.mermaid",
            Self::Markdown => "preview.md",
            Self::Stats => "expected.stats",
        }
    }

    fn cli_format(self) -> &'static str {
        match self {
            Self::Ir => "ir",
            Self::Json => "json",
            Self::Mermaid => "mermaid",
            Self::Markdown => "markdown",
            Self::Stats => "stats",
        }
    }

    fn test_suffix(self) -> &'static str {
        self.cli_format()
    }
}

struct CliCase {
    name: String,
    input: PathBuf,
    expected: PathBuf,
    baseline: Baseline,
}

fn is_supported_input(name: &str) -> bool {
    matches!(
        Path::new(name).extension().and_then(|e| e.to_str()),
        Some("ts") | Some("tsx") | Some("js") | Some("jsx") | Some("mjs") | Some("cjs")
    )
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
        if is_supported_input(name) {
            return Some(path);
        }
    }
    None
}

fn visit_dir(root: &Path, dir: &Path, out: &mut Vec<CliCase>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let entries: Vec<_> = entries.flatten().collect();
    if let Some(input) = find_input_file(dir) {
        let rel_name = dir
            .strip_prefix(root)
            .unwrap_or(dir)
            .to_string_lossy()
            .replace('\\', "/")
            .to_string();
        for baseline in [
            Baseline::Ir,
            Baseline::Json,
            Baseline::Mermaid,
            Baseline::Markdown,
            Baseline::Stats,
        ] {
            let expected = dir.join(baseline.file_name());
            if !expected.is_file() {
                continue;
            }
            out.push(CliCase {
                name: format!("{rel_name}::{}", baseline.test_suffix()),
                input: input.clone(),
                expected,
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

fn collect_cases() -> Vec<CliCase> {
    let root = fixtures_root();
    let mut out = Vec::new();
    visit_dir(&root, &root, &mut out);
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

fn run_case(case: &CliCase) -> Result<(), Failed> {
    let expected = fs::read_to_string(&case.expected)
        .map_err(|e| Failed::from(format!("read expected {}: {e}", case.expected.display())))?;
    // Run with cwd = ts/ and feed the input as a relative path so the
    // CLI's recorded `source` field matches the parity baselines,
    // which embed paths shaped like `integration/fixtures/.../input.ts`
    // (the `relative(PROJECT_ROOT, ...)` form `fixture-snapshot.ts`
    // emits under `PROJECT_ROOT = ts/`).
    let ts = ts_root();
    let rel_input = case.input.strip_prefix(&ts).unwrap_or(&case.input);
    let output = Command::new(UNS_BIN)
        .current_dir(&ts)
        .arg("-f")
        .arg(case.baseline.cli_format())
        .arg(rel_input)
        .output()
        .map_err(|e| Failed::from(format!("spawn {UNS_BIN}: {e}")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Failed::from(format!(
            "{} exited with {} for {}\nstderr:\n{}",
            UNS_BIN, output.status, case.name, stderr
        )));
    }
    let actual = String::from_utf8(output.stdout)
        .map_err(|e| Failed::from(format!("invalid utf-8 stdout for {}: {e}", case.name)))?;
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
    let cases = collect_cases();
    let trials: Vec<Trial> = cases
        .into_iter()
        .map(|case| {
            let name = case.name.clone();
            Trial::test(name, move || run_case(&case))
        })
        .collect();
    libtest_mimic::run(&args, trials).exit();
}
