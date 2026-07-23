// Copyright (c) 2025-2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.
//! Python bindings for pyavd-utils.

#![allow(
    missing_docs,
    missing_debug_implementations,
    clippy::fn_params_excessive_bools,
    clippy::manual_let_else,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value,
    clippy::struct_excessive_bools,
    clippy::unnecessary_trailing_comma,
    clippy::unnecessary_wraps,
    reason = "PyO3-facing API names and test assertions mirror the exported Python module contract"
)]

use log::debug;
use pyo3::Bound;
use pyo3::PyResult;
use pyo3::pymodule;
use pyo3::types::PyModule;

mod passwords;
mod schema_store;
mod validation;

#[pymodule(gil_used = false)]
#[pyo3(name = "_bindings")]
pub fn bindings(module: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();
    debug!("initialized pyavd_utils._bindings");
    schema_store::register(module)?;
    validation::register(module)?;
    passwords::register(module)?;
    Ok(())
}

#[cfg(test)]
mod tests;
