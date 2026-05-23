//! Verbose-gated profiling probes.
//!
//! `unsnarl` is structured as many small crates, each of which used to
//! scatter its own `Instant::now`, `AtomicU64` accumulators, `record()`
//! helpers, `*Stats` drain types, and `tracing::info_span!` wrappers
//! across files. Those probes were paid on every `uns` invocation,
//! even when `--verbose` was absent, because nothing gated them.
//!
//! This crate centralises the probes behind a single process-global
//! `VERBOSE` flag. The flag is set by `unsnarl::run::init_verbose_tracing`
//! the moment `--verbose` is detected (which `main.rs` does ahead of
//! clap so the gate is live before any work). Every primitive in this
//! crate short-circuits on `!verbose()`, so a release build without
//! `--verbose` pays only one relaxed atomic load per probe.
//!
//! The primitives:
//!
//! - [`verbose`] / [`set_verbose`]: the gate.
//! - [`timing_start`]: returns `Some(Instant::now())` only when verbose,
//!   so the `mach_absolute_time` syscall is skipped otherwise.
//! - [`record_elapsed_ns`]: adds elapsed nanoseconds to an atomic
//!   counter, no-op when the matching `timing_start` returned `None`.
//! - [`count_if_verbose`]: gated counter increment for "size" totals
//!   that the per-call sites accumulate alongside the timers.
//! - [`add_elapsed`]: gated accumulation into a local `Duration`, used
//!   by the few in-place loops that keep their accumulators on the
//!   stack instead of in static atomics.
//! - [`TimingScope`]: thread-local cumulative timer keyed by a static
//!   name. Used by hot recursive code paths in `unsnarl-visual-graph`
//!   that would otherwise emit one tracing span per call.
//! - [`drain_and_emit`]: flushes the `TimingScope` accumulator as one
//!   `tracing::info!` event per recorded name.
//! - The [`span!`](crate::span) macro: a drop-in for
//!   `tracing::info_span!("name", ...).entered()` that returns `None`
//!   when verbose is off, so the span object is never constructed.

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, Instant};

static VERBOSE: AtomicBool = AtomicBool::new(false);

/// Set the process-global verbose gate. Called by
/// `unsnarl::run::init_verbose_tracing` the moment the binary detects
/// `--verbose`; nothing else should flip the gate.
pub fn set_verbose(on: bool) {
    VERBOSE.store(on, Ordering::Relaxed);
}

#[inline]
pub fn verbose() -> bool {
    VERBOSE.load(Ordering::Relaxed)
}

/// `Some(Instant::now())` when verbose, `None` otherwise. The matching
/// [`record_elapsed_ns`] / [`add_elapsed`] no-op on `None`, so the
/// `mach_absolute_time` syscall is paid only under `--verbose`.
#[inline]
pub fn timing_start() -> Option<Instant> {
    if verbose() {
        Some(Instant::now())
    } else {
        None
    }
}

/// Add the nanoseconds elapsed since `t` to `counter`. No-op when `t`
/// is `None` (i.e. when [`timing_start`] short-circuited).
#[inline]
pub fn record_elapsed_ns(counter: &AtomicU64, t: Option<Instant>) {
    if let Some(t) = t {
        counter.fetch_add(t.elapsed().as_nanos() as u64, Ordering::Relaxed);
    }
}

/// Increment `counter` by `n`, but only when verbose is on. Used for
/// "how many items did this phase touch" totals that sit next to the
/// nanosecond timers and have no value outside `--verbose` output.
#[inline]
pub fn count_if_verbose(counter: &AtomicU64, n: u64) {
    if verbose() {
        counter.fetch_add(n, Ordering::Relaxed);
    }
}

/// Add the elapsed `Duration` since `t` to `d`. No-op on `None`. The
/// inline counterpart of [`record_elapsed_ns`] for the few sites that
/// accumulate into a stack-local `Duration` rather than a static
/// `AtomicU64`.
#[inline]
pub fn add_elapsed(d: &mut Duration, t: Option<Instant>) {
    if let Some(t) = t {
        *d += t.elapsed();
    }
}

/// `tracing::info_span!("name", ...).entered()` gated by [`verbose`].
/// Expands to `Some(EnteredSpan)` when verbose is on and `None`
/// otherwise; both branches share the same `Option<EnteredSpan>` type
/// so `let _span = span!("...")` keeps the existing drop semantics
/// (span exits when `_span` goes out of scope) regardless of the
/// branch taken.
#[macro_export]
macro_rules! span {
    ($name:literal $(, $($rest:tt)*)?) => {{
        if $crate::verbose() {
            Some(::tracing::info_span!($name $(, $($rest)*)?).entered())
        } else {
            None
        }
    }};
}

thread_local! {
    static TIMERS: RefCell<Vec<TimingEntry>> = const { RefCell::new(Vec::new()) };
}

struct TimingEntry {
    name: &'static str,
    total: Duration,
    calls: u64,
}

/// Per-function cumulative timer keyed by a static name. Drop adds the
/// elapsed time to the thread-local accumulator; [`drain_and_emit`]
/// flushes the accumulator as one tracing event per name. Used by
/// `unsnarl-visual-graph` for functions called from inside recursive
/// `build_scope` or the 200k-iteration `emit_reference_edges` loop,
/// where one tracing span per call would either be too noisy or too
/// aggregated.
pub struct TimingScope {
    name: &'static str,
    start: Instant,
}

impl TimingScope {
    /// `Some(scope)` when verbose, `None` otherwise. The `None` case
    /// also keeps the thread-local accumulator empty, so
    /// [`drain_and_emit`] has nothing to flush.
    pub fn start(name: &'static str) -> Option<Self> {
        if verbose() {
            Some(Self {
                name,
                start: Instant::now(),
            })
        } else {
            None
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

/// Flush every [`TimingScope`] entry as a single `tracing::info!`
/// event under the caller-supplied tracing target and reset the
/// accumulator. No-op when verbose is off (the accumulator is empty in
/// that case because [`TimingScope::start`] returned `None`).
///
/// The `tracing::info!` macro embeds `target:` into a static callsite,
/// so the target must be a string literal at the macro call site —
/// hence the macro shape rather than a function that takes
/// `&'static str`. The actual draining loop lives in
/// [`__drain_with`] so each call site only re-expands a single
/// `tracing::info!`.
#[macro_export]
macro_rules! drain_and_emit {
    ($target:literal) => {{
        if $crate::verbose() {
            $crate::__drain_with(|name, total_ms, calls| {
                ::tracing::info!(
                    target: $target,
                    name = name,
                    total_ms = total_ms,
                    calls = calls,
                    "function timing",
                );
            });
        }
    }};
}

/// Implementation detail of [`drain_and_emit!`]. Drains the
/// thread-local [`TimingScope`] accumulator and invokes `emit` once per
/// entry. Not intended to be called directly; the macro wires in the
/// correct static tracing target.
#[doc(hidden)]
pub fn __drain_with(emit: impl Fn(&'static str, f64, u64)) {
    TIMERS.with(|t| {
        let entries = std::mem::take(&mut *t.borrow_mut());
        for entry in entries {
            emit(entry.name, entry.total.as_secs_f64() * 1000.0, entry.calls);
        }
    });
}
