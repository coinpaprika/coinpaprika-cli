#!/bin/sh
# CoinPaprika CLI installer
# Usage: curl -sSL https://raw.githubusercontent.com/coinpaprika/coinpaprika-cli/main/install.sh | sh
#
# Detects your OS/arch, downloads the latest release binary from GitHub, and
# drops it into ~/.local/bin (or /usr/local/bin with sudo).

set -e

REPO="coinpaprika/coinpaprika-cli"
BINARY="coinpaprika-cli"
INSTALL_DIR="${COINPAPRIKA_INSTALL_DIR:-$HOME/.local/bin}"

# ---------------------------------------------------------------------------
# helpers
# ---------------------------------------------------------------------------
info()  { printf '\033[1;34m%s\033[0m\n' "$*"; }
ok()    { printf '\033[1;32m%s\033[0m\n' "$*"; }
err()   { printf '\033[1;31mError: %s\033[0m\n' "$*" >&2; exit 1; }

need() {
    command -v "$1" >/dev/null 2>&1 || err "'$1' is required but not found. Install it and try again."
}

# ---------------------------------------------------------------------------
# detect platform
# ---------------------------------------------------------------------------
detect_platform() {
    OS="$(uname -s)"
    ARCH="$(uname -m)"

    case "$OS" in
        Linux)  OS="linux"  ;;
        Darwin) OS="darwin" ;;
        MINGW*|MSYS*|CYGWIN*) OS="windows" ;;
        *) err "Unsupported OS: $OS" ;;
    esac

    case "$ARCH" in
        x86_64|amd64)  ARCH="x86_64"  ;;
        arm64|aarch64) ARCH="aarch64" ;;
        *) err "Unsupported architecture: $ARCH" ;;
    esac

    PLATFORM="${OS}-${ARCH}"
}

# ---------------------------------------------------------------------------
# find latest release tag
# ---------------------------------------------------------------------------
get_latest_version() {
    need curl
    VERSION=$(curl -sSf "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name"' | head -1 | sed 's/.*"tag_name": *"//;s/".*//')

    [ -z "$VERSION" ] && err "Could not determine latest version. Check https://github.com/${REPO}/releases"
    info "Latest version: $VERSION"
}

# ---------------------------------------------------------------------------
# download & install
# ---------------------------------------------------------------------------
download_and_install() {
    EXT="tar.gz"
    [ "$OS" = "windows" ] && EXT="zip"

    ARCHIVE="${BINARY}-${VERSION}-${PLATFORM}.${EXT}"
    URL="https://github.com/${REPO}/releases/download/${VERSION}/${ARCHIVE}"

    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT

    info "Downloading ${URL}..."
    curl -sSfL "$URL" -o "${TMPDIR}/${ARCHIVE}" || err "Download failed. Check that a release exists for your platform (${PLATFORM}).\nURL: ${URL}"

    info "Extracting..."
    if [ "$EXT" = "zip" ]; then
        need unzip
        unzip -q "${TMPDIR}/${ARCHIVE}" -d "$TMPDIR"
    else
        tar xzf "${TMPDIR}/${ARCHIVE}" -C "$TMPDIR"
    fi

    # Find the binary (it may be at the top level or inside a directory)
    BIN_PATH=$(find "$TMPDIR" -name "$BINARY" -type f | head -1)
    [ -z "$BIN_PATH" ] && BIN_PATH=$(find "$TMPDIR" -name "${BINARY}.exe" -type f | head -1)
    [ -z "$BIN_PATH" ] && err "Could not find '${BINARY}' binary in the archive"

    chmod +x "$BIN_PATH"

    # Install
    mkdir -p "$INSTALL_DIR"
    if [ -w "$INSTALL_DIR" ]; then
        mv "$BIN_PATH" "${INSTALL_DIR}/${BINARY}"
    else
        info "Need sudo to write to ${INSTALL_DIR}"
        sudo mv "$BIN_PATH" "${INSTALL_DIR}/${BINARY}"
    fi

    ok "Installed ${BINARY} to ${INSTALL_DIR}/${BINARY}"
}

# ---------------------------------------------------------------------------
# check PATH
# ---------------------------------------------------------------------------
check_path() {
    case ":$PATH:" in
        *":${INSTALL_DIR}:"*) ;;
        *)
            printf '\n'
            info "Add this to your shell profile (~/.bashrc, ~/.zshrc, etc.):"
            printf '\n  export PATH="%s:$PATH"\n\n' "$INSTALL_DIR"
            ;;
    esac
}

# ---------------------------------------------------------------------------
# main
# ---------------------------------------------------------------------------
main() {
    info "Installing CoinPaprika CLI..."
    printf '\n'

    detect_platform
    info "Platform: ${PLATFORM}"

    get_latest_version
    download_and_install
    check_path

    printf '\n'
    ok "Done! Run 'coinpaprika-cli --help' to get started."
    printf '\n'
    printf '  Quick start: coinpaprika-cli onboard\n'
    printf '  Free tier works without an API key.\n'
    printf '\n'
}

main
