//! End-to-end CLI parity harness.
//!
//! Invokes the `uns` binary itself as a subprocess for each
//! baseline, covering the CLI-only code paths (args parsing,
//! emitter selection, stdout / output-path handling).
//!
//! Scope: baseline + variant. Every fixture root that carries
//! `input.*` plus one of the five `expected.*` siblings yields one
//! subprocess invocation per format. The same variant tree the
//! in-process harness consumes from
//! `crates/unsnarl/tests/fixture-variants/<rel>/variants.json` (and
//! `plugin-<slug>/` auto-discovery) is also walked here; each
//! variant entry is translated into the equivalent
//! `-r` / `-A` / `-B` / `--depth*` / `-H` / `--plugin` invocation,
//! and the variant directory's baselines (`Json`/`Mermaid`/
//! `Markdown`/`Stats` for non-plugin variants — `Ir` is intentionally
//! omitted because pruning / depth / highlight only narrow the
//! downstream `VisualGraph` and the parent IR stays identical — plus
//! `Ir` for plugin variants because plugin transforms reshape the IR
//! itself) are compared byte-for-byte to the subprocess stdout.
//!
//! Per-kind depth variants (the programmatic-only form documented in
//! `format_depth_query.rs`) are intentionally rejected at manifest
//! parse time: the CLI surface exposes only `--depth`,
//! `--depth-function`, and `--depth-block`, so a per-kind manifest
//! cannot be reconstructed from CLI flags. If such an entry ever
//! appears, the parser panics loudly rather than silently dropping
//! the case.

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
    workspace_root().join("integration/fixtures")
}

/// Root of the per-fixture `variants.json` manifests. The fixtures
/// tree is treated as immutable; the manifests live alongside the
/// harness.
fn fixture_variants_root() -> PathBuf {
    workspace_root().join("crates/unsnarl/tests/fixture-variants")
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
    /// Extra CLI flags appended after `-f <format>` and before the
    /// input path. Empty for baseline cases; populated from a
    /// `variants.json` entry or a `plugin-<slug>/` auto-discovery
    /// for variant cases.
    extra_args: Vec<String>,
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

#[derive(Clone, Copy)]
enum VariantKind {
    Pruned,
    Depth,
    PrunedDepth,
    Highlight,
    PrunedHighlight,
}

impl VariantKind {
    fn parse(s: &str) -> Result<Self, String> {
        match s {
            "pruned" => Ok(Self::Pruned),
            "depth" => Ok(Self::Depth),
            "pruned-depth" => Ok(Self::PrunedDepth),
            "highlight" => Ok(Self::Highlight),
            "pruned-highlight" => Ok(Self::PrunedHighlight),
            other => Err(format!(
                "unknown variant kind '{other}' (expected 'pruned', 'depth', \
                 'pruned-depth', 'highlight', or 'pruned-highlight')"
            )),
        }
    }

    fn dir_prefix(self) -> &'static str {
        match self {
            Self::Pruned => "pruned",
            Self::Depth => "depth",
            Self::PrunedDepth => "pruned-depth",
            Self::Highlight => "highlight",
            Self::PrunedHighlight => "pruned-highlight",
        }
    }

    fn needs_pruning(self) -> bool {
        matches!(
            self,
            Self::Pruned | Self::PrunedDepth | Self::PrunedHighlight
        )
    }

    fn needs_depths(self) -> bool {
        matches!(self, Self::Depth | Self::PrunedDepth)
    }

    fn needs_highlight(self) -> bool {
        matches!(self, Self::Highlight | Self::PrunedHighlight)
    }
}

/// Subset of depth shapes the CLI surface can reconstruct. The full
/// per-kind form is intentionally rejected (see module docs) — it
/// has no CLI flag.
enum DepthsSpec {
    Uniform(u32),
    FunctionBlock { function: u32, block: u32 },
}

enum HighlightSpec {
    Roots,
    Queries(String),
}

struct VariantSpec {
    kind: VariantKind,
    slug: String,
    roots: Option<String>,
    descendants: Option<u32>,
    ancestors: Option<u32>,
    depths: Option<DepthsSpec>,
    highlight: Option<HighlightSpec>,
}

impl VariantSpec {
    fn dir_name(&self) -> String {
        format!("{}-{}", self.kind.dir_prefix(), self.slug)
    }

    /// Convert the variant into the CLI flag sequence the in-process
    /// `PipelineRunOptions` would otherwise carry. The flag order
    /// mirrors the CLI's docstring ordering (`-r`, `-A`, `-B`,
    /// `--depth*`, `-H`); the binary is order-insensitive so this is
    /// only for readability when a test fails.
    fn build_cli_args(&self) -> Vec<String> {
        let mut args = Vec::new();
        if self.kind.needs_pruning() {
            let roots = self.roots.as_deref().unwrap_or_else(|| {
                panic!("variant {}: kind requires 'roots'", self.slug);
            });
            args.push("-r".into());
            args.push(roots.into());
            let descendants = self.descendants.unwrap_or_else(|| {
                panic!("variant {}: kind requires 'descendants'", self.slug);
            });
            let ancestors = self.ancestors.unwrap_or_else(|| {
                panic!("variant {}: kind requires 'ancestors'", self.slug);
            });
            args.push("-A".into());
            args.push(descendants.to_string());
            args.push("-B".into());
            args.push(ancestors.to_string());
        }
        if let Some(d) = &self.depths {
            match d {
                DepthsSpec::Uniform(n) => {
                    args.push("--depth".into());
                    args.push(n.to_string());
                }
                DepthsSpec::FunctionBlock { function, block } => {
                    args.push("--depth-function".into());
                    args.push(function.to_string());
                    args.push("--depth-block".into());
                    args.push(block.to_string());
                }
            }
        }
        if let Some(h) = &self.highlight {
            match h {
                HighlightSpec::Roots => args.push("-H".into()),
                HighlightSpec::Queries(raw) => {
                    args.push("-H".into());
                    args.push(raw.clone());
                }
            }
        }
        args
    }
}

fn read_variants(root: &Path, dir: &Path) -> Vec<VariantSpec> {
    let rel = match dir.strip_prefix(root) {
        Ok(r) => r,
        Err(_) => return Vec::new(),
    };
    let manifest = fixture_variants_root().join(rel).join("variants.json");
    let Ok(text) = fs::read_to_string(&manifest) else {
        return Vec::new();
    };
    parse_variants_json(&text)
        .unwrap_or_else(|e| panic!("failed to parse {}: {e}", manifest.display()))
}

fn parse_variants_json(text: &str) -> Result<Vec<VariantSpec>, String> {
    let value: serde_json::Value = serde_json::from_str(text).map_err(|e| e.to_string())?;
    let arr = value
        .get("variants")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "missing top-level 'variants' array".to_string())?;
    let mut out = Vec::with_capacity(arr.len());
    for v in arr {
        let slug = v
            .get("slug")
            .and_then(|s| s.as_str())
            .ok_or_else(|| "missing 'slug'".to_string())?;
        let kind = match v.get("kind").and_then(|s| s.as_str()) {
            Some(k) => VariantKind::parse(k).map_err(|e| format!("variant {slug}: {e}"))?,
            None => VariantKind::Pruned,
        };
        let roots = v.get("roots").and_then(|s| s.as_str()).map(str::to_string);
        let descendants = v
            .get("descendants")
            .and_then(|n| n.as_u64())
            .map(|n| n as u32);
        let ancestors = v
            .get("ancestors")
            .and_then(|n| n.as_u64())
            .map(|n| n as u32);
        let depths = match v.get("depths") {
            Some(d) => Some(parse_depths(slug, d)?),
            None => None,
        };
        let highlight = match v.get("highlight") {
            Some(h) => Some(parse_highlight(slug, h)?),
            None => None,
        };
        if kind.needs_pruning() && (roots.is_none() || descendants.is_none() || ancestors.is_none())
        {
            return Err(format!(
                "variant {slug}: kind '{}' requires 'roots', 'descendants', and 'ancestors'",
                kind.dir_prefix()
            ));
        }
        if kind.needs_depths() && depths.is_none() {
            return Err(format!(
                "variant {slug}: kind '{}' requires 'depths'",
                kind.dir_prefix()
            ));
        }
        if kind.needs_highlight() && highlight.is_none() {
            return Err(format!(
                "variant {slug}: kind '{}' requires 'highlight'",
                kind.dir_prefix()
            ));
        }
        out.push(VariantSpec {
            kind,
            slug: slug.to_string(),
            roots,
            descendants,
            ancestors,
            depths,
            highlight,
        });
    }
    Ok(out)
}

fn parse_depths(slug: &str, v: &serde_json::Value) -> Result<DepthsSpec, String> {
    let obj = v
        .as_object()
        .ok_or_else(|| format!("variant {slug}: 'depths' must be an object"))?;
    if let Some(n) = obj.get("uniform") {
        let value = n.as_u64().ok_or_else(|| {
            format!("variant {slug}: 'depths.uniform' must be a non-negative integer")
        })?;
        return Ok(DepthsSpec::Uniform(value as u32));
    }
    if obj.contains_key("function") && obj.contains_key("block") && obj.len() == 2 {
        let function = obj["function"].as_u64().ok_or_else(|| {
            format!("variant {slug}: 'depths.function' must be a non-negative integer")
        })?;
        let block = obj["block"].as_u64().ok_or_else(|| {
            format!("variant {slug}: 'depths.block' must be a non-negative integer")
        })?;
        return Ok(DepthsSpec::FunctionBlock {
            function: function as u32,
            block: block as u32,
        });
    }
    Err(format!(
        "variant {slug}: 'depths' uses the per-kind form, which has no CLI \
         equivalent (the CLI exposes only --depth, --depth-function, and \
         --depth-block). Per-kind cases are unit-tested directly in \
         crates/unsnarl-emitter-markdown/src/format_depth_query/."
    ))
}

fn parse_highlight(slug: &str, v: &serde_json::Value) -> Result<HighlightSpec, String> {
    let obj = v
        .as_object()
        .ok_or_else(|| format!("variant {slug}: 'highlight' must be an object"))?;
    let mode = obj
        .get("mode")
        .and_then(|m| m.as_str())
        .ok_or_else(|| format!("variant {slug}: 'highlight.mode' is required"))?;
    match mode {
        "roots" => {
            if obj.len() != 1 {
                return Err(format!(
                    "variant {slug}: 'highlight.mode' is 'roots'; no other keys are allowed"
                ));
            }
            Ok(HighlightSpec::Roots)
        }
        "queries" => {
            let raw = obj.get("raw").and_then(|r| r.as_str()).ok_or_else(|| {
                format!("variant {slug}: 'highlight.raw' is required when mode is 'queries'")
            })?;
            if obj.len() != 2 {
                return Err(format!(
                    "variant {slug}: 'highlight' has unexpected keys; allowed shapes are \
                     {{\"mode\": \"roots\"}} or {{\"mode\": \"queries\", \"raw\": \"...\"}}"
                ));
            }
            Ok(HighlightSpec::Queries(raw.to_string()))
        }
        other => Err(format!(
            "variant {slug}: 'highlight.mode' is '{other}' (expected 'roots' or 'queries')"
        )),
    }
}

/// Scans `dir` for
/// `plugin-<slug>` sibling subdirectories and returns the slugs in
/// stable (sorted) order. The slug is the directory name with the
/// `plugin-` prefix stripped — the same post-strip plugin short name
/// the CLI's `--plugin` flag accepts.
fn discover_plugin_slugs(dir: &Path) -> Vec<String> {
    let Ok(entries) = fs::read_dir(dir) else {
        return Vec::new();
    };
    let mut slugs: Vec<String> = entries
        .flatten()
        .filter_map(|entry| {
            if !entry.path().is_dir() {
                return None;
            }
            let name = entry.file_name();
            let name = name.to_string_lossy();
            name.strip_prefix("plugin-").map(|s| s.to_string())
        })
        .collect();
    slugs.sort();
    slugs
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
                extra_args: Vec::new(),
            });
        }
        // Manifest-driven variants (pruned / depth / highlight and their
        // combinations). Ir is omitted because pruning / depth / highlight
        // only narrow the downstream VisualGraph and the parent IR stays
        // identical.
        for variant in read_variants(root, dir) {
            let variant_dir = dir.join(variant.dir_name());
            if !variant_dir.is_dir() {
                continue;
            }
            let extra = variant.build_cli_args();
            for baseline in [
                Baseline::Json,
                Baseline::Mermaid,
                Baseline::Markdown,
                Baseline::Stats,
            ] {
                let expected = variant_dir.join(baseline.file_name());
                if !expected.is_file() {
                    continue;
                }
                out.push(CliCase {
                    name: format!(
                        "{rel_name}/{}::{}",
                        variant.dir_name(),
                        baseline.test_suffix()
                    ),
                    input: input.clone(),
                    expected,
                    baseline,
                    extra_args: extra.clone(),
                });
            }
        }
        // Plugin variants are auto-discovered. Plugin transforms
        // reshape the IR itself, so the IR baseline IS per-variant.
        for plugin_slug in discover_plugin_slugs(dir) {
            let variant_dir = dir.join(format!("plugin-{plugin_slug}"));
            for baseline in [
                Baseline::Ir,
                Baseline::Json,
                Baseline::Mermaid,
                Baseline::Markdown,
                Baseline::Stats,
            ] {
                let expected = variant_dir.join(baseline.file_name());
                if !expected.is_file() {
                    continue;
                }
                out.push(CliCase {
                    name: format!(
                        "{rel_name}/plugin-{plugin_slug}::{}",
                        baseline.test_suffix()
                    ),
                    input: input.clone(),
                    expected,
                    baseline,
                    extra_args: vec!["--plugin".into(), plugin_slug.clone()],
                });
            }
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
    let ws = workspace_root();
    let rel_input = case.input.strip_prefix(&ws).unwrap_or(&case.input);
    let mut command = Command::new(UNS_BIN);
    command
        .current_dir(&ws)
        .arg("-f")
        .arg(case.baseline.cli_format());
    for arg in &case.extra_args {
        command.arg(arg);
    }
    // `-H` is declared `num_args = 0..=1` in `cli/args.rs`, so a
    // bare `uns ... -H <input>` would let clap grab `<input>` as the
    // `-H` value. Pin positional handling with the standard `--`
    // separator so the input path is unambiguous regardless of
    // which flags `extra_args` carries.
    command.arg("--").arg(rel_input);
    let output = command
        .output()
        .map_err(|e| Failed::from(format!("spawn {UNS_BIN}: {e}")))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Failed::from(format!(
            "{} exited with {} for {}\nargs: {:?}\nstderr:\n{}",
            UNS_BIN, output.status, case.name, case.extra_args, stderr
        )));
    }
    let actual = String::from_utf8(output.stdout)
        .map_err(|e| Failed::from(format!("invalid utf-8 stdout for {}: {e}", case.name)))?;
    if actual != expected {
        return Err(Failed::from(format!(
            "{} mismatch for {}\nargs: {:?}\n{}",
            case.baseline.test_suffix(),
            case.name,
            case.extra_args,
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
