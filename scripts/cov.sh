#!/usr/bin/env zsh
# Line / region / function coverage for the Rust workspace.
#
# Builds every workspace test target with rustc's
# `-Cinstrument-coverage`, runs `cargo test --workspace` so each test
# binary writes its own `*.profraw`, merges them, and produces a
# per-source-file report via `llvm-cov report`.
#
# All artifacts live under `target/coverage/` so they stay
# git-ignored alongside the rest of the Cargo target tree.
# `target/cov-build/` is used as a sibling Cargo target dir so the
# instrumented build does not invalidate the default `cargo`
# incremental cache (re-flipping `RUSTFLAGS` between calls would
# otherwise force every dep to recompile in `target/`).
#
# Requires the `llvm-tools-preview` rustup component for
# `llvm-profdata` / `llvm-cov`. Install once with:
#
#   rustup component add llvm-tools-preview
#
# Usage:
#   ./scripts/cov.sh                              # full workspace report
#   ./scripts/cov.sh build_analysis_visitor.rs    # filter rows by filename regex

set -u
emulate -L zsh
setopt pipefail

# Script lives at `scripts/cov.sh`; the repo root is the parent
# directory.
REPO_ROOT="${0:A:h:h}"
cd "$REPO_ROOT"

WORK="$REPO_ROOT/target/coverage"
BUILD_DIR="$REPO_ROOT/target/cov-build"
PROF_DIR="$WORK/profraw"
MERGED="$WORK/merged.profdata"

LLVM_BIN_DIR="$(rustc --print sysroot)/lib/rustlib/$(rustc -vV | awk -F' ' '/^host:/ {print $2}')/bin"
PROFDATA="$LLVM_BIN_DIR/llvm-profdata"
LLVMCOV="$LLVM_BIN_DIR/llvm-cov"

if [[ ! -x "$PROFDATA" || ! -x "$LLVMCOV" ]]; then
    print -u 2 -- "missing llvm-profdata / llvm-cov in $LLVM_BIN_DIR"
    print -u 2 -- "install the rustup component:   rustup component add llvm-tools-preview"
    exit 1
fi

FILTER="${1:-}"

rm -rf "$WORK"
mkdir -p "$PROF_DIR"

# Build all workspace tests with coverage instrumentation. Pinning
# both CARGO_TARGET_DIR and LLVM_PROFILE_FILE here means subsequent
# `cargo test --workspace` reruns inherit the same isolated tree and
# the same .profraw destination, so the post-test merge sees every
# instance.
export CARGO_TARGET_DIR="$BUILD_DIR"
export RUSTFLAGS="-Cinstrument-coverage"
export LLVM_PROFILE_FILE="$PROF_DIR/cov-%p-%m.profraw"

print -u 2 -- "[1/3] building instrumented test binaries..."
cargo test --workspace --no-run >/dev/null

print -u 2 -- "[2/3] running tests..."
cargo test --workspace --quiet >/dev/null 2>&1 || {
    print -u 2 -- "cargo test --workspace failed; report follows for partial runs."
}

PROFCOUNT=$(ls "$PROF_DIR"/*.profraw 2>/dev/null | wc -l | tr -d ' ')
if [[ "$PROFCOUNT" == "0" ]]; then
    print -u 2 -- "no .profraw files produced under $PROF_DIR"
    exit 1
fi
print -u 2 -- "      collected $PROFCOUNT raw profile(s)"

print -u 2 -- "[3/3] merging profiles and rendering report..."
"$PROFDATA" merge -sparse "$PROF_DIR"/*.profraw -o "$MERGED"

# Pass every executable test artifact to `llvm-cov` so the report
# aggregates across the workspace. Skip `*.d` depfiles and `.dSYM`
# debug-symbol bundles.
OBJECTS=()
for bin in "$BUILD_DIR"/debug/deps/*; do
    [[ -f "$bin" && -x "$bin" && "$bin" != *.d && "$bin" != *.dSYM* ]] || continue
    OBJECTS+=(-object "$bin")
done
if (( ${#OBJECTS[@]} == 0 )); then
    print -u 2 -- "no executables found in $BUILD_DIR/debug/deps"
    exit 1
fi

IGNORE='/.cargo/|/rustc/|/target/'
REPORT="$WORK/report.txt"
"$LLVMCOV" report \
    "${OBJECTS[@]}" \
    -instr-profile="$MERGED" \
    -ignore-filename-regex="$IGNORE" \
    -show-functions=0 \
    -use-color=false \
    > "$REPORT"

if [[ -n "$FILTER" ]]; then
    # Always include the header rows + the TOTAL line so the
    # filtered view is self-describing.
    awk -v f="$FILTER" 'NR<=2 || $0 ~ f || /^TOTAL/' "$REPORT"
else
    cat "$REPORT"
fi
