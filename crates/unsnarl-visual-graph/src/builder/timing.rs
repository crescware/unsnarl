//! Per-function cumulative timing for `build_visual_graph`.
//!
//! `tracing` spans report inclusive walltime per call. For functions
//! that are called from inside `build_scope`'s recursion or
//! `emit_reference_edges`'s 200k-iteration loop, the per-call span is
//! either too noisy (one stderr line per invocation) or too aggregated
//! (one number for the entire recursive subtree). This module collects
//! cumulative `total_duration` / `call_count` for named scopes via a
//! thread-local accumulator; [`drain_and_emit`] then writes each entry
//! out as one `tracing::info!` event at the end of the build, so
//! `--verbose` reports the per-function totals without one line per
//! call.
//!
//! Usage:
//!
//! ```ignore
//! fn read_origins(...) -> ... {
//!     let _t = TimingScope::start("read_origins");
//!     // ... body ...
//! }
//! ```
//!
//! The scope's `Drop` adds the elapsed time to the accumulator. Call
//! [`drain_and_emit`] once at the end of `build_visual_graph` to flush.

use std::cell::RefCell;
use std::time::{Duration, Instant};

thread_local! {
    static TIMERS: RefCell<Vec<TimingEntry>> = const { RefCell::new(Vec::new()) };
}

struct TimingEntry {
    name: &'static str,
    total: Duration,
    calls: u64,
}

pub struct TimingScope {
    name: &'static str,
    start: Instant,
}

impl TimingScope {
    pub fn start(name: &'static str) -> Self {
        Self {
            name,
            start: Instant::now(),
        }
    }
}

impl Drop for TimingScope {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        TIMERS.with(|t| {
            let mut entries = t.borrow_mut();
            if let Some(entry) = entries.iter_mut().find(|e| e.name == self.name) {
                entry.total += elapsed;
                entry.calls += 1;
            } else {
                entries.push(TimingEntry {
                    name: self.name,
                    total: elapsed,
                    calls: 1,
                });
            }
        });
    }
}

/// Flush every accumulated entry as a single `tracing::info!` event and
/// reset the accumulator. Intended to be called once at the end of
/// `build_visual_graph`.
pub fn drain_and_emit() {
    TIMERS.with(|t| {
        let entries = std::mem::take(&mut *t.borrow_mut());
        for entry in entries {
            tracing::info!(
                target: "build_visual_graph::timing",
                name = entry.name,
                total_ms = entry.total.as_secs_f64() * 1000.0,
                calls = entry.calls,
                "function timing",
            );
        }
    });
}
