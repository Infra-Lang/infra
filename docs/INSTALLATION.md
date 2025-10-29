# Infra Installation Guide

This comprehensive guide covers all installation methods for the Infra programming language on Windows, macOS, and Linux.

## Table of Contents

- [System Requirements](#system-requirements)
- [Quick Installation](#quick-installation)
- [Installation Methods](#installation-methods)
  - [Pre-built Binaries](#pre-built-binaries-recommended)
  - [Package Managers](#package-managers)
  - [Build from Source](#build-from-source)
  - [Docker](#docker)
- [Platform-Specific Instructions](#platform-specific-instructions)
  - [Windows](#windows)
  - [macOS](#macos)
  - [Linux](#linux)
- [Verification](#verification)
- [Troubleshooting](#troubleshooting)
- [Uninstallation](#uninstallation)
- [Upgrading](#upgrading)

## System Requirements

### Minimum Requirements
- **OS**: Windows 10+, macOS 10.15+, or Linux (Ubuntu 18.04+, CentOS 7+, etc.)
- **Architecture**: x86_64 (64-bit) or ARM64
- **Memory**: 4GB RAM minimum
- **Disk Space**: 50MB for the Infra compiler
- **Network**: Internet connection for package managers and downloads

### Recommended Requirements
- **OS**: Latest stable version of your OS
- **Architecture**: x86_64 or ARM64 with native support
- **Memory**: 8GB+ RAM
- **Disk Space**: 100MB+ for projects and dependencies
- **Network**: Broadband connection

## Quick Installation

### One-Line Installation (Unix-like systems)

```bash
# Download and install automatically
curl -fsSL https://raw.githubusercontent.com/infra-lang/infra/main/install.sh | sh

# Or with wget
wget -qO- https://raw.githubusercontent.com/infra-lang/infra/main/install.sh | sh
```

### Windows PowerShell (Administrator)

```powershell
# Download and install automatically
iwr -useb https://raw.githubusercontent.com/infra-lang/infra/main/install.ps1 | iex
```

## Installation Methods

### Pre-built Binaries (Recommended)

The easiest way to get started is by downloading pre-compiled binaries from the [GitHub Releases](https://github.com/infra-lang/infra/releases/latest).

#### Windows

1. **Download the binary**:
   ```powershell
   # Direct download from GitHub releases
   Invoke-WebRequest -Uri "https://github.com/infra-lang/infra/releases/latest/download/infra-windows.exe" -OutFile "infra.exe"
   
   # Or use the automated installer (recommended)
   Invoke-WebRequest -Uri "https://raw.githubusercontent.com/infra-lang/infra/main/install.ps1" -OutFile "install.ps1"
   Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
   .\install.ps1
   ```

2. **Add to PATH** (optional but recommended):
   ```powershell
   # Add current directory to PATH (temporary)
   $env:PATH += ";$(Get-Location)"
   
   # Or add permanently (run as Administrator)
   [Environment]::SetEnvironmentVariable("PATH", $env:PATH + ";C:\infra", "User")
   ```

3. **Verify installation**:
   ```powershell
   .\infra.exe --version
   ```

#### macOS

1. **Download the binary**:
   ```bash
   # Use automated installer (recommended)
   curl -fsSL https://raw.githubusercontent.com/infra-lang/infra/main/install.sh | sh
   
   # Or manual download:
   # Intel Mac
   curl -L "https://github.com/infra-lang/infra/releases/latest/download/infra-macos" -o infra
   
   # Apple Silicon (M1/M2)
   curl -L "https://github.com/infra-lang/infra/releases/latest/download/infra-macos-arm64" -o infra
   ```

2. **Make executable and move to PATH**:
   ```bash
   chmod +x infra
   sudo mv infra /usr/local/bin/infra
   ```

3. **Verify installation**:
   ```bash
   infra --version
   ```

#### Linux

1. **Download the binary**:
   ```bash
   # Use automated installer (recommended)
   curl -fsSL https://raw.githubusercontent.com/infra-lang/infra/main/install.sh | sh
   
   # Or manual download:
   # x86_64
   curl -L "https://github.com/infra-lang/infra/releases/latest/download/infra-linux" -o infra
   
   # ARM64
   curl -L "https://github.com/infra-lang/infra/releases/latest/download/infra-linux-arm64" -o infra
   ```

2. **Make executable and move to PATH**:
   ```bash
   chmod +x infra
   sudo mv infra /usr/local/bin/infra
   ```

3. **Verify installation**:
   ```bash
   infra --version
   ```

### Package Managers

#### Homebrew (macOS)

```bash
# Install Infra
brew install infra-lang/infra/infra

# Update Infra
brew upgrade infra-lang/infra/infra

# Uninstall
brew uninstall infra-lang/infra/infra
```

#### MacPorts (macOS)

```bash
# Install Infra
sudo port install infra

# Update Infra
sudo port selfupdate
sudo port upgrade infra

# Uninstall
sudo port uninstall infra
```

#### APT (Debian/Ubuntu)

```bash
# Add repository
echo "deb [trusted=yes] https://apt.infra-lang.org stable main" | sudo tee /etc/apt/sources.list.d/infra.list

# Update package list
sudo apt update

# Install Infra
sudo apt install infra

# Update Infra
sudo apt update && sudo apt upgrade infra

# Uninstall
sudo apt remove infra
```

#### YUM/DNF (Red Hat/CentOS/Fedora)

```bash
# Add repository
sudo yum-config-manager --add-repo https://yum.infra-lang.org/infra.repo

# Install Infra
sudo yum install infra

# Update Infra
sudo yum update infra

# Uninstall
sudo yum remove infra
```

#### Pacman (Arch Linux)

```bash
# Install from AUR
yay -S infra-lang

# Or using paru
paru -S infra-lang

# Update Infra
yay -Syu infra-lang

# Uninstall
yay -R infra-lang
```

#### Chocolatey (Windows)

```powershell
# Install Infra
choco install infra-lang

# Update Infra
choco upgrade infra-lang

# Uninstall
choco uninstall infra-lang
```

#### Scoop (Windows)

```powershell
# Add bucket (if not already added)
scoop bucket add infra-lang https://github.com/infra-lang/scoop.git

# Install Infra
scoop install infra-lang/infra

# Update Infra
scoop update infra-lang/infra

# Uninstall
scoop uninstall infra-lang/infra
```

#### Cargo (Rust package manager)

```bash
# Install Infra
cargo install infra-lang

# Update Infra
cargo install --force infra-lang

# Uninstall
cargo uninstall infra-lang
```

#### Nix/Nixpkgs

```bash
# Install Infra
nix-env -iA nixpkgs.infra

# Update Infra
nix-env -uA nixpkgs.infra

# Uninstall
nix-env -e nixpkgs.infra
```

### Build from Source

If you prefer to build from source or need a custom build:

#### Prerequisites

You need to have Rust installed. If you don't have Rust:

```bash
# Install Rust (https://rustup.rs/)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### Building

```bash
# Clone the repository
git clone https://github.com/infra-lang/infra.git
cd infra

# Build in release mode (optimized)
cargo build --release

# Or build in debug mode (faster compilation)
cargo build

# Install to system path
cargo install --path .

# Run tests
cargo test

# Run benchmarks
cargo bench
```

The compiled binary will be at:
- `target/release/infra` (release build)
- `target/debug/infra` (debug build)

#### Custom Installation

```bash
# Install to custom location
cargo install --path . --root /opt/infra

# Add to PATH
echo 'export PATH="/opt/infra/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

### Docker

#### Using Official Docker Image

```bash
# Pull the latest image
docker pull infralang/infra:latest

# Run Infra in a container
docker run --rm -v $(pwd):/workspace infralang/infra:latest infra /workspace/hello.if

# Interactive shell
docker run --rm -it -v $(pwd):/workspace infralang/infra:latest bash
```

#### Building Docker Image

```bash
# Clone and build
git clone https://github.com/infra-lang/infra.git
cd infra

# Build Docker image
docker build -t infra-local .

# Run local image
docker run --rm -v $(pwd):/workspace infra-local infra /workspace/hello.if
```

## Platform-Specific Instructions

### Windows

#### Method 1: PowerShell (Recommended)

```powershell
# Check if running as Administrator
if (-NOT ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Error "Please run PowerShell as Administrator"
    exit 1
}

# Download and install
$downloadUrl = "https://github.com/infra-lang/infra/releases/latest/download/infra-windows.exe"
$installPath = "C:\Program Files\Infra"

# Create installation directory
New-Item -ItemType Directory -Force -Path $installPath

# Download
Invoke-WebRequest -Uri $downloadUrl -OutFile "$installPath\infra.exe"

# Add to PATH for current user
$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$installPath*") {
    [Environment]::SetEnvironmentVariable("PATH", "$userPath;$installPath", "User")
    $env:PATH += ";$installPath"
}

# Verify
& "$installPath\infra.exe" --version
```

#### Method 2: Chocolatey

```powershell
# Install Chocolatey (if not installed)
Set-ExecutionPolicy Bypass -Scope Process -Force; [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072; iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Install Infra
choco install infra-lang
```

#### Method 3: Windows Subsystem for Linux (WSL)

```bash
# Inside WSL
curl -L "https://github.com/infra-lang/infra/releases/latest/download/infra-linux" -o infra
chmod +x infra
sudo mv infra /usr/local/bin/infra
```

#### VS Code Integration

1. Install the [Infra Language Support](https://marketplace.visualstudio.com/items?itemName=infra-lang.infra) extension
2. Open VS Code settings (Ctrl+,)
3. Search for "infra.executablePath"
4. Set it to the full path to your infra.exe (e.g., `C:\Program Files\Infra\infra.exe`)

### macOS

#### Method 1: Homebrew (Recommended)

```bash
# Install Homebrew (if not installed)
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Infra
brew install infra-lang/infra/infra

# Verify
infra --version
```

#### Method 2: Manual Installation

```bash
# Detect architecture
ARCH=$(uname -m)
if [ "$ARCH" = "arm64" ]; then
    INFRA_ARCH="macos-arm64"
else
    INFRA_ARCH="macos"
fi

# Download
curl -L "https://github.com/infra-lang/infra/releases/latest/download/infra-${INFRA_ARCH}" -o infra

# Make executable
chmod +x infra

# Move to PATH
sudo mv infra /usr/local/bin/infra

# Verify
infra --version
```

#### Method 3: MacPorts

```bash
# Install MacPorts (https://www.macports.org/install.php)

# Install Infra
sudo port install infra

# Verify
infra --version
```

### Linux

#### Debian/Ubuntu

```bash
# Method 1: APT repository (recommended)
curl -fsSL https://apt.infra-lang.org/infra.gpg | sudo apt-key add -
echo "deb https://apt.infra-lang.org stable main" | sudo tee /etc/apt/sources.list.d/infra.list
sudo apt update
sudo apt install infra

# Method 2: Manual download
ARCH=$(uname -m)
if [ "$ARCH" = "aarch64" ]; then
    INFRA_ARCH="linux-arm64"
else
    INFRA_ARCH="linux"
fi

curl -L "https://github.com/infra-lang/infra/releases/latest/download/infra-${INFRA_ARCH}" -o infra
chmod +x infra
sudo mv infra /usr/local/bin/infra
```

#### Red Hat/CentOS/Fedora

```bash
# Method 1: YUM repository (recommended)
sudo yum-config-manager --add-repo https://yum.infra-lang.org/infra.repo
sudo yum install infra

# Method 2: Manual download
ARCH=$(uname -m)
if [ "$ARCH" = "aarch64" ]; then
    INFRA_ARCH="linux-arm64"
else
    INFRA_ARCH="linux"
fi

curl -L "https://github.com/infra-lang/infra/releases/latest/download/infra-${INFRA_ARCH}" -o infra
chmod +x infra
sudo mv infra /usr/local/bin/infra
```

#### Arch Linux

```bash
# Method 1: AUR helper (recommended)
yay -S infra-lang

# Method 2: Manual AUR build
git clone https://aur.archlinux.org/infra-lang.git
cd infra-lang
makepkg -si

# Verify
infra --version
```

## Verification

After installation, verify that Infra is working correctly:

```bash
# Check version
infra --version

# Check help
infra --help

# Run a simple program
echo 'print("Hello, Infra!")' > test.if
infra test.if

# Start REPL
infra --repl
```

Expected output:
```
Infra 0.1.1 (built with Rust 1.70.0)
Type 'exit' or press Ctrl+D to exit
>>> print("Hello, Infra!")
Hello, Infra!
>>> exit
```

## Troubleshooting

### Common Issues

#### "command not found: infra"

**Solution**: Ensure Infra is in your PATH

```bash
# Check if infra is in PATH
which infra

# If not found, add it manually
echo 'export PATH="/usr/local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc

# Or for the current session
export PATH="/usr/local/bin:$PATH"
```

#### "Permission denied"

**Solution**: Make the binary executable

```bash
chmod +x infra
```

#### "libssl not found" (Linux)

**Solution**: Install OpenSSL development libraries

```bash
# Debian/Ubuntu
sudo apt install libssl-dev pkg-config

# Red Hat/CentOS
sudo yum install openssl-devel

# Arch Linux
sudo pacman -S openssl
```

#### "Rust not found" (Building from source)

**Solution**: Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

#### Network issues during download

**Solution**: Use a proxy or alternative mirror

```bash
# Use proxy
export https_proxy=http://proxy.example.com:8080
curl -L "https://github.com/infra-lang/infra/releases/latest/download/infra-linux" -o infra

# Or use alternative mirror
curl -L "https://mirror.infra-lang.org/releases/latest/infra-linux" -o infra
```

### Getting Help

If you encounter issues:

1. **Check the logs**: Run with `--verbose` flag
   ```bash
   infra --verbose program.if
   ```

2. **Check system compatibility**:
   ```bash
   # Check architecture
   uname -m
   
   # Check OS version
   cat /etc/os-release
   ```

3. **Report issues**: [Create an issue on GitHub](https://github.com/infra-lang/infra/issues)

4. **Community support**: 
   - [Discord Server](https://discord.gg/infra)
   - [GitHub Discussions](https://github.com/infra-lang/infra/discussions)

## Uninstallation

### Pre-built Binaries

```bash
# Remove the binary
sudo rm /usr/local/bin/infra

# Or if installed elsewhere
sudo rm /path/to/infra

# Remove VS Code extension (optional)
code --uninstall-extension infra-lang.infra
```

### Package Managers

```bash
# Homebrew
brew uninstall infra-lang/infra/infra

# APT
sudo apt remove infra

# YUM/DNF
sudo yum remove infra

# Pacman/AUR
yay -R infra-lang

# Chocolatey
choco uninstall infra-lang

# Scoop
scoop uninstall infra-lang/infra

# Cargo
cargo uninstall infra-lang
```

### Build from Source

```bash
# If installed with cargo
cargo uninstall infra-lang

# If manually installed
sudo rm /usr/local/bin/infra

# Remove source directory
rm -rf ~/infra
```

## Upgrading

### Automatic Upgrades

```bash
# Using install script (recommended)
curl -fsSL https://infra-lang.org/install.sh | sh

# Windows PowerShell
iwr -useb https://infra-lang.org/install.ps1 | iex
```

### Manual Upgrades

```bash
# Download latest version
curl -L "https://github.com/infra-lang/infra/releases/latest/download/infra-linux" -o infra-new
chmod +x infra-new

# Replace old version
sudo mv infra-new /usr/local/bin/infra
```

### Package Manager Upgrades

```bash
# Homebrew
brew upgrade infra-lang/infra/infra

# APT
sudo apt update && sudo apt upgrade infra

# YUM/DNF
sudo yum update infra

# Cargo
cargo install --force infra-lang
```

### Development Versions

For users who want to try the latest features:

```bash
# Install from git master branch
cargo install --git https://github.com/infra-lang/infra.git --branch main

# Or build manually
git clone https://github.com/infra-lang/infra.git
cd infra
git checkout main
cargo build --release
sudo mv target/release/infra /usr/local/bin/infra
```

**Note**: Development versions may have bugs and incomplete features.

## Next Steps

After successful installation:

1. **Read the [Language Guide](LANGUAGE_GUIDE.md)** - Comprehensive language reference
2. **Try the [Quick Start Tutorial](QUICK_START.md)** - Learn Infra in 15 minutes
3. **Install VS Code Extension** - Better development experience
4. **Join the Community** - Get help and contribute
5. **Build Your First Project** - Start with a simple "Hello World" program

Happy coding with Infra! 🚀