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
//!
//! ## Pruning / depth / highlight variants
//!
//! Some fixtures additionally carry sibling `pruned-<slug>/`,
//! `depth-<slug>/`, `pruned-depth-<slug>/`, `highlight-<slug>/`, or
//! `pruned-highlight-<slug>/` directories whose expected baselines
//! reflect a pruned, depth-collapsed, highlighted, or combined visual
//! graph. The TS test-snapshot setup
//! (`ts/integration/fixture-snapshot.ts` invoked from `index.test.ts`)
//! declares the underlying pruning / depth / highlight options inline.
//! The Rust harness reads the same options from an adjacent
//! `variants.json` manifest in the fixture root (one entry per
//! variant slug) and runs the pipeline with the matching
//! [`PruningRunOptions`] / [`NestingDepths`] / [`HighlightRunOptions`]
//! so the variant baselines stay covered.

use std::fs;
use std::path::{Path, PathBuf};

use libtest_mimic::{Arguments, Failed, Trial};
use pretty_assertions::StrComparison;
use unsnarl_emitter_mermaid::strategy::MermaidStrategy;
use unsnarl_emitter_mermaid::theme::DARK_THEME;
use unsnarl_ir::nesting_kind::{NestingDepth, NestingDepths};
use unsnarl_root_query::{parse_root_queries, ParsedRootQuery};
use unsnarl_visual_graph::highlight::HighlightRunOptions;

use unsnarl::pipeline::prune::PruningRunOptions;
use unsnarl::pipeline::{
    emit_ir_text, emit_json_text, emit_markdown_text, emit_mermaid_text, emit_stats_text,
    language_for_path, PipelineRunOptions,
};

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
    /// `preview.md` (Step 15 markdown baseline). Same CLI defaults
    /// as `Mermaid` — the markdown emitter embeds the mermaid render
    /// inside a fenced ```mermaid block.
    Markdown,
    /// `expected.stats` (Step 16 stats baseline). One TSV row per
    /// visual-graph node followed by a `<N> total` summary; the
    /// emitter has no CLI knobs to thread.
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

    fn test_suffix(self) -> &'static str {
        match self {
            Self::Ir => "ir",
            Self::Json => "json",
            Self::Mermaid => "mermaid",
            Self::Markdown => "markdown",
            Self::Stats => "stats",
        }
    }
}

/// One fixture × one baseline test case (fixture root with
/// `input.*` + the matching `expected.*` baseline). When `pruning`
/// / `depths` are `Some`, the pipeline is run with those options
/// before comparing against the variant's baseline.
struct FixtureCase {
    name: String,
    input: PathBuf,
    expected: PathBuf,
    rel_source_path: String,
    baseline: Baseline,
    pruning: Option<PruningRunOptions>,
    depths: Option<NestingDepths>,
    highlight: Option<HighlightRunOptions>,
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
            out.push(FixtureCase {
                name: format!("{rel_name}::{}", baseline.test_suffix()),
                input: input.clone(),
                expected,
                rel_source_path: rel_source.clone(),
                baseline,
                pruning: None,
                depths: None,
                highlight: None,
            });
        }
        // Generate variant cases from a sibling `variants.json`
        // manifest, if one is present. Each variant entry pulls in
        // every baseline that exists under the variant directory.
        // The baseline list intentionally excludes `Ir`: pruning /
        // depth only narrow the downstream `VisualGraph`, so the IR
        // snapshot matches the baseline and the TS side does not
        // record a per-variant IR fixture either.
        for variant in read_variants(dir) {
            let variant_dir = dir.join(variant.dir_name());
            if !variant_dir.is_dir() {
                continue;
            }
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
                let pruning = variant.pruning();
                let depths = variant.depths.clone();
                let highlight = variant.highlight();
                out.push(FixtureCase {
                    name: format!(
                        "{rel_name}/{}::{}",
                        variant.dir_name(),
                        baseline.test_suffix()
                    ),
                    input: input.clone(),
                    expected,
                    rel_source_path: rel_source.clone(),
                    baseline,
                    pruning,
                    depths,
                    highlight,
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

/// Per-fixture variant declared in `variants.json`.
///
/// One entry per variant sibling directory. Mirrors the `pruning` /
/// `depths` / `highlight` args of `fixtureSnapshot` in
/// `ts/integration/fixture-snapshot.ts`. `kind` selects whether the
/// variant directory is `pruned-<slug>/`, `depth-<slug>/`,
/// `pruned-depth-<slug>/`, `highlight-<slug>/`, or
/// `pruned-highlight-<slug>/`; the field defaults to `pruned` for
/// backward compatibility with the original manifests (which only
/// described pruning variants).
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

    fn needs_highlight(self) -> bool {
        matches!(self, Self::Highlight | Self::PrunedHighlight)
    }
}

/// `highlight` field on a variant. Mirrors `FixtureHighlight` in
/// `ts/integration/fixture-snapshot.ts`:
/// - `Roots` -> `-H` with no inline value (the highlight follows
///   `pruning.roots`; only meaningful alongside a pruned variant).
/// - `Queries(raw)` -> `-H <raw>`. The raw string is fed verbatim to
///   `parse_root_queries`, matching the grammar the CLI accepts.
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
    depths: Option<NestingDepths>,
    highlight: Option<HighlightSpec>,
}

impl VariantSpec {
    fn dir_name(&self) -> String {
        format!("{}-{}", self.kind.dir_prefix(), self.slug)
    }

    fn pruning(&self) -> Option<PruningRunOptions> {
        if !self.kind.needs_pruning() {
            return None;
        }
        let roots = self
            .roots
            .as_deref()
            .unwrap_or_else(|| panic!("variant {}: missing 'roots' for kind 'pruned'", self.slug));
        let queries: Vec<ParsedRootQuery> = parse_root_queries(roots).unwrap_or_else(|e| {
            panic!(
                "variant {}: parse_root_queries({}) failed: {e}",
                self.slug, roots
            )
        });
        Some(PruningRunOptions {
            roots: queries,
            descendants: self.descendants.unwrap_or_else(|| {
                panic!("variant {}: missing 'descendants'", self.slug);
            }),
            ancestors: self.ancestors.unwrap_or_else(|| {
                panic!("variant {}: missing 'ancestors'", self.slug);
            }),
        })
    }

    fn highlight(&self) -> Option<HighlightRunOptions> {
        let spec = self.highlight.as_ref()?;
        Some(match spec {
            HighlightSpec::Roots => HighlightRunOptions::Roots,
            HighlightSpec::Queries(raw) => {
                let queries: Vec<ParsedRootQuery> = parse_root_queries(raw).unwrap_or_else(|e| {
                    panic!(
                        "variant {}: parse_root_queries({}) failed: {e}",
                        self.slug, raw
                    )
                });
                HighlightRunOptions::Queries(queries)
            }
        })
    }
}

fn read_variants(dir: &Path) -> Vec<VariantSpec> {
    let manifest = dir.join("variants.json");
    let Ok(text) = fs::read_to_string(&manifest) else {
        return Vec::new();
    };
    parse_variants_json(&text).unwrap_or_else(|e| {
        panic!(
            "failed to parse {}: {e}\nThe manifest must be a JSON object \
             {{\"variants\": [{{\"kind\": ..., \"slug\": ..., \
             \"roots\": ..., \"descendants\": ..., \"ancestors\": ..., \
             \"depths\": ..., \"highlight\": ...}}, ...]}}.",
            manifest.display()
        )
    })
}

/// Minimal JSON manifest parser. The schema is fixed (`variants[]`
/// of `{kind?, slug, roots?, descendants?, ancestors?, depths?, highlight?}`)
/// so a hand-written parser avoids pulling `serde` into the dev-dep
/// surface beyond the `serde_json::Value` already in use.
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
        if matches!(kind, VariantKind::Depth | VariantKind::PrunedDepth) && depths.is_none() {
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

/// Parse the `highlight` field. Two shapes are accepted, matching the
/// TS `FixtureHighlight` union: `{"mode": "roots"}` for the `-H`
/// no-value form (the highlight follows `pruning.roots`), and
/// `{"mode": "queries", "raw": "<raw>"}` for `-H <raw>`. The raw
/// string is fed verbatim to `parse_root_queries` so multi-token
/// strings (`"a,L7"`) round-trip unchanged.
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

/// Parse the `depths` field. Supports either a uniform shorthand
/// (`{"uniform": N}`), a function-vs-block split
/// (`{"function": N, "block": N}` — matches the CLI flag surface),
/// or a fully per-kind object listing every `NestingKind`. Other
/// shapes are rejected so a typo cannot silently leak default
/// values into a test case.
fn parse_depths(slug: &str, v: &serde_json::Value) -> Result<NestingDepths, String> {
    let obj = v
        .as_object()
        .ok_or_else(|| format!("variant {slug}: 'depths' must be an object"))?;
    let to_depth = |key: &str, n: &serde_json::Value| -> Result<NestingDepth, String> {
        let n = n.as_u64().ok_or_else(|| {
            format!("variant {slug}: 'depths.{key}' must be a non-negative integer")
        })?;
        Ok(NestingDepth(n as u32))
    };
    if let Some(n) = obj.get("uniform") {
        let value = to_depth("uniform", n)?;
        return Ok(NestingDepths::uniform(value));
    }
    if obj.contains_key("function") && obj.contains_key("block") && obj.len() == 2 {
        let function = to_depth("function", &obj["function"])?;
        let block = to_depth("block", &obj["block"])?;
        return Ok(NestingDepths {
            function,
            r#if: block,
            r#for: block,
            r#while: block,
            switch: block,
            try_catch_finally: block,
            block,
        });
    }
    let mut depths = NestingDepths::uniform(NestingDepth(0));
    let expected_keys = [
        "function",
        "if",
        "for",
        "while",
        "switch",
        "try-catch-finally",
        "block",
    ];
    for key in expected_keys {
        let n = obj.get(key).ok_or_else(|| {
            format!("variant {slug}: 'depths.{key}' is required for per-kind form")
        })?;
        let value = to_depth(key, n)?;
        match key {
            "function" => depths.function = value,
            "if" => depths.r#if = value,
            "for" => depths.r#for = value,
            "while" => depths.r#while = value,
            "switch" => depths.switch = value,
            "try-catch-finally" => depths.try_catch_finally = value,
            "block" => depths.block = value,
            _ => unreachable!(),
        }
    }
    if obj.len() != expected_keys.len() {
        return Err(format!(
            "variant {slug}: 'depths' object has unexpected keys; \
             allowed shapes are {{\"uniform\": N}}, \
             {{\"function\": N, \"block\": N}}, \
             or a full per-kind object listing every NestingKind"
        ));
    }
    Ok(depths)
}

fn run_case(case: &FixtureCase) -> Result<(), Failed> {
    let code = fs::read_to_string(&case.input)
        .map_err(|e| Failed::from(format!("read input {}: {e}", case.input.display())))?;
    let expected = fs::read_to_string(&case.expected)
        .map_err(|e| Failed::from(format!("read expected {}: {e}", case.expected.display())))?;
    let language = language_for_path(case.rel_source_path.as_str()).ok_or_else(|| {
        Failed::from(format!("unsupported language for {}", case.rel_source_path))
    })?;
    let run = PipelineRunOptions {
        pruning: case.pruning.as_ref(),
        depths: case.depths.as_ref(),
        highlight: case.highlight.as_ref(),
    };
    let actual = match case.baseline {
        Baseline::Ir => emit_ir_text(&code, &case.rel_source_path, language, true)
            .map_err(|e| Failed::from(format!("emit_ir_text failed: {e:?}")))?,
        Baseline::Json => emit_json_text(&code, &case.rel_source_path, language, true, run)
            .map_err(|e| Failed::from(format!("emit_json_text failed: {e:?}")))?,
        Baseline::Mermaid => emit_mermaid_text(
            &code,
            &case.rel_source_path,
            language,
            MermaidStrategy::Elk,
            &DARK_THEME,
            false,
            run,
        )
        .map_err(|e| Failed::from(format!("emit_mermaid_text failed: {e:?}")))?,
        Baseline::Markdown => emit_markdown_text(
            &code,
            &case.rel_source_path,
            language,
            MermaidStrategy::Elk,
            &DARK_THEME,
            false,
            run,
        )
        .map_err(|e| Failed::from(format!("emit_markdown_text failed: {e:?}")))?,
        Baseline::Stats => emit_stats_text(&code, &case.rel_source_path, language, run)
            .map_err(|e| Failed::from(format!("emit_stats_text failed: {e:?}")))?,
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
