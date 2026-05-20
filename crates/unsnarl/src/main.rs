//! `uns` binary entry point.

use std::process::ExitCode;

use unsnarl::cli::args::Args;
use unsnarl::run::run;

fn main() -> ExitCode {
    let args = Args::parse();
    run(&args)
}
