use std::{fs::File, path::PathBuf};

use anyhow::{Context as _, Result};
use serde::{Deserialize, Serialize};
use tracing::{info, instrument};

use super::CountingWriter;
use crate::utils::human;

/// Write a human readable JSON file (slow and large).
#[instrument(skip(value))]
pub fn write_json<T: Serialize>(value: &T, path: &PathBuf) -> Result<()> {
    // Open file
    let mut file = File::create(path).context("while creating output file")?;
    let mut file_counter = CountingWriter::new(&mut file);

    // Write pretty JSON (for smaller files, use the bin format)
    serde_json::to_writer_pretty(&mut file_counter, value).context("while writing JSON")?;

    let size = file_counter.count();
    file.sync_all().context("while syncing output file")?;
    drop(file);

    info!(
        ?path,
        size,
        "Wrote {}B bytes to {path:?}",
        human(size as f64)
    );
    Ok(())
}

/// Read a JSON file.
pub fn read_json<T: for<'a> Deserialize<'a>>(path: &PathBuf) -> Result<T> {
    let mut file = File::open(path).context("while opening input file")?;
    serde_json::from_reader(&mut file).context("while reading JSON")
}
