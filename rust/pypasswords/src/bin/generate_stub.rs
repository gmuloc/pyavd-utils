// Copyright (c) 2025-2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.

#[cfg(pyavd_stubgen)]
fn main() -> pyo3_stub_gen::Result<()> {
    pypasswords::stub_info()?.generate()
}

#[cfg(not(pyavd_stubgen))]
fn main() {
    panic!("generate_stub must be run with RUSTFLAGS=\"--cfg pyavd_stubgen\"");
}
