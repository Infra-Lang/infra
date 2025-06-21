# Infra Programming Language v0.1.0 - Binary Distribution

## 📦 What's Included

- `infra-linux` - Linux x86_64 binary (1.0 MB)
- `infra-windows.exe` - Windows x86_64 binary (1.7 MB)

## 🚀 Quick Installation

### Linux
```bash
# Download and install
curl -L -o infra-linux https://github.com/infra-lang/infra/releases/latest/download/infra-linux
chmod +x infra-linux
sudo mv infra-linux /usr/local/bin/infra

# Test installation
infra --version
```

### Windows
1. Download `infra-windows.exe`
2. Rename it to `infra.exe` 
3. Add to your PATH or place in a folder already in PATH
4. Test: open Command Prompt and run `infra --version`

### VS Code Extension
Install the companion VS Code extension for full IDE support:
```bash
code --install-extension MdAshiquzzamanKhan.infra-lang-support
```

## ✅ Quick Test

Create `hello.if`:
```infra
let message = "Hello, Infra!"
print(message)
```

Run it:
```bash
infra hello.if
```

Expected output:
```
Hello, Infra!
```

## 🛠️ Build Information

- **Version**: v0.1.0
- **Built with**: Rust 1.82+ 
- **Target architectures**: 
  - Linux: x86_64-unknown-linux-gnu
  - Windows: x86_64-pc-windows-gnu
- **Features**: Tree-walking interpreter, full type system, standard library

## 📚 Next Steps

1. **Install VS Code Extension** - Get syntax highlighting, IntelliSense, and error detection
2. **Read Documentation** - Learn the language syntax and features
3. **Try Examples** - Explore the example programs in the repository

## 🐛 Issues or Questions?

- GitHub Issues: https://github.com/infra-lang/infra/issues
- Documentation: https://github.com/infra-lang/infra/wiki

---

**Note**: This is a development release. For production use, consider compiling with specific optimizations for your target environment.
