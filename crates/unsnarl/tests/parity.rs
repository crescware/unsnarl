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
//! ## Pruning variants
//!
//! Some fixtures additionally carry sibling `pruned-<slug>/`
//! directories whose expected baselines reflect a pruned visual
//! graph. The TS test-snapshot setup (`ts/integration/fixture-snapshot.ts`
//! invoked from `index.test.ts`) declares the underlying pruning
//! options inline. The Rust harness reads the same options from an
//! adjacent `variants.json` manifest in the fixture root (one entry
//! per `pruned-*` slug) and runs the pipeline with the matching
//! [`PruningRunOptions`] so the pruned baselines stay covered.

use std::fs;
use std::path::{Path, PathBuf};

use libtest_mimic::{Arguments, Failed, Trial};
use pretty_assertions::StrComparison;
use unsnarl_emitter_mermaid::strategy::MermaidStrategy;
use unsnarl_emitter_mermaid::theme::DARK_THEME;
use unsnarl_root_query::{parse_root_queries, ParsedRootQuery};

use unsnarl::pipeline::prune::PruningRunOptions;
use unsnarl::pipeline::{
    emit_ir_text, emit_json_text, emit_markdown_text, emit_mermaid_text, emit_stats_text,
    language_for_path,
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
/// is `Some`, the pipeline is run with those options before
/// comparing against the variant's pruned baseline.
struct FixtureCase {
    name: String,
    input: PathBuf,
    expected: PathBuf,
    rel_source_path: String,
    baseline: Baseline,
    pruning: Option<PruningRunOptions>,
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
            });
        }
        // Generate variant cases from a sibling `variants.json`
        // manifest, if one is present. Each variant entry pulls in
        // every baseline that exists under `pruned-<slug>/`. The
        // baseline list intentionally excludes `Ir`: pruning only
        // narrows the downstream `VisualGraph`, so the IR snapshot
        // matches the baseline and the TS side does not record a
        // per-variant IR fixture either.
        for variant in read_variants(dir) {
            let variant_dir = dir.join(format!("pruned-{}", variant.slug));
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
                let pruning = build_pruning(&variant);
                out.push(FixtureCase {
                    name: format!(
                        "{rel_name}/pruned-{}::{}",
                        variant.slug,
                        baseline.test_suffix()
                    ),
                    input: input.clone(),
                    expected,
                    rel_source_path: rel_source.clone(),
                    baseline,
                    pruning: Some(pruning),
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

/// Per-fixture pruning variant declared in `variants.json`.
///
/// One entry per `pruned-<slug>/` sibling directory. Mirrors the
/// `pruning` arg of `fixtureSnapshot` in
/// `ts/integration/fixture-snapshot.ts`.
struct VariantSpec {
    slug: String,
    roots: String,
    descendants: u32,
    ancestors: u32,
}

fn read_variants(dir: &Path) -> Vec<VariantSpec> {
    let manifest = dir.join("variants.json");
    let Ok(text) = fs::read_to_string(&manifest) else {
        return Vec::new();
    };
    parse_variants_json(&text).unwrap_or_else(|e| {
        panic!(
            "failed to parse {}: {e}\nThe manifest must be a JSON object \
             {{\"variants\": [{{\"slug\": ..., \"roots\": ..., \
             \"descendants\": ..., \"ancestors\": ...}}, ...]}}.",
            manifest.display()
        )
    })
}

/// Minimal JSON manifest parser. The schema is fixed (`variants[]`
/// of `{slug, roots, descendants, ancestors}`) so a hand-written
/// parser avoids pulling `serde` into the dev-dep surface.
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
        let roots = v
            .get("roots")
            .and_then(|s| s.as_str())
            .ok_or_else(|| format!("variant {slug}: missing 'roots'"))?;
        let descendants = v
            .get("descendants")
            .and_then(|n| n.as_u64())
            .ok_or_else(|| format!("variant {slug}: missing 'descendants'"))?
            as u32;
        let ancestors =
            v.get("ancestors")
                .and_then(|n| n.as_u64())
                .ok_or_else(|| format!("variant {slug}: missing 'ancestors'"))? as u32;
        out.push(VariantSpec {
            slug: slug.to_string(),
            roots: roots.to_string(),
            descendants,
            ancestors,
        });
    }
    Ok(out)
}

fn build_pruning(v: &VariantSpec) -> PruningRunOptions {
    let queries: Vec<ParsedRootQuery> = parse_root_queries(&v.roots).unwrap_or_else(|e| {
        panic!(
            "variant {}: parse_root_queries({}) failed: {e}",
            v.slug, v.roots
        )
    });
    PruningRunOptions {
        roots: queries,
        descendants: v.descendants,
        ancestors: v.ancestors,
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
    let pruning = case.pruning.as_ref();
    let actual = match case.baseline {
        Baseline::Ir => emit_ir_text(&code, &case.rel_source_path, language, true)
            .map_err(|e| Failed::from(format!("emit_ir_text failed: {e:?}")))?,
        Baseline::Json => emit_json_text(&code, &case.rel_source_path, language, true, pruning)
            .map_err(|e| Failed::from(format!("emit_json_text failed: {e:?}")))?,
        Baseline::Mermaid => emit_mermaid_text(
            &code,
            &case.rel_source_path,
            language,
            MermaidStrategy::Elk,
            &DARK_THEME,
            false,
            pruning,
        )
        .map_err(|e| Failed::from(format!("emit_mermaid_text failed: {e:?}")))?,
        Baseline::Markdown => emit_markdown_text(
            &code,
            &case.rel_source_path,
            language,
            MermaidStrategy::Elk,
            &DARK_THEME,
            false,
            pruning,
        )
        .map_err(|e| Failed::from(format!("emit_markdown_text failed: {e:?}")))?,
        Baseline::Stats => emit_stats_text(&code, &case.rel_source_path, language, pruning)
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
