#!/bin/bash

# theoshot installer script
# This script downloads the latest release of theoshot, installs it,
# and handles system dependencies automatically.

set -e

REPO="alvescruz/theoshot"
BINARY_NAME="theoshot"
INSTALL_DIR="/usr/local/bin"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

printf "${BLUE}==>${NC} Installing ${GREEN}theoshot${NC}...\n"

# Check basic requirements
check_dep() {
    if ! command -v "$1" >/dev/null 2>&1; then
        printf "${YELLOW}Info:${NC} $1 is required for the installer.\n"
        return 1
    fi
    return 0
}

# Function to install system dependencies
install_sys_deps() {
    printf "${BLUE}==>${NC} Checking system dependencies...\n"
    
    if [ -f /etc/os-release ]; then
        . /etc/os-release
        case "$ID" in
            ubuntu|debian|pop|mint|kali)
                PKGS="libgtk-3-0 libpipewire-0.3-0 libdbus-1-3 curl jq"
                MISSING=""
                for pkg in $PKGS; do
                    if ! dpkg -s "$pkg" >/dev/null 2>&1; then
                        MISSING="$MISSING $pkg"
                    fi
                done
                if [ -n "$MISSING" ]; then
                    printf "${BLUE}==>${NC} Installing missing dependencies: ${YELLOW}$MISSING${NC}...\n"
                    sudo apt-get update -qq
                    sudo apt-get install -y $MISSING
                fi
                ;;
            fedora|rhel|centos)
                PKGS="gtk3 pipewire-libs dbus-libs curl jq"
                MISSING=""
                for pkg in $PKGS; do
                    if ! rpm -q "$pkg" >/dev/null 2>&1; then
                        MISSING="$MISSING $pkg"
                    fi
                done
                if [ -n "$MISSING" ]; then
                    printf "${BLUE}==>${NC} Installing missing dependencies: ${YELLOW}$MISSING${NC}...\n"
                    sudo dnf install -y $MISSING
                fi
                ;;
            arch|manjaro)
                PKGS="gtk3 pipewire dbus curl jq"
                MISSING=""
                for pkg in $PKGS; do
                    if ! pacman -Qi "$pkg" >/dev/null 2>&1; then
                        MISSING="$MISSING $pkg"
                    fi
                done
                if [ -n "$MISSING" ]; then
                    printf "${BLUE}==>${NC} Installing missing dependencies: ${YELLOW}$MISSING${NC}...\n"
                    sudo pacman -Sy --noconfirm $MISSING
                fi
                ;;
            *)
                printf "${YELLOW}Warning:${NC} Unsupported distribution for automatic dependency installation.\n"
                printf "Please ensure you have: libgtk-3, libpipewire-0.3, and libdbus-1 installed.\n"
                ;;
        esac
    fi
}

# Ensure all dependencies are available
install_sys_deps

# Detect Architecture
ARCH=$(uname -m)
case $ARCH in
    x86_64)
        ASSET_NAME="theoshot-linux-amd64"
        ;;
    *)
        printf "${RED}Error:${NC} Unsupported architecture: $ARCH. theoshot currently only provides amd64 binaries via this script.\n"
        exit 1
        ;;
esac

# Get latest release tag
printf "${BLUE}==>${NC} Finding latest release...\n"
LATEST_RELEASE=$(curl -s "https://api.github.com/repos/$REPO/releases/latest" | jq -r .tag_name)

if [ "$LATEST_RELEASE" == "null" ] || [ -z "$LATEST_RELEASE" ]; then
    printf "${RED}Error:${NC} Could not find latest release on GitHub.\n"
    exit 1
fi

printf "${BLUE}==>${NC} Downloading version ${GREEN}$LATEST_RELEASE${NC} ($ARCH)...\n"
DOWNLOAD_URL="https://github.com/$REPO/releases/download/$LATEST_RELEASE/$ASSET_NAME"

# Download to temporary location with progress bar
TMP_DIR=$(mktemp -d)
if ! curl -L --progress-bar "$DOWNLOAD_URL" -o "$TMP_DIR/$BINARY_NAME"; then
    printf "${RED}Error:${NC} Failed to download binary from $DOWNLOAD_URL\n"
    rm -rf "$TMP_DIR"
    exit 1
fi

# Installation
printf "${BLUE}==>${NC} Installing to $INSTALL_DIR (requires sudo)...\n"
sudo mv "$TMP_DIR/$BINARY_NAME" "$INSTALL_DIR/$BINARY_NAME"
sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"

# Clean up
rm -rf "$TMP_DIR"

printf "\n${GREEN}Success!${NC} theoshot has been installed to $INSTALL_DIR\n"
printf "You can now run it by typing: ${BLUE}theoshot interactive${NC} or ${BLUE}theoshot screen${NC}\n"
