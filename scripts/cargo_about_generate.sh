#!/usr/bin/env bash

set -euo pipefail

readonly cargo_about_version="0.8.4"

export PATH="${HOME}/.cargo/bin:${PATH}"
export BINSTALL_DISABLE_TELEMETRY=true

if ! command -v cargo-about > /dev/null; then
  if ! command -v cargo-binstall > /dev/null; then
    curl -L --proto '=https' --tlsv1.2 -sSf \
      https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
  fi

  cargo binstall "cargo-about@${cargo_about_version}" --no-confirm
fi

cargo about generate --locked --fail -o pyavd_utils/THIRD_PARTY_LICENSES.txt about-text.hbs
