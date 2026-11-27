#!/bin/sh
# Installer for grandjury release binaries on Linux and macOS.
# Downloads the matching GitHub Release archive, verifies SHA256SUMS,
# and installs grandjury into $HOME/.local/bin or --to DIR.

set -eu

REPO="david-engelmann/grandjury"
BIN="grandjury"
INSTALL_DIR="${GRANDJURY_INSTALL_DIR:-$HOME/.local/bin}"
VERSION="${GRANDJURY_VERSION:-latest}"
TARGET_OVERRIDE=""

usage() {
  cat <<'EOF'
Install a prebuilt release binary from GitHub Releases.

Options
  --version <TAG>    Release tag such as v0.1.0 (default latest)
  --to <DIR>         Destination directory (default $HOME/.local/bin)
  --target <TRIPLE>  Override target triple detection
  -h, --help         Print this help text
EOF
}

require_arg() {
  if [ "$#" -lt 2 ] || [ -z "$2" ]; then
    echo "error option $1 requires a value" >&2
    usage >&2
    exit 2
  fi
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --version)
      require_arg "$@"
      VERSION="$2"
      shift 2
      ;;
    --to)
      require_arg "$@"
      INSTALL_DIR="$2"
      shift 2
      ;;
    --target)
      require_arg "$@"
      TARGET_OVERRIDE="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "error unknown option $1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

detect_target() {
  if [ -n "$TARGET_OVERRIDE" ]; then
    printf '%s\n' "$TARGET_OVERRIDE"
    return
  fi
  os_name="$(uname -s)"
  arch_name="$(uname -m)"
  case "$os_name" in
    Linux)
      case "$arch_name" in
        x86_64|amd64) printf 'x86_64-unknown-linux-gnu\n' ;;
        aarch64|arm64) printf 'aarch64-unknown-linux-gnu\n' ;;
        *) echo "error unsupported Linux architecture $arch_name" >&2; exit 1 ;;
      esac
      ;;
    Darwin)
      case "$arch_name" in
        x86_64) printf 'x86_64-apple-darwin\n' ;;
        arm64|aarch64) printf 'aarch64-apple-darwin\n' ;;
        *) echo "error unsupported macOS architecture $arch_name" >&2; exit 1 ;;
      esac
      ;;
    *)
      echo "error unsupported operating system $os_name" >&2
      exit 1
      ;;
  esac
}

fetch() {
  url="$1"
  dest="$2"
  if command -v curl >/dev/null 2>&1; then
    curl --proto '=https' --tlsv1.2 -fsSL "$url" -o "$dest"
  elif command -v wget >/dev/null 2>&1; then
    wget -qO "$dest" "$url"
  else
    echo "error curl or wget is required" >&2
    exit 1
  fi
}

resolve_tag() {
  if [ "$VERSION" != "latest" ]; then
    case "$VERSION" in
      v*) printf '%s\n' "$VERSION" ;;
      *) printf 'v%s\n' "$VERSION" ;;
    esac
    return
  fi
  tmp_json="$WORK_DIR/latest.json"
  fetch "https://api.github.com/repos/$REPO/releases/latest" "$tmp_json"
  tag="$(sed -n 's/.*"tag_name"[[:space:]]*:[[:space:]]*"\([^"]*\)".*/\1/p' "$tmp_json" | head -n 1)"
  if [ -z "$tag" ]; then
    echo "error could not resolve latest release tag for $REPO" >&2
    exit 1
  fi
  printf '%s\n' "$tag"
}

verify_sha256() {
  archive_file="$1"
  sums_file="$2"
  archive_base="$(basename "$archive_file")"
  expected="$(awk -v name="$archive_base" '$2 == name || $2 == "*" name { print $1; exit }' "$sums_file")"
  if [ -z "$expected" ]; then
    echo "error $archive_base not listed in SHA256SUMS" >&2
    exit 1
  fi
  if command -v sha256sum >/dev/null 2>&1; then
    actual="$(sha256sum "$archive_file" | awk '{ print $1 }')"
  elif command -v shasum >/dev/null 2>&1; then
    actual="$(shasum -a 256 "$archive_file" | awk '{ print $1 }')"
  else
    echo "error sha256sum or shasum is required to verify release archives" >&2
    exit 1
  fi
  if [ "$expected" != "$actual" ]; then
    echo "error SHA-256 mismatch for $archive_base" >&2
    exit 1
  fi
}

WORK_DIR="$(mktemp -d)"
trap 'rm -rf "$WORK_DIR"' EXIT INT TERM

TARGET="$(detect_target)"
TAG="$(resolve_tag)"
STEM="${BIN}-${TAG}-${TARGET}"
ARCHIVE="${STEM}.tar.gz"
BASE_URL="https://github.com/${REPO}/releases/download/${TAG}"

fetch "${BASE_URL}/${ARCHIVE}" "${WORK_DIR}/${ARCHIVE}"
fetch "${BASE_URL}/SHA256SUMS" "${WORK_DIR}/SHA256SUMS"
verify_sha256 "${WORK_DIR}/${ARCHIVE}" "${WORK_DIR}/SHA256SUMS"

tar -xzf "${WORK_DIR}/${ARCHIVE}" -C "$WORK_DIR"
CANDIDATE="${WORK_DIR}/${STEM}/${BIN}"
chmod 0755 "$CANDIDATE"
"${CANDIDATE}" --help >/dev/null

mkdir -p "$INSTALL_DIR"
install -m 0755 "$CANDIDATE" "${INSTALL_DIR}/${BIN}"
echo "installed ${BIN} (${TAG}) to ${INSTALL_DIR}/${BIN}"
