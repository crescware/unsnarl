//! Land the emitted text on the chosen destination.
//!
//! Mirrors `ts/src/cli/run-cli/write-output.ts`. When `output_path`
//! is `None`, the text goes to the supplied stdout writer (the CLI
//! binary passes `io::stdout().lock()`). When a path is given, the
//! parent directory is created recursively and the file is written
//! in full (overwriting any existing content).

use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn write_output(
    output_path: Option<&Path>,
    text: &str,
    stdout: &mut dyn Write,
) -> io::Result<()> {
    match output_path {
        Some(path) => {
            if let Some(parent) = path.parent() {
                if !parent.as_os_str().is_empty() {
                    fs::create_dir_all(parent)?;
                }
            }
            fs::write(path, text)?;
            Ok(())
        }
        None => stdout.write_all(text.as_bytes()),
    }
}

#[cfg(test)]
#[path = "write_output_test.rs"]
mod write_output_test;
