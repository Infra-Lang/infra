# Infra Programming Language Installer for Windows
# This script downloads and installs the latest version of Infra
# Supports both fresh installation and upgrading existing installations

param(
    [switch]$Force,
    [switch]$Upgrade,
    [switch]$Help,
    [string]$InstallPath = "$env:USERPROFILE\.infra"
)

# Default values
$script:UPGRADE_MODE = $Upgrade
$script:REPO_OWNER = "infra-lang"
$script:REPO_NAME = "infra"

# Colors for output
$Colors = @{
    Red = "Red"
    Green = "Green"
    Yellow = "Yellow"
    Blue = "Blue"
    White = "White"
}

function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Colors[$Color]
}

function Write-Info {
    param([string]$Message)
    Write-ColorOutput "ℹ️  $Message" "Blue"
}

function Write-Success {
    param([string]$Message)
    Write-ColorOutput "✅ $Message" "Green"
}

function Write-Warning {
    param([string]$Message)
    Write-ColorOutput "⚠️  $Message" "Yellow"
}

function Write-Error {
    param([string]$Message)
    Write-ColorOutput "❌ $Message" "Red"
}

# Detect architecture
function Get-Architecture {
    $arch = $env:PROCESSOR_ARCHITECTURE
    switch ($arch) {
        "AMD64" { return "windows" }
        "ARM64" { return "windows-arm64" }
        default {
            Write-Error "Unsupported architecture: $arch"
            exit 1
        }
    }
}

# Check if Infra is already installed
function Test-InfraInstalled {
    $infraCommand = Get-Command infra -ErrorAction SilentlyContinue
    return $null -ne $infraCommand
}

# Get current installed version
function Get-InstalledVersion {
    if (Test-InfraInstalled) {
        try {
            $versionOutput = & infra --version 2>$null
            # Extract version number from various output formats
            if ($versionOutput -match 'v[0-9]+\.[0-9]+\.[0-9]+') {
                return $matches[0]
            } else {
                return $versionOutput
            }
        } catch {
            return "unknown"
        }
    } else {
        return "not installed"
    }
}

# Get latest version from GitHub API
function Get-LatestVersion {
    Write-Info "Fetching latest version information..."

    try {
        # Try to get repo info from git config or use defaults
        $repoOwner = $script:REPO_OWNER
        $repoName = $script:REPO_NAME

        if (Get-Command git -ErrorAction SilentlyContinue) {
            try {
                $remoteUrl = git config --get remote.origin.url 2>$null
                if ($remoteUrl) {
                    if ($remoteUrl -match 'github\.com[/:]([^/]+)/([^/.]+)') {
                        $repoOwner = $matches[1]
                        $repoName = $matches[2]
                    }
                }
            } catch {
                # Use defaults if git config fails
            }
        }

        $response = Invoke-RestMethod -Uri "https://api.github.com/repos/$repoOwner/$repoName/releases/latest" -ErrorAction Stop
        $version = $response.tag_name
        Write-Info "Version: $version"
        return @{ Version = $version; Owner = $repoOwner; Name = $repoName }
    }
    catch {
        Write-Warning "Failed to fetch version from GitHub API, using latest"
        return @{ Version = "latest"; Owner = $repoOwner; Name = $repoName }
    }
}

# Compare version strings
function Compare-Versions {
    param(
        [string]$Version1,
        [string]$Version2
    )

    # Remove 'v' prefix if present
    $v1 = $Version1 -replace '^v', ''
    $v2 = $Version2 -replace '^v', ''

    if ($v1 -eq $v2) {
        return 0
    }

    # Split into parts
    $v1Parts = $v1 -split '\.'
    $v2Parts = $v2 -split '\.'

    # Compare major, minor, patch
    for ($i = 0; $i -lt 3; $i++) {
        $part1 = if ($i -lt $v1Parts.Length) { [int]$v1Parts[$i] } else { 0 }
        $part2 = if ($i -lt $v2Parts.Length) { [int]$v2Parts[$i] } else { 0 }

        if ($part1 -gt $part2) { return 1 }
        if ($part1 -lt $part2) { return 2 }
    }

    return 0
}

# Detect platform and architecture
function Get-PlatformArch {
    $arch = $env:PROCESSOR_ARCHITECTURE
    switch ($arch) {
        "AMD64" { return "windows-x86_64" }
        "ARM64" { return "windows-arm64" }
        default {
            Write-Error "Unsupported architecture: $arch"
            exit 1
        }
    }
}

# Download and install/upgrade Infra
function Download-AndInstall {
    param(
        [string]$Version,
        [string]$Mode
    )

    $platformArch = Get-PlatformArch
    $platform = ($platformArch -split '-')[0]

    Write-Info "Detected platform: $platformArch"

    # Construct download URL
    $binaryName = "infra-$platformArch"
    $baseUrl = "https://github.com/$script:REPO_OWNER/$script:REPO_NAME/releases"

    if ($Version -eq "latest") {
        $downloadUrl = "$baseUrl/latest/download/$binaryName"
    } else {
        $downloadUrl = "$baseUrl/download/$Version/$binaryName"
    }

    Write-Info "Downloading Infra $Version for $platformArch from: $downloadUrl"

    $binDir = Join-Path $InstallPath "bin"

    # Create directories
    Write-Info "Creating installation directories..."
    New-Item -ItemType Directory -Force -Path $binDir | Out-Null

    try {
        # Download binary
        $outputPath = Join-Path $binDir "infra.exe"
        Invoke-WebRequest -Uri $downloadUrl -OutFile $outputPath -ErrorAction Stop

        # Verify download
        if (!(Test-Path $outputPath)) {
            throw "Download failed - file not found"
        }

        Write-Success "Download completed successfully"
    }
    catch {
        Write-Error "Failed to download Infra: $($_.Exception.Message)"
        exit 1
    }

    # Add to PATH
    Add-ToPath $binDir

    Write-Success "Infra $Version installed successfully!"
    Write-Info "Binary location: $outputPath"

    if ($Mode -eq "upgrade") {
        Write-Info "Upgrade completed!"
    }
}

# Add to PATH
function Add-ToPath($binDir) {
    # Get current user PATH
    $userPath = [Environment]::GetEnvironmentVariable("PATH", "User")

    # Check if already in PATH
    if ($userPath -like "*$binDir*") {
        Write-Info "Infra directory is already in PATH"
        return
    }

    # Add to PATH
    Write-Info "Adding Infra to PATH..."
    $newPath = "$userPath;$binDir"
    [Environment]::SetEnvironmentVariable("PATH", $newPath, "User")

    # Also add to current session
    $env:PATH = "$env:PATH;$binDir"
}

# Show help information
function Show-Help {
    Write-Host "Infra Programming Language Installer for Windows" -ForegroundColor Cyan
    Write-Host "===============================================" -ForegroundColor Cyan
    Write-Host
    Write-Host "Usage: .\install.ps1 [OPTIONS]"
    Write-Host
    Write-Host "Options:"
    Write-Host "  -Upgrade         Upgrade existing installation to latest version"
    Write-Host "  -Force           Force installation even if Infra is already installed"
    Write-Host "  -InstallPath     Specify custom installation path (default: ~\.infra)"
    Write-Host "  -Help            Show this help message"
    Write-Host
    Write-Host "Examples:"
    Write-Host "  .\install.ps1                    # Fresh installation"
    Write-Host "  .\install.ps1 -Upgrade           # Upgrade existing installation"
    Write-Host "  .\install.ps1 -Force             # Force reinstallation"
    Write-Host "  .\install.ps1 -InstallPath C:\Infra  # Custom installation path"
    Write-Host
}

# Check for existing installation
function Check-Existing {
    $infraCommand = Get-Command infra -ErrorAction SilentlyContinue
    if ($infraCommand -and !$Force) {
        try {
            $existingVersion = & $infraCommand.Path --version 2>$null
            Write-Warning "Infra is already installed (version: $existingVersion)"
            $response = Read-Host "Do you want to continue and overwrite the existing installation? (y/N)"
            if ($response -notmatch '^[Yy]') {
                Write-Info "Installation cancelled."
                exit 0
            }
        } catch {
            # Continue if version check fails
        }
    }
}

# Check prerequisites
function Test-Prerequisites {
    if (!(Get-Command Invoke-WebRequest -ErrorAction SilentlyContinue)) {
        Write-Error "Invoke-WebRequest is not available. Please use a newer version of PowerShell."
        exit 1
    }
}

# Main installation flow
function Main {
    Write-Host "🚀 Infra Programming Language Installer v1.0.0 for Windows" -ForegroundColor Cyan
    Write-Host "============================================================" -ForegroundColor Cyan
    Write-Host

    # Show help if requested
    if ($Help) {
        Show-Help
        return
    }

    # Create install directory
    New-Item -ItemType Directory -Force -Path $InstallPath | Out-Null

    if ($script:UPGRADE_MODE) {
        # Upgrade mode
        if (Test-InfraInstalled) {
            $installedVersion = Get-InstalledVersion
            $versionInfo = Get-LatestVersion
            $latestVersion = $versionInfo.Version

            # Compare versions
            $comparison = Compare-Versions -Version1 $installedVersion -Version2 $latestVersion

            if ($comparison -eq 0 -or $installedVersion -eq "latest") {
                Write-Success "Infra is already up to date ($installedVersion)"
                exit 0
            } else {
                Write-Info "Upgrading Infra from $installedVersion to $latestVersion..."
                Download-AndInstall -Version $latestVersion -Mode "upgrade"
            }
        } else {
            Write-Error "Infra is not installed. Run without -Upgrade flag for fresh installation."
            exit 1
        }
    } else {
        # Fresh installation
        if (Test-InfraInstalled -and !$Force) {
            $installedVersion = Get-InstalledVersion
            Write-Warning "Infra is already installed ($installedVersion)"
            Write-Info "Use -Upgrade flag to upgrade to the latest version"
            exit 0
        } else {
            $versionInfo = Get-LatestVersion
            $latestVersion = $versionInfo.Version
            Write-Info "Installing Infra $latestVersion..."
            Download-AndInstall -Version $latestVersion -Mode "install"
        }
    }

    Write-Host
    Write-Success "Installation completed! 🎉"
    Write-Host
    Write-Info "Next steps:"
    Write-Host "  1. Restart PowerShell or open a new terminal"
    Write-Host "  2. Verify installation: infra --version"
    Write-Host "  3. Try the REPL: infra --repl"
    Write-Host "  4. Read the docs: https://github.com/$script:REPO_OWNER/$script:REPO_NAME#readme"
    Write-Host
}

# Run the installer
try {
    Main
}
catch {
    Write-Error "Installation failed: $($_.Exception.Message)"
    exit 1
}
