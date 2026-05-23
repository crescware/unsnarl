//! `uns` binary entry point.

use std::process::ExitCode;
use std::time::Instant;

use unsnarl::cli::args::Args;
use unsnarl::run::{init_verbose_tracing, run};

fn main() -> ExitCode {
    // Stamp the entry point before any work so the `--verbose` log
    // can report (a) the time `Args::parse` (clap) consumed and
    // (b) the time spent inside `run`, both measured from the
    // start of `main`. Pre-`main` cost (dyld, Rust runtime init)
    // is unobservable from inside the process; that is recovered
    // by comparing the externally-measured wall clock to the
    // `main entry` timestamp printed here.
    let t_main = Instant::now();

    // `Args::parse` itself is what we want to time, so we cannot
    // wait for it to tell us whether `--verbose` was passed. Scan
    // the raw argv ahead of clap; the cost (one linear sweep of
    // the few CLI tokens) is negligible.
    let verbose = std::env::args_os().any(|a| a == "--verbose");
    if verbose {
        init_verbose_tracing();
    }
    tracing::info!(elapsed_us = 0u64, "main entry");

    let args = Args::parse();
    tracing::info!(
        elapsed_us = t_main.elapsed().as_micros() as u64,
        "args parsed"
    );

    let code = run(&args);
    tracing::info!(
        elapsed_us = t_main.elapsed().as_micros() as u64,
        "run returned"
    );

    code
}
