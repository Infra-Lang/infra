#!/bin/bash

# Infra Programming Language Installer
# This script downloads and installs the latest version of Infra
# Supports both fresh installation and upgrading existing installations

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
UPGRADE_MODE=false
INSTALL_DIR="$HOME/.infra"
REPO_OWNER="infra-lang"
REPO_NAME="infra"

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

# Check if infra is already installed
is_infra_installed() {
    if command -v infra >/dev/null 2>&1; then
        return 0
    else
        return 1
    fi
}

# Get current installed version
get_installed_version() {
    if is_infra_installed; then
        local version_output
        version_output=$(infra --version 2>/dev/null || echo "unknown")
        # Extract version number from various output formats
        echo "$version_output" | grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+' | head -1 || echo "$version_output"
    else
        echo "not installed"
    fi
}

# Get latest release version from GitHub API
get_latest_version() {
    local latest_version
    local api_url="https://api.github.com/repos/${REPO_OWNER}/${REPO_NAME}/releases/latest"

    if command -v curl >/dev/null 2>&1; then
        latest_version=$(curl -s "$api_url" | grep '"tag_name":' | cut -d'"' -f4 | sed 's/v//' || echo "latest")
    elif command -v wget >/dev/null 2>&1; then
        latest_version=$(wget -qO- "$api_url" | grep '"tag_name":' | cut -d'"' -f4 | sed 's/v//' || echo "latest")
    else
        print_error "Neither curl nor wget is available"
        exit 1
    fi

    echo "$latest_version"
}

# Compare version strings (returns 0 if equal, 1 if first > second, 2 if first < second)
compare_versions() {
    local v1="$1"
    local v2="$2"

    if [[ "$v1" == "$v2" ]]; then
        return 0
    fi

    # Convert version strings to arrays for comparison
    IFS='.' read -ra V1 <<< "${v1#v}"
    IFS='.' read -ra V2 <<< "${v2#v}"

    # Compare major, minor, patch
    for i in {0..2}; do
        local part1=${V1[i]:-0}
        local part2=${V2[i]:-0}

        if ((part1 > part2)); then
            return 1
        elif ((part1 < part2)); then
            return 2
        fi
    done

    return 0
}

# Detect platform and architecture
detect_platform() {
    local platform
    local arch

    case "$(uname -s)" in
        Linux*)
            platform="linux"
            ;;
        Darwin*)
            platform="macos"
            ;;
        CYGWIN*|MINGW*|MSYS*)
            platform="windows"
            ;;
        *)
            print_error "Unsupported platform: $(uname -s)"
            exit 1
            ;;
    esac

    case "$(uname -m)" in
        x86_64)
            arch="x86_64"
            ;;
        aarch64|arm64)
            arch="arm64"
            ;;
        armv7l)
            arch="armv7"
            ;;
        *)
            print_error "Unsupported architecture: $(uname -m)"
            exit 1
            ;;
    esac

    echo "${platform}-${arch}"
}

# Download and install/upgrade infra
download_and_install() {
    local version="$1"
    local mode="$2"
    local platform_arch=$(detect_platform)
    local platform="${platform_arch%%-*}"

    print_info "Detected platform: $platform_arch"

    # Construct download URL
    local filename="infra-$platform"
    if [ "$platform" = "windows" ]; then
        filename="$filename.zip"
        local download_url="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/v$version/infra-windows.zip"
    else
        filename="$platform_arch.tar.gz"
        local download_url="https://github.com/${REPO_OWNER}/${REPO_NAME}/releases/download/v$version/infra-$platform_arch.tar.gz"
    fi

    print_info "Downloading Infra v$version for $platform_arch..."

    # Create temp directory for download
    local temp_dir="/tmp/infra-install-$$"
    mkdir -p "$temp_dir"

    # Download the release
    if command -v curl >/dev/null 2>&1; then
        curl -L "$download_url" -o "$temp_dir/$filename"
    else
        print_error "curl is required to download Infra"
        print_info "Please install curl and try again"
        rm -rf "$temp_dir"
        exit 1
    fi

    # Extract and install
    cd "$temp_dir"
    if [ "$platform" = "windows" ]; then
        print_info "Extracting Windows archive..."
        if command -v unzip >/dev/null 2>&1; then
            unzip -q "$filename" -d "$temp_dir"
        else
            print_error "unzip is required to extract Windows archives"
            rm -rf "$temp_dir"
            exit 1
        fi

        # Move .exe to infra.exe for Windows
        if [ -f "$temp_dir/release/infra.exe" ]; then
            mv "$temp_dir/release/infra.exe" "$INSTALL_DIR/infra.exe"
        fi
    else
        print_info "Extracting Unix archive..."
        if command -v tar >/dev/null 2>&1; then
            tar -xzf "$filename" -C "$temp_dir"
        else
            print_error "tar is required to extract Unix archives"
            rm -rf "$temp_dir"
            exit 1
        fi

        # Move binary from release directory
        if [ -f "$temp_dir/release/infra" ]; then
            mv "$temp_dir/release/infra" "$INSTALL_DIR/infra"
        fi
    fi

    # Make binary executable (Unix systems)
    if [ "$platform" != "windows" ]; then
        chmod +x "$INSTALL_DIR/infra"
    fi

    # Create bin directory and symlink
    local bin_dir="$INSTALL_DIR/bin"
    mkdir -p "$bin_dir"

    # Remove old symlink if exists
    if [ -L "$bin_dir/infra" ]; then
        rm "$bin_dir/infra"
    fi
    ln -sf "$INSTALL_DIR/infra" "$bin_dir/infra"

    # Clean up
    rm -rf "$temp_dir"

    print_success "Infra v$version installed successfully!"
    print_info "Installation location: $INSTALL_DIR"
    print_info "Binary location: $bin_dir/infra"

    # Show version
    if [ "$mode" = "upgrade" ]; then
        print_info "Upgrade completed!"
    fi
}

# Add to PATH in shell configurations
add_to_path() {
    local bin_dir="$1"
    local shell_configs=("$HOME/.bashrc" "$HOME/.zshrc" "$HOME/.profile")
    local added=false

    for config_file in "${shell_configs[@]}"; do
        if [ -f "$config_file" ] && ! grep -q "$INSTALL_DIR/bin" "$config_file" 2>/dev/null; then
            echo "" >> "$config_file"
            echo "# Infra Programming Language" >> "$config_file"
            echo "export PATH=\"$INSTALL_DIR/bin:\$PATH\"" >> "$config_file"
            print_info "Added Infra to PATH in $(basename "$config_file")"
            added=true
        fi
    done

    if [ "$added" = true ]; then
        print_warning "Run 'source ~/.bashrc' or restart your shell to update your PATH"
    fi
}

# Parse command line arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --upgrade|-u)
                UPGRADE_MODE=true
                shift
                ;;
            --help|-h)
                show_help
                exit 0
                ;;
            --version)
                echo "Infra Installer v1.0.0"
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
}

# Show help information
show_help() {
    echo "Infra Programming Language Installer"
    echo "===================================="
    echo
    echo "Usage: $0 [OPTIONS]"
    echo
    echo "Options:"
    echo "  --upgrade, -u    Upgrade existing installation to latest version"
    echo "  --help, -h       Show this help message"
    echo "  --version       Show installer version"
    echo
    echo "Examples:"
    echo "  $0                    # Fresh installation"
    echo "  $0 --upgrade          # Upgrade existing installation"
    echo
}

# Main installation logic
main() {
    echo "🚀 Infra Programming Language Installer v1.0.0"
    echo "==============================================="
    echo

    # Parse command line arguments
    parse_args "$@"

    # Create install directory
    mkdir -p "$INSTALL_DIR"

    if [ "$UPGRADE_MODE" = true ]; then
        # Upgrade mode
        if is_infra_installed; then
            local installed_version=$(get_installed_version)
            local latest_version=$(get_latest_version)

            # Remove 'v' prefix if present for comparison
            installed_clean="${installed_version#v}"

            if [ "$installed_clean" = "$latest_version" ] || [ "$installed_version" = "latest" ]; then
                print_success "Infra is already up to date ($installed_version)"
                exit 0
            else
                print_info "Upgrading Infra from $installed_version to v$latest_version..."
                download_and_install "$latest_version" "upgrade"
                add_to_path "$INSTALL_DIR/bin"
            fi
        else
            print_error "Infra is not installed. Use without --upgrade flag for fresh installation."
            exit 1
        fi
    else
        # Fresh installation
        if is_infra_installed; then
            local installed_version=$(get_installed_version)
            print_warning "Infra is already installed ($installed_version)"
            print_info "Use --upgrade flag to upgrade to the latest version"
            exit 0
        else
            local latest_version=$(get_latest_version)
            print_info "Installing Infra v$latest_version..."
            download_and_install "$latest_version" "install"
            add_to_path "$INSTALL_DIR/bin"
        fi
    fi

    echo
    print_success "Installation complete! 🎉"
    echo
    print_info "Next steps:"
    echo "  1. Run 'source ~/.bashrc' or restart your shell to update PATH"
    echo "  2. Verify installation: infra --version"
    echo "  3. Try the REPL: infra --repl"
    echo "  4. Read the docs: https://github.com/${REPO_OWNER}/${REPO_NAME}#readme"
    echo
}

# Run main function with all arguments
main "$@"
