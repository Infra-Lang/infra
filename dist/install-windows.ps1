# Infra Programming Language - Windows Installation Script
# Run this in PowerShell as Administrator

Write-Host "🚀 Installing Infra Programming Language..." -ForegroundColor Green

# Check if running as Administrator
if (-NOT ([Security.Principal.WindowsPrincipal] [Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole] "Administrator")) {
    Write-Host "❌ Please run PowerShell as Administrator" -ForegroundColor Red
    exit 1
}

# Create installation directory
$installDir = "C:\Program Files\Infra"
if (!(Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

# Download binary
Write-Host "📦 Downloading infra-windows.exe..." -ForegroundColor Yellow
$url = "https://github.com/infra-lang/infra/releases/latest/download/infra-windows.exe"
$output = "$installDir\infra.exe"

try {
    Invoke-WebRequest -Uri $url -OutFile $output
    Write-Host "✅ Downloaded successfully" -ForegroundColor Green
} catch {
    Write-Host "❌ Download failed: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

# Add to PATH
Write-Host "📁 Adding to system PATH..." -ForegroundColor Yellow
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "Machine")
if ($currentPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$installDir", "Machine")
    Write-Host "✅ Added to PATH" -ForegroundColor Green
} else {
    Write-Host "ℹ️ Already in PATH" -ForegroundColor Cyan
}

# Verify installation
Write-Host "✅ Testing installation..." -ForegroundColor Yellow
try {
    $version = & "$output" --version
    Write-Host "🎉 Infra installed successfully!" -ForegroundColor Green
    Write-Host "Version: $version" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Next steps:" -ForegroundColor Yellow
    Write-Host "1. Restart your command prompt/PowerShell"
    Write-Host "2. Install VS Code extension: code --install-extension MdAshiquzzamanKhan.infra-lang-support"
    Write-Host "3. Create a hello.if file and run: infra hello.if"
} catch {
    Write-Host "❌ Installation verification failed: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}
