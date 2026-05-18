//! `uns` binary entry point.
//!
//! Step 2 stub. `clap` parses every flag defined in the TS commander
//! source (`ts/src/cli/args/build-command.ts`); execution writes
//! `Not implemented yet` plus the parsed args as pretty JSON to stderr
//! and exits 0. clap argument errors exit 2 as usual.

use clap::Parser;
use unsnarl::cli::args::Args;

fn main() {
    let args = Args::parse();
    let json = serde_json::to_string_pretty(&args).expect("serialize CLI args");
    eprintln!("Not implemented yet");
    eprintln!("{json}");
}
