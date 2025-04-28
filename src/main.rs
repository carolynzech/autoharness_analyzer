use anyhow::Result;
use make_tables::compute_metrics;
use parse_scanner_output::process_scan_fns;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, HashMap},
    fs,
    path::Path,
};
use strum_macros::{Display, EnumString};

mod make_tables;
mod parse_scanner_output;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoHarnessMetadata {
    /// Functions we generated automatic harnesses for.
    pub chosen: Vec<String>,
    /// Map function names to the reason why we did not generate an automatic harness for that function.
    /// We use an ordered map so that when we print the data, it is ordered alphabetically by function name.
    pub skipped: BTreeMap<String, AutoHarnessSkipReason>,
}

/// Reasons that Kani does not generate an automatic harness for a function.
#[derive(Debug, Clone, Serialize, Deserialize, Display, EnumString)]
pub enum AutoHarnessSkipReason {
    /// The function is generic.
    #[strum(serialize = "Generic Function")]
    GenericFn,
    /// A Kani-internal function: already a harness, implementation of a Kani associated item or Kani contract instrumentation functions).
    #[strum(serialize = "Kani implementation")]
    KaniImpl,
    /// At least one of the function's arguments does not implement kani::Arbitrary
    /// (The Vec<(String, String)> contains the list of (name, type) tuples for each argument that does not implement it
    #[strum(serialize = "Missing Arbitrary implementation for argument(s)")]
    MissingArbitraryImpl(Vec<(String, String)>),
    /// The function does not have a body.
    #[strum(serialize = "The function does not have a body")]
    NoBody,
    /// The function doesn't match the user's provided filters.
    #[strum(serialize = "Did not match provided filters")]
    UserFilter,
}

impl Default for AutoHarnessMetadata {
    fn default() -> Self {
        Self::new()
    }
}

impl AutoHarnessMetadata {
    pub fn new() -> Self {
        Self {
            chosen: Vec::new(),
            skipped: BTreeMap::new(),
        }
    }

    pub fn extend(&mut self, other: AutoHarnessMetadata) {
        self.chosen.extend(other.chosen);
        self.skipped.extend(other.skipped);
    }
}

// TODO: add a command line flag --per-crate that outputs per-crate tables, not just the combined data
fn main() -> Result<()> {
    let metadata_dir_path = std::env::args()
        .nth(1)
        .expect("No path to directory with kani_metadata.json files provided");
    let scanner_results_path = std::env::args()
        .nth(2)
        .expect("No scanner results directory path provided");

    // Collection of data from all crates
    let mut cross_crate_fn_to_row_data = HashMap::new();
    let mut cross_crate_autoharness_md = AutoHarnessMetadata::new();

    // Iterate over all kani-metadata.json files; one per crate
    for entry in fs::read_dir(&metadata_dir_path)? {
        let entry = entry?;
        let path = entry.path();
        if !path.to_string_lossy().contains("kani-metadata.json") {
            continue;
        }

        let kani_md_file_data =
            std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("Unable to read {:?}", path));
        let v: Value = serde_json::from_str(&kani_md_file_data)?;
        let crate_name = v["crate_name"].as_str().unwrap();

        println!("Processing crate {crate_name}");

        let scanner_fn_csv = format!("{scanner_results_path}/{crate_name}_scan_functions.csv");
        let scanner_fn_csv_path = Path::new(&scanner_fn_csv);
        let fn_to_row_data = process_scan_fns(scanner_fn_csv_path)?;

        let autoharness_md: AutoHarnessMetadata =
            serde_json::from_value(v["autoharness_md"].clone())?;

        cross_crate_fn_to_row_data.extend(fn_to_row_data);
        cross_crate_autoharness_md.extend(autoharness_md);
    }

    compute_metrics(&cross_crate_autoharness_md, &cross_crate_fn_to_row_data)?;

    Ok(())
}
