// Copyright (c) 2025-2026 Arista Networks, Inc.
// Use of this source code is governed by the Apache License 2.0
// that can be found in the LICENSE file.
#![deny(unused_crate_dependencies)]

use pyo3::pymodule;

#[pymodule]
#[pyo3(name = "passwords")]
mod passwords {

    use pyo3::{
        PyResult,
        exceptions::{PyRuntimeError, PyValueError},
        pyfunction,
    };

    #[cfg(feature = "sha512")]
    #[cfg_attr(
        pyavd_stubgen,
        pyo3_stub_gen_derive::gen_stub_pyfunction(module = "passwords")
    )]
    #[pyfunction]
    /// Computes the SHA512 crypt value for the password given the salt.
    ///
    /// The number of rounds is hardcoded to 5000 as expected by EOS.
    ///
    /// Args:
    ///   password: The password.
    ///   salt: The salt to use (truncated to 16 characters). Allowed characters are [a-zA-Z0-9/.].
    ///
    /// Returns:
    ///   The sha512 crypt value.
    ///
    /// Raises:
    ///   ValueError: If the salt is empty or contains invalid characters.
    ///   RuntimeError: If the underlying SHA crypt library returns an error.
    pub fn sha512_crypt(password: String, salt: String) -> PyResult<String> {
        passwords::sha512_crypt(&password, &salt).map_err(|err| {
            // Mapping our crates error to Python errors.
            match err {
                passwords::Sha512CryptError::InvalidSalt(_) => {
                    PyValueError::new_err(err.to_string())
                }
                passwords::Sha512CryptError::ShaCrypt(_) => {
                    PyRuntimeError::new_err(err.to_string())
                }
            }
        })
    }

    #[cfg(feature = "cbc")]
    #[cfg_attr(
        pyavd_stubgen,
        pyo3_stub_gen_derive::gen_stub_pyfunction(module = "passwords")
    )]
    #[pyfunction]
    /// Encrypt the data string using CBC TripleDES.
    ///
    /// Args:
    ///     password: The encryption key.
    ///     data: The data to be encrypted.
    ///
    /// Returns:
    ///     The encrypted data, encoded in base64.
    ///
    /// Raises:
    ///     RuntimeError: If encryption fails or base64 output contains invalid UTF-8.
    pub fn cbc_encrypt(password: String, data: String) -> PyResult<String> {
        let result_bytes = passwords::cbc_encrypt(password.as_bytes(), data.as_bytes())
            .map_err(|err| PyRuntimeError::new_err(err.to_string()))?;
        String::from_utf8(result_bytes)
            .map_err(|_| PyRuntimeError::new_err("Base64 output contained invalid UTF-8"))
    }

    #[cfg(feature = "cbc")]
    #[cfg_attr(
        pyavd_stubgen,
        pyo3_stub_gen_derive::gen_stub_pyfunction(module = "passwords")
    )]
    #[pyfunction]
    /// Decrypt the encrypted_data string using CBC TripleDES.
    ///
    /// Args:
    ///     password: The encryption key.
    ///     encrypted_data: The base64-encoded encrypted data to be decrypted.
    ///
    /// Returns:
    ///     The decrypted data.
    ///
    /// Raises:
    ///     ValueError: If encrypted_data is not valid base64 or decrypted data is not valid UTF-8.
    ///     RuntimeError: If decryption fails.
    pub fn cbc_decrypt(password: String, encrypted_data: String) -> PyResult<String> {
        let decrypted_bytes =
            passwords::cbc_decrypt(password.as_bytes(), encrypted_data.as_bytes()).map_err(
                |err| match err {
                    passwords::CbcError::InvalidBase64 => PyValueError::new_err(err.to_string()),
                    _ => PyRuntimeError::new_err(err.to_string()),
                },
            )?;

        String::from_utf8(decrypted_bytes)
            .map_err(|_| PyValueError::new_err(passwords::CbcError::InvalidUtf8.to_string()))
    }

    #[cfg(feature = "cbc")]
    #[cfg_attr(
        pyavd_stubgen,
        pyo3_stub_gen_derive::gen_stub_pyfunction(module = "passwords")
    )]
    #[pyfunction]
    /// Verify if an encrypted password is decryptable with the given key.
    ///
    /// It does not return the password but only raises an error if the password cannot be decrypted.
    ///
    /// Args:
    ///     password: The decryption key.
    ///     encrypted_data: The base64-encoded encrypted data to be decrypted.
    ///
    /// Returns:
    ///     `True` if the password is decryptable, `False` otherwise.
    ///
    /// Raises:
    ///     This function does not raise validation errors; invalid encrypted data returns `False`.
    pub fn cbc_verify(password: String, encrypted_data: String) -> bool {
        passwords::cbc_check_password(password.as_bytes(), encrypted_data.as_bytes())
    }

    #[cfg(feature = "simple-7")]
    #[cfg_attr(
        pyavd_stubgen,
        pyo3_stub_gen_derive::gen_stub_pyfunction(module = "passwords")
    )]
    #[pyfunction]
    /// Encrypt (obfuscate) a password with insecure type-7.
    ///
    /// WARNING: Type-7 encryption is NOT secure and should only be used for compatibility
    /// with legacy systems. It provides only obfuscation, not real encryption.
    ///
    /// If salt is None, a random salt in the range 0-15 will be used.
    /// Raises ValueError if the password is empty or the salt is out of range.
    ///
    /// Args:
    ///     data: The password to encrypt.
    ///     salt: The salt value (0-15). If None, a random salt will be generated.
    ///
    /// Returns:
    ///     The encrypted password in type-7 format.
    ///
    /// Raises:
    ///     ValueError: If the password is empty or the salt is out of range.
    ///     RuntimeError: If the random salt source is unavailable or encrypted data is not valid UTF-8.
    pub fn simple_7_encrypt(data: String, salt: Option<u8>) -> PyResult<String> {
        passwords::simple_7_encrypt(&data, salt).map_err(|err| match err {
            passwords::Simple7Error::InvalidSaltValue(_)
            | passwords::Simple7Error::EmptyPassword => PyValueError::new_err(err.to_string()),
            _ => PyRuntimeError::new_err(err.to_string()),
        })
    }

    #[cfg(feature = "simple-7")]
    #[cfg_attr(
        pyavd_stubgen,
        pyo3_stub_gen_derive::gen_stub_pyfunction(module = "passwords")
    )]
    #[pyfunction]
    /// Decrypt (deobfuscate) a password from insecure type-7.
    ///
    /// Raises ValueError if the password is empty or decryption fails.
    ///
    /// WARNING: Type-7 encryption is NOT secure and should only be used for compatibility
    /// with legacy systems. It provides only obfuscation, not real encryption.
    ///
    /// Args:
    ///     data: The type-7 encrypted password to decrypt.
    ///
    /// Returns:
    ///     The decrypted password.
    ///
    /// Raises:
    ///     ValueError: If the password is empty or decryption fails.
    ///     RuntimeError: If decrypted data is not valid UTF-8.
    pub fn simple_7_decrypt(data: String) -> PyResult<String> {
        passwords::simple_7_decrypt(&data).map_err(|err| match err {
            passwords::Simple7Error::InvalidUtf8(_) => PyRuntimeError::new_err(err.to_string()),
            _ => PyValueError::new_err(err.to_string()),
        })
    }
}

/// Gather stub generation metadata for the Python package layout.
#[cfg(pyavd_stubgen)]
pub fn stub_info() -> pyo3_stub_gen::Result<pyo3_stub_gen::StubInfo> {
    pyo3_stub_gen::StubInfo::from_project_root(
        "passwords".to_string(),
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../pyavd_utils"),
        false,
        pyo3_stub_gen::StubGenConfig::default(),
    )
}

// Implementation of the pytests but here using pyo3 wrappers in Rust, to ensure we get coverage data
// and that we can catch issues in Rust without building the Python first.
#[cfg(test)]
mod tests;
