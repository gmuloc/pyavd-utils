// Copyright (c) 2026 Arista Networks, Inc.
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

#[pymodule]
#[pyo3(name = "_bindings")]
pub mod bindings {
    use super::*;

    #[pymodule_export]
    use super::passwords_module as passwords;
    #[pymodule_export]
    use super::schema_store_module as schema_store;
    #[pymodule_export]
    use super::validation_module as validation;

    #[pymodule_init]
    fn init(_module: &Bound<'_, PyModule>) -> PyResult<()> {
        pyo3_log::init();
        debug!("initialized pyavd_utils._bindings");
        Ok(())
    }
}

#[pymodule(module = "_bindings")]
#[pyo3(name = "schema_store")]
mod schema_store_module {
    #[pymodule_export]
    use super::schema_store::init_store_from_file;
}

#[pymodule(module = "_bindings")]
#[pyo3(name = "validation")]
mod validation_module {
    #[pymodule_export]
    use super::validation::{
        Configuration, Deprecation, IgnoredEosConfigKey, ValidatedDataResult, ValidationResult,
        Violation, get_validated_data, validate_json, validate_json_with_adhoc_schema,
    };
}

#[pymodule(module = "_bindings")]
#[pyo3(name = "passwords")]
mod passwords_module {
    #[cfg(feature = "sha512")]
    #[pymodule_export]
    use super::passwords::sha512_crypt;
    #[cfg(feature = "cbc")]
    #[pymodule_export]
    use super::passwords::{cbc_decrypt, cbc_encrypt, cbc_verify};
    #[cfg(feature = "simple-7")]
    #[pymodule_export]
    use super::passwords::{simple_7_decrypt, simple_7_encrypt};
}

#[cfg(test)]
mod tests;
