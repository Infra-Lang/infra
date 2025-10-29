# Infra Programming Language Installer for Windows
# This script downloads and installs the latest version of Infra

param(
    [switch]$Force,
    [string]$InstallPath = "$env:USERPROFILE\.infra"
)

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

# Get latest version from GitHub API
function Get-LatestVersion {
    Write-Info "Fetching latest version information..."

    try {
        # Try to get repo info from git config or use defaults
        $repoOwner = "infra-lang"
        $repoName = "infra"

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

# Download and install Infra
function Install-Infra {
    $arch = Get-Architecture
    $versionInfo = Get-LatestVersion
    $version = $versionInfo.Version
    $repoOwner = $versionInfo.Owner
    $repoName = $versionInfo.Name

    $binDir = Join-Path $InstallPath "bin"
    $binaryName = "infra-$arch"

    # Create directories
    Write-Info "Creating installation directories..."
    New-Item -ItemType Directory -Force -Path $binDir | Out-Null

    # Download URL
    $baseUrl = "https://github.com/$repoOwner/$repoName/releases"
    if ($version -eq "latest") {
        $downloadUrl = "$baseUrl/latest/download/$binaryName"
    } else {
        $downloadUrl = "$baseUrl/download/$version/$binaryName"
    }

    Write-Info "Downloading Infra from: $downloadUrl"

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

    Write-Success "Infra has been installed successfully!"
    Write-Info "Binary location: $outputPath"
    Write-Info "Run 'infra --version' to verify installation."
    Write-Info "You may need to restart PowerShell to use the 'infra' command."
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
    Write-Host "🚀 Infra Programming Language Installer" -ForegroundColor Cyan
    Write-Host "=======================================" -ForegroundColor Cyan
    Write-Host

    Check-Existing
    Test-Prerequisites
    Install-Infra

    Write-Host
    Write-Success "Installation completed! 🎉"
    Write-Host
    Write-Info "Next steps:"
    Write-Host "  1. Restart PowerShell or open a new terminal"
    Write-Host "  2. Verify installation: infra --version"
    Write-Host "  3. Try the REPL: infra --repl"
    Write-Host "  4. Read the docs: https://github.com/infra-lang/infra#readme"
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
