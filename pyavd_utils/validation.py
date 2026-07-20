# Copyright (c) 2025-2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
from __future__ import annotations

import warnings
from typing import TYPE_CHECKING

from ._bindings import (
    Configuration,
    Deprecation,
    IgnoredEosConfigKey,
    ValidatedDataResult,
    ValidationResult,
    Violation,
    get_validated_data,
    validate_json,
    validate_json_with_adhoc_schema,
)
from .schema_store import init_store_from_file as _init_store_from_file

if TYPE_CHECKING:
    from pathlib import Path

__all__ = [
    "Configuration",
    "Deprecation",
    "IgnoredEosConfigKey",
    "ValidatedDataResult",
    "ValidationResult",
    "Violation",
    "get_validated_data",
    "init_store_from_file",
    "validate_json",
    "validate_json_with_adhoc_schema",
]


def init_store_from_file(file: Path) -> None:
    """Initialize the schema store. Deprecated; use pyavd_utils.schema_store."""
    warnings.warn(
        "pyavd_utils.validation.init_store_from_file() is deprecated. Use pyavd_utils.schema_store.init_store_from_file() instead.",
        DeprecationWarning,
        stacklevel=2,
    )
    _init_store_from_file(file)
