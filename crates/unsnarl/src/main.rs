//! `uns` binary entry point.

use clap::Parser;
use unsnarl::cli::args::Args;

fn main() {
    let args = Args::parse();
    let json = serde_json::to_string_pretty(&args).expect("serialize CLI args");
    eprintln!("Not implemented yet");
    eprintln!("{json}");
}
