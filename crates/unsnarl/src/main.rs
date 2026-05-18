//! `uns` binary entry point.

use clap::Parser;
use unsnarl::cli::args::Args;
use unsnarl::run::run;

fn main() {
    let args = Args::parse();
    run(&args);
}
