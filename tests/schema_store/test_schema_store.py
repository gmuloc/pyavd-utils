# Copyright (c) 2026 Arista Networks, Inc.
# Use of this source code is governed by the Apache License 2.0
# that can be found in the LICENSE file.
from __future__ import annotations

from typing import TYPE_CHECKING

import pytest

from pyavd_utils.schema_store import init_store_from_file

if TYPE_CHECKING:
    from pathlib import Path


@pytest.mark.usefixtures("init_store")
def test_schema_store_init_store_from_file_twice_errors(tmp_path: Path) -> None:
    schema_file = tmp_path / "schemas.json"
    schema_file.write_text("{}", encoding="UTF-8")

    with pytest.raises(RuntimeError, match="Initialization can only happen once"):
        init_store_from_file(schema_file)
