// Copyright (c) 2025-2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

use pyo3::types::PyAnyMethods as _;

use crate::bindings;

mod passwords;
mod validation;

fn setup_python() {
    test_schema_store::setup_python(|| pyo3::append_to_inittab!(bindings));
}

fn setup() {
    test_schema_store::setup_python_with_store(|| pyo3::append_to_inittab!(bindings));
}

fn get_path_and_message_from_py_violation(
    violation: pyo3::Bound<'_, pyo3::PyAny>,
) -> (Vec<String>, String) {
    let path: Vec<String> = violation
        .getattr("path")
        .unwrap()
        .cast_into_exact::<pyo3::types::PyList>()
        .unwrap()
        .into_iter()
        .map(|item| {
            item.cast_into_exact::<pyo3::types::PyString>()
                .unwrap()
                .to_string()
        })
        .collect();
    let message = violation
        .getattr("message")
        .unwrap()
        .cast_into_exact::<pyo3::types::PyString>()
        .unwrap()
        .to_string();
    (path, message)
}
