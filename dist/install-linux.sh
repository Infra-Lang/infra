#!/bin/bash
# Infra Programming Language - Linux Installation Script

set -e

echo "🚀 Installing Infra Programming Language..."

# Check if running as root
if [[ $EUID -eq 0 ]]; then
   echo "❌ Please don't run this script as root"
   exit 1
fi

# Download binary
echo "📦 Downloading infra-linux..."
curl -L -o /tmp/infra-linux https://github.com/infra-lang/infra/releases/latest/download/infra-linux

# Make executable
chmod +x /tmp/infra-linux

# Install to /usr/local/bin
echo "📁 Installing to /usr/local/bin/infra..."
sudo mv /tmp/infra-linux /usr/local/bin/infra

# Verify installation
echo "✅ Testing installation..."
if infra --version > /dev/null 2>&1; then
    echo "🎉 Infra installed successfully!"
    echo "Version: $(infra --version)"
    echo ""
    echo "Next steps:"
    echo "1. Install VS Code extension: code --install-extension MdAshiquzzamanKhan.infra-lang-support"
    echo "2. Create a hello.if file and run: infra hello.if"
else
    echo "❌ Installation failed. Please check the error messages above."
    exit 1
fi
