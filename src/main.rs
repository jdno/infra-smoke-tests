//! Smoke tests for the infrastructure of the Rust project
//!
//! This command-line application can be used to run smoke tests against the cloud infrastructure of
//! the Rust project. The tests confirm that the infrastructure is working as expected and that no
//! regressions have been introduced.

// Make it easier for future generations to maintain this code base by documenting it.
#![warn(clippy::missing_docs_in_private_items)]

use clap::Parser;

use crate::cli::Cli;
use crate::crates::Crates;
use crate::test::{TestSuite, TestSuiteResult};

mod cli;
mod environment;
mod test;

// Test suites
mod crates;

#[cfg(test)]
mod test_utils;

#[tokio::main]
async fn main() {
    let _cli = Cli::parse();

    let tests = vec![Crates::default()];

    let mut results: Vec<TestSuiteResult> = Vec::with_capacity(tests.len());
    for test in &tests {
        results.push(test.run().await);
    }

    for result in &results {
        println!("{:?}", result);
    }

    if results.iter().any(|result| !result.success()) {
        std::process::exit(1);
    }
}