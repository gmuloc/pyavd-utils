// Copyright (c) 2025-2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.
//! Criterion benchmarks.
#![allow(
    clippy::unwrap_used,
    missing_docs,
    reason = "criterion_group generates an undocumented entrypoint, and benchmarks fail fast with unwrap during setup"
)]

use avdschema::Load as _;
use avdschema::Store;
use criterion::Criterion;
use criterion::criterion_group;
use criterion::criterion_main;
use pyo3::types::PyAnyMethods as _;
use pyo3::types::PyDict;
use python_bindings::bindings;
use test_schema_store::get_store_gz_path;

const TEST_DATA: &str = r#"{"fabric_name":"foo","type":"l3ls-evpn"}"#;

fn benchmark_load_and_resolve_store(criterion: &mut Criterion) {
    let schema_file = get_store_gz_path();
    let mut group = criterion.benchmark_group("sample-size-10");
    group.sample_size(10);
    group.bench_function("load_and_resolve_store", |bencher| {
        bencher.iter(|| {
            let store = Store::from_file(Some(std::hint::black_box(schema_file))).unwrap();
            std::hint::black_box(store.as_resolved().unwrap());
        });
    });
    group.finish();
}

fn benchmark_get_validated_data(criterion: &mut Criterion) {
    test_schema_store::setup_python_with_store(|| pyo3::append_to_inittab!(bindings));
    criterion.bench_function("get_validated_data", |bencher| {
        pyo3::Python::attach(|py| {
            let module = py.import("_bindings").unwrap();
            bencher.iter(|| {
                let kwargs = PyDict::new(py);
                kwargs
                    .set_item("data_as_json", std::hint::black_box(TEST_DATA))
                    .unwrap();
                kwargs.set_item("schema_name", "avd_design").unwrap();
                std::hint::black_box(
                    module
                        .call_method("get_validated_data", (), Some(&kwargs))
                        .unwrap(),
                );
            });
        });
    });
}

criterion_group!(
    benches,
    benchmark_load_and_resolve_store,
    benchmark_get_validated_data
);
criterion_main!(benches);
