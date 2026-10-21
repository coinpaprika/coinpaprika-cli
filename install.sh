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
    SUMS_URL="https://github.com/${REPO}/releases/download/${VERSION}/SHA256SUMS"

    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT

    info "Downloading ${URL}..."
    curl -sSfL "$URL" -o "${TMPDIR}/${ARCHIVE}" || err "Download failed. Check that a release exists for your platform (${PLATFORM}).\nURL: ${URL}"

    info "Verifying checksum..."
    curl -sSfL "$SUMS_URL" -o "${TMPDIR}/SHA256SUMS" || err "Could not download SHA256SUMS for ${VERSION} from the same release. Refusing to install an unverified binary."

    EXPECTED=$(awk -v f="$ARCHIVE" '{ name=$2; sub(/^\*/, "", name); if (name == f) print $1 }' "${TMPDIR}/SHA256SUMS")
    [ -z "$EXPECTED" ] && err "No checksum entry for ${ARCHIVE} in SHA256SUMS. Refusing to install an unverified binary."

    if command -v sha256sum >/dev/null 2>&1; then
        ACTUAL=$(sha256sum "${TMPDIR}/${ARCHIVE}" | awk '{print $1}')
    elif command -v shasum >/dev/null 2>&1; then
        ACTUAL=$(shasum -a 256 "${TMPDIR}/${ARCHIVE}" | awk '{print $1}')
    else
        err "Neither sha256sum nor shasum is available to verify the download."
    fi

    [ "$EXPECTED" = "$ACTUAL" ] || err "Checksum mismatch for ${ARCHIVE}.\nExpected: ${EXPECTED}\nGot:      ${ACTUAL}\nRefusing to install."
    ok "Checksum verified."

    info "Extracting..."
    EXTRACT_DIR="${TMPDIR}/extract"
    mkdir -p "$EXTRACT_DIR"
    LISTING="${TMPDIR}/listing.txt"

    if [ "$EXT" = "zip" ]; then
        need unzip
        unzip -Z1 "${TMPDIR}/${ARCHIVE}" > "$LISTING"
    else
        tar tzf "${TMPDIR}/${ARCHIVE}" > "$LISTING"
    fi

    # Reject any archive member that could escape EXTRACT_DIR before we
    # extract anything.
    while IFS= read -r member; do
        case "$member" in
            /*) err "Refusing to extract archive member with an absolute path: ${member}" ;;
            *..*) err "Refusing to extract archive member containing '..': ${member}" ;;
        esac
    done < "$LISTING"

    if [ "$EXT" = "zip" ]; then
        unzip -q "${TMPDIR}/${ARCHIVE}" -d "$EXTRACT_DIR"
    else
        tar xzf "${TMPDIR}/${ARCHIVE}" -C "$EXTRACT_DIR"
    fi

    # Find the binary (it may be at the top level or inside a directory)
    BIN_PATH=$(find "$EXTRACT_DIR" -name "$BINARY" -type f | head -1)
    [ -z "$BIN_PATH" ] && BIN_PATH=$(find "$EXTRACT_DIR" -name "${BINARY}.exe" -type f | head -1)
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
