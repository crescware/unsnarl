#!/usr/bin/env zsh
# Byte-for-byte parity + per-file timing for `uns -f ir` between the
# Rust implementation (`target/release/uns`) and the TypeScript
# implementation (`ts/dist/index.js`).
#
# Iterates every `.ts` / `.tsx` file under `ts/src`, runs both
# implementations with `-f ir`, and compares the IR (stdout) byte
# stream with `cmp`. Stderr from each implementation is captured
# separately so it never pollutes the comparison.
#
# Per-file wall-clock times in milliseconds are recorded for each
# implementation. Defaults to writing all artifacts under
# `target/parity-bench/` so they are git-ignored alongside the Cargo
# target directory.
#
# Usage:
#   ./bench-ir-parity.sh               # default work dir
#   ./bench-ir-parity.sh path/to/dir   # custom work dir
#
# Outputs (under the work dir):
#   summary.txt         human-readable totals
#   timings.tsv         file<TAB>rust_ms<TAB>ts_ms<TAB>match
#   fail_list.txt       paths whose stdouts differ
#   diff/<safe>.diff    unified diff (TS vs Rust) for each mismatch
#   rust/, ts/          raw IR stdouts (mismatches only; matches are deleted)
#   stderr/             non-empty stderr from either implementation

set -u
zmodload zsh/datetime
zmodload zsh/mathfunc

REPO_ROOT="${0:A:h}"
TS_ROOT="${REPO_ROOT}/ts"
RUST_BIN="${REPO_ROOT}/target/release/uns"
TS_BIN=(node "${TS_ROOT}/dist/index.js")

WORK="${1:-${REPO_ROOT}/target/parity-bench}"
rm -rf "$WORK"
mkdir -p "$WORK/rust" "$WORK/ts" "$WORK/diff" "$WORK/stderr"

if [[ ! -x "$RUST_BIN" ]]; then
    print -u 2 -- "missing $RUST_BIN — run \`mise run build\` first"
    exit 1
fi
if [[ ! -f "${TS_ROOT}/dist/index.js" ]]; then
    print -u 2 -- "missing ${TS_ROOT}/dist/index.js — run \`pnpm -C ts build\` first"
    exit 1
fi

cd "$TS_ROOT"

FILES=("${(@f)$(find src -type f \( -name '*.ts' -o -name '*.tsx' \) | sort)}")
TOTAL=${#FILES[@]}

PASS=0
FAIL=0
RUST_ERR=0
TS_ERR=0
RUST_TOTAL_MS=0
TS_TOTAL_MS=0
RUST_MAX_MS=0
TS_MAX_MS=0
RUST_MAX_FILE=""
TS_MAX_FILE=""

TIMINGS_TSV="$WORK/timings.tsv"
print -- "file\trust_ms\tts_ms\tmatch" > "$TIMINGS_TSV"

FAIL_LIST_FILE="$WORK/fail_list.txt"
: > "$FAIL_LIST_FILE"

bench_overall_start=$EPOCHREALTIME

i=0
for rel in $FILES; do
    (( i++ ))
    safe="${rel//\//__}"
    r_out="$WORK/rust/${safe}.out"
    t_out="$WORK/ts/${safe}.out"

    r_err="$WORK/stderr/${safe}.rust.err"
    t_err="$WORK/stderr/${safe}.ts.err"

    s=$EPOCHREALTIME
    "$RUST_BIN" -f ir "$rel" >"$r_out" 2>"$r_err"
    r_rc=$?
    e=$EPOCHREALTIME
    rust_ms=$(( int(((e - s) * 1000.0) + 0.5) ))

    s=$EPOCHREALTIME
    "${TS_BIN[@]}" -f ir "$rel" >"$t_out" 2>"$t_err"
    t_rc=$?
    e=$EPOCHREALTIME
    ts_ms=$(( int(((e - s) * 1000.0) + 0.5) ))

    [[ ! -s "$r_err" ]] && rm -f "$r_err"
    [[ ! -s "$t_err" ]] && rm -f "$t_err"

    (( r_rc != 0 )) && (( RUST_ERR++ ))
    (( t_rc != 0 )) && (( TS_ERR++ ))

    (( RUST_TOTAL_MS += rust_ms ))
    (( TS_TOTAL_MS += ts_ms ))

    if (( rust_ms > RUST_MAX_MS )); then
        RUST_MAX_MS=$rust_ms
        RUST_MAX_FILE=$rel
    fi
    if (( ts_ms > TS_MAX_MS )); then
        TS_MAX_MS=$ts_ms
        TS_MAX_FILE=$rel
    fi

    if cmp -s "$r_out" "$t_out"; then
        (( PASS++ ))
        match=1
        rm -f "$r_out" "$t_out"
    else
        (( FAIL++ ))
        match=0
        print -- "$rel" >> "$FAIL_LIST_FILE"
        diff -u "$t_out" "$r_out" >"$WORK/diff/${safe}.diff" 2>&1 || true
    fi

    print -- "$rel\t$rust_ms\t$ts_ms\t$match" >> "$TIMINGS_TSV"

    if (( i % 50 == 0 )); then
        print -u 2 -- "progress: $i/$TOTAL pass=$PASS fail=$FAIL rust_ms_total=$RUST_TOTAL_MS ts_ms_total=$TS_TOTAL_MS"
    fi
done

bench_overall_end=$EPOCHREALTIME
overall_ms=$(( int(((bench_overall_end - bench_overall_start) * 1000.0) + 0.5) ))

denom=$(( TOTAL > 0 ? TOTAL : 1 ))
RUST_AVG_MS=$(( RUST_TOTAL_MS / denom ))
TS_AVG_MS=$(( TS_TOTAL_MS / denom ))

{
    print -- "files_total=$TOTAL"
    print -- "files_pass=$PASS"
    print -- "files_fail=$FAIL"
    print -- "rust_nonzero_exit=$RUST_ERR"
    print -- "ts_nonzero_exit=$TS_ERR"
    print -- "rust_total_ms=$RUST_TOTAL_MS"
    print -- "ts_total_ms=$TS_TOTAL_MS"
    print -- "rust_avg_ms_per_file=$RUST_AVG_MS"
    print -- "ts_avg_ms_per_file=$TS_AVG_MS"
    print -- "rust_max_ms=$RUST_MAX_MS\t$RUST_MAX_FILE"
    print -- "ts_max_ms=$TS_MAX_MS\t$TS_MAX_FILE"
    print -- "wallclock_total_ms=$overall_ms"
} | tee "$WORK/summary.txt"
