//! It's not uncommon for programs like batched task runners or parsers to want to return an error
//! with multiple sources. The current version of the error trait does not support this usecase
//! very well, though there is work being done to improve this.
//!
//! For now however one way to work around this is to compose errors outside the error trait.
//! `color-eyre` supports such composition in its error reports via its custom sections feature.
//!
//! This example is intended to demonstrate two approaches to error aggregation with color-eyre

use color_eyre::{Help, Report};
use eyre::eyre;
use thiserror::Error;

fn main() -> Result<(), Report> {
    let errors = get_errors();
    join_errors(errors)
}

fn join_errors(results: Vec<Result<(), SourceError>>) -> Result<(), Report> {
    if results.iter().all(|r| r.is_ok()) {
        return Ok(());
    }

    results
        .into_iter()
        .filter(Result::is_err)
        .map(Result::unwrap_err)
        .fold(Err(eyre!("encountered multiple errors")), |report, e| {
            report.error(e)
        })
}

/// Helper function to generate errors
fn get_errors() -> Vec<Result<(), SourceError>> {
    vec![
        Err(SourceError {
            source: StrError("The task you ran encountered an error"),
            msg: "The task could not be complete",
        }),
        Err(SourceError {
            source: StrError("The machine you're connecting too is actively on fire"),
            msg: "the machine is unreachable",
        }),
        Err(SourceError {
            source: StrError("The file you're parsing is literally written in c++ instead of rust, what the hell"),
            msg: "The file could not be parsed",
        }),
    ]
}

/// Arbitrary error type for demonstration purposes
#[derive(Debug, Error)]
#[error("{0}")]
struct StrError(&'static str);

/// Arbitrary error type for demonstration purposes with a source error
#[derive(Debug, Error)]
#[error("{msg}")]
struct SourceError {
    msg: &'static str,
    source: StrError,
}
