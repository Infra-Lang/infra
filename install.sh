#!/bin/bash

# Infra Programming Language Installer
# This script downloads and installs the latest version of Infra

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Functions
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Detect architecture
detect_arch() {
    ARCH=$(uname -m)
    case $ARCH in
        x86_64)
            ARCH="linux"
            ;;
        aarch64|arm64)
            ARCH="linux-arm64"
            ;;
        armv7l)
            ARCH="linux-armv7"
            ;;
        *)
            print_error "Unsupported architecture: $ARCH"
            exit 1
            ;;
    esac
    print_info "Detected architecture: $ARCH"
}

# Get latest version from GitHub API
get_latest_version() {
    print_info "Fetching latest version information..."

    # Try to get version from GitHub API
    REPO_OWNER=$(git config --get remote.origin.url 2>/dev/null | sed -n 's/.*github.com[:/]\([^/]*\)\/.*/\1/p' || echo "infra-lang")
    REPO_NAME=$(git config --get remote.origin.url 2>/dev/null | sed -n 's/.*github.com[:/]\([^/]*\/\([^.]*\)\).*/\2/p' || echo "infra")

    if command -v curl >/dev/null 2>&1; then
        VERSION=$(curl -s "https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/' || echo "latest")
    elif command -v wget >/dev/null 2>&1; then
        VERSION=$(wget -qO- "https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest" | grep '"tag_name"' | sed -E 's/.*"([^"]+)".*/\1/' || echo "latest")
    else
        print_warning "curl or wget not found, using latest version"
        VERSION="latest"
    fi

    print_info "Version: $VERSION"
}

# Download and install Infra
install_infra() {
    detect_arch
    get_latest_version

    # Installation directories
    INSTALL_DIR="$HOME/.infra"
    BIN_DIR="$INSTALL_DIR/bin"
    BINARY_NAME="infra-$ARCH"

    # Create directories
    print_info "Creating installation directories..."
    mkdir -p "$BIN_DIR"

    # Download URL
    BASE_URL="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases"
    if [ "$VERSION" = "latest" ]; then
        DOWNLOAD_URL="${BASE_URL}/latest/download/${BINARY_NAME}"
    else
        DOWNLOAD_URL="${BASE_URL}/download/${VERSION}/${BINARY_NAME}"
    fi

    print_info "Downloading Infra from: $DOWNLOAD_URL"

    # Download binary
    if command -v curl >/dev/null 2>&1; then
        curl -L -o "$BIN_DIR/infra" "$DOWNLOAD_URL" || {
            print_error "Failed to download Infra"
            exit 1
        }
    elif command -v wget >/dev/null 2>&1; then
        wget -O "$BIN_DIR/infra" "$DOWNLOAD_URL" || {
            print_error "Failed to download Infra"
            exit 1
        }
    else
        print_error "Neither curl nor wget is available. Please install one of them and try again."
        exit 1
    fi

    # Make executable
    chmod +x "$BIN_DIR/infra"

    # Add to PATH
    update_shell_configs

    print_success "Infra has been installed successfully!"
    print_info "Binary location: $BIN_DIR/infra"
    print_info "Run 'infra --version' to verify installation."
    print_info "You may need to restart your shell or run 'source ~/.bashrc' to use the 'infra' command."
}

# Update shell configuration files
update_shell_configs() {
    SHELL_RC=""

    # Detect shell and update appropriate config file
    if [ -n "$BASH_VERSION" ]; then
        SHELL_RC="$HOME/.bashrc"
    elif [ -n "$ZSH_VERSION" ]; then
        SHELL_RC="$HOME/.zshrc"
    else
        SHELL_RC="$HOME/.profile"
    fi

    # Add to PATH if not already present
    if ! grep -q "$BIN_DIR" "$SHELL_RC" 2>/dev/null; then
        print_info "Adding Infra to PATH in $SHELL_RC"
        echo "" >> "$SHELL_RC"
        echo "# Infra Programming Language" >> "$SHELL_RC"
        echo "export PATH=\"$BIN_DIR:\$PATH\"" >> "$SHELL_RC"
    fi
}

# Check for existing installation
check_existing() {
    if command -v infra >/dev/null 2>&1; then
        EXISTING_VERSION=$(infra --version 2>/dev/null || echo "unknown")
        print_warning "Infra is already installed (version: $EXISTING_VERSION)"
        read -p "Do you want to continue and overwrite the existing installation? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            print_info "Installation cancelled."
            exit 0
        fi
    fi
}

# Main installation flow
main() {
    echo "🚀 Infra Programming Language Installer"
    echo "======================================="
    echo

    check_existing

    # Check prerequisites
    if ! command -v curl >/dev/null 2>&1 && ! command -v wget >/dev/null 2>&1; then
        print_error "Neither curl nor wget is available. Please install one of them and try again."
        exit 1
    fi

    install_infra

    echo
    print_success "Installation completed! 🎉"
    echo
    print_info "Next steps:"
    echo "  1. Restart your shell or run: source $SHELL_RC"
    echo "  2. Verify installation: infra --version"
    echo "  3. Try the REPL: infra --repl"
    echo "  4. Read the docs: https://github.com/${REPO_OWNER}/${REPO_NAME}#readme"
    echo
}

# Run the installer
main "$@"
