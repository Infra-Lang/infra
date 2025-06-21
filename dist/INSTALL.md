# Infra Programming Language - Quick Install Commands

## 🚀 One-Line Installation

### Linux/macOS
```bash
curl -fsSL https://github.com/infra-lang/infra/releases/latest/download/install-linux.sh | bash
```

### Windows (PowerShell as Administrator)
```powershell
iex ((New-Object System.Net.WebClient).DownloadString('https://github.com/infra-lang/infra/releases/latest/download/install-windows.ps1'))
```

## 📋 Manual Installation

### Linux
```bash
# Download installer script
curl -L -o install-linux.sh https://github.com/infra-lang/infra/releases/latest/download/install-linux.sh
chmod +x install-linux.sh
./install-linux.sh
```

### Windows
```powershell
# Download installer script
Invoke-WebRequest -Uri https://github.com/infra-lang/infra/releases/latest/download/install-windows.ps1 -OutFile install-windows.ps1
.\install-windows.ps1
```

## ✅ Verify Installation

After installation, test that everything works:

```bash
# Check version
infra --version

# Run example
curl -L -o hello.if https://github.com/infra-lang/infra/releases/latest/download/hello.if
infra hello.if

# Install VS Code extension
code --install-extension MdAshiquzzamanKhan.infra-lang-support
```

## 🎯 What Gets Installed

- ✅ `infra` command available globally in terminal
- ✅ Binary added to system PATH
- ✅ Ready to run `.if` files from any directory
- ✅ Compatible with VS Code extension for full IDE support

---

**Next:** Install the VS Code extension and start coding! 🚀
