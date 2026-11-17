#!/usr/bin/env bash
# Fetch the prebuilt V8 static library and Rust binding that Cargo builds of this
# workspace need, verify them, and print the two variables that point the v8 crate at
# them:
#
#   eval "$(scripts/fetch-rusty-v8.sh)"
#   cargo build -p codex-code-mode-host
#
# Usage: fetch-rusty-v8.sh [<TARGET>] [<DIR>]
#   <TARGET>  Rust target triple; the host's by default.
#   <DIR>     Where the files are kept; ${XDG_CACHE_HOME:-~/.cache}/codex-rusty-v8 by
#             default. Files already there and intact are not downloaded again.
#
# The v8 crate only finds a prebuilt library for its default features. This workspace
# enables pointer compression and the sandbox, for which upstream publishes its own
# builds; without them the crate falls back to compiling V8 from source.
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
target="${1:-$(rustc -vV | sed -n 's/^host: //p')}"
dir="${2:-${XDG_CACHE_HOME:-$HOME/.cache}/codex-rusty-v8}"

version="$(awk '$0 == "name = \"v8\"" { getline; gsub(/version = |"/, ""); print; exit }' \
  "$repo_root/codex-rs/Cargo.lock")"
if [[ -z "$version" ]]; then
  echo "fetch-rusty-v8: no v8 package in codex-rs/Cargo.lock" >&2
  exit 1
fi

profile="ptrcomp_sandbox_release"
base_url="https://github.com/openai/codex/releases/download/rusty-v8-v${version}"
archive="librusty_v8_${profile}_${target}.a.gz"
binding="src_binding_${profile}_${target}.rs"
checksums="rusty_v8_${profile}_${target}.sha256"
trusted="$repo_root/third_party/v8/rusty_v8_${version//./_}_release_manifests.sha256"

if [[ ! -f "$trusted" ]]; then
  echo "fetch-rusty-v8: no trusted checksums for v8 ${version} (${trusted})" >&2
  exit 1
fi
expected="$(awk -v name="$checksums" '$2 == name { print $1 }' "$trusted")"
if [[ -z "$expected" ]]; then
  echo "fetch-rusty-v8: no prebuilt V8 ${version} is listed for ${target}" >&2
  exit 1
fi

if command -v sha256sum >/dev/null 2>&1; then
  sha256() { sha256sum "$@"; }
else
  sha256() { shasum -a 256 "$@"; }
fi

mkdir -p "$dir/$version"
cd "$dir/$version"

verify() {
  [[ -f "$checksums" && -f "$archive" && -f "$binding" ]] || return 1
  [[ "$(sha256 "$checksums" | cut -d ' ' -f 1)" == "$expected" ]] || return 1
  # Manifests built on Windows have CRLF line endings.
  tr -d '\r' < "$checksums" | sha256 --check - >/dev/null 2>&1
}

if ! verify; then
  for name in "$checksums" "$archive" "$binding"; do
    curl -fsSL --retry 3 "$base_url/$name" -o "$name"
  done
  if ! verify; then
    echo "fetch-rusty-v8: checksum mismatch for the files in $PWD" >&2
    exit 1
  fi
fi

printf 'export RUSTY_V8_ARCHIVE=%q\n' "$PWD/$archive"
printf 'export RUSTY_V8_SRC_BINDING_PATH=%q\n' "$PWD/$binding"
