// Copyright (c) 2025-2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.
#![deny(unused_crate_dependencies)]

use std::io::Write as _;
use std::path::PathBuf;
use std::sync::OnceLock;

use pyo3::types::PyAnyMethods as _;
use pyo3::types::PyDict;

const CRATE_DIR: &str = env!("CARGO_MANIFEST_DIR");
const ADV_SCHEMA_URL: &str =
    "https://github.com/aristanetworks/avd/releases/download/v6.1.0/schemas.json.gz";

static INIT_PY: OnceLock<()> = OnceLock::new();
static INIT_STORE: OnceLock<()> = OnceLock::new();
static STORE_GZ_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn initialize() -> PathBuf {
    let resp = reqwest::blocking::get(ADV_SCHEMA_URL).unwrap();
    let body = resp.bytes().unwrap();
    let path = _get_store_gz_path();
    let file = std::fs::File::create(&path).unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(&body).unwrap();
    path
}

fn _get_store_gz_path() -> PathBuf {
    let url = reqwest::Url::parse(ADV_SCHEMA_URL).unwrap();
    let url_as_path = PathBuf::from(url.path());
    let filename = url_as_path.file_name().unwrap();
    PathBuf::from(CRATE_DIR).join("tmp").join(filename)
}

pub fn get_store_gz_path() -> &'static PathBuf {
    STORE_GZ_PATH.get_or_init(initialize)
}

pub fn setup_python(append_bindings_to_inittab: impl FnOnce()) {
    INIT_PY.get_or_init(|| {
        append_bindings_to_inittab();
        pyo3::Python::initialize();
    });
}

pub fn setup_python_with_store(append_bindings_to_inittab: impl FnOnce()) {
    setup_python(append_bindings_to_inittab);
    INIT_STORE.get_or_init(|| {
        pyo3::Python::attach(|py| {
            let module = py.import("_bindings").unwrap();
            let kwargs = PyDict::new(py);
            let file = py.detach(get_store_gz_path);
            kwargs.set_item("file", file).unwrap();
            module
                .call_method("init_store_from_file", (), Some(&kwargs))
                .unwrap();
        });
    });
}
