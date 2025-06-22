# 🚀 Infra Programming Language

A modern programming language with optional static typing, built for simplicity and performance.

## ⚡ Super Quick Start

### 1. Download the Compiler
- **Windows**: Download `infra-windows.exe` from [Releases](https://github.com/infra-lang/infra/releases/latest)
- **macOS**: Download `infra-macos` from [Releases](https://github.com/infra-lang/infra/releases/latest)  
- **Linux**: Download `infra-linux` from [Releases](https://github.com/infra-lang/infra/releases/latest)

### 2. Install VS Code Extension
1. Open VS Code
2. Go to Extensions (Ctrl+Shift+X)
3. Search "Infra Language Support" 
4. Click Install

### 3. Start Coding!
Create `hello.if`:
```infra
let name = "World"
print("Hello, " + name + "!")

function fibonacci(n: number) -> number:
    if n <= 1: return n
    return fibonacci(n-1) + fibonacci(n-2)

print("Fibonacci(10) = " + fibonacci(10))
```

Run it:
```bash
infra hello.if
```

**That's it! 🎉 You're ready to build with Infra!**

## ✨ Language Features

- **Optional Static Typing**: `let x: number = 42` or `let x = 42`
- **Modern Syntax**: Clean, readable code structure
- **Fast Execution**: Bytecode virtual machine for high performance
- **Rich Standard Library**: Math, String, Array, and I/O operations
- **Error Handling**: Built-in try/catch exception handling
- **Module System**: Import/export for code organization

## 🛠️ IDE Support

The VS Code extension provides:
- ✅ Syntax highlighting
- ✅ IntelliSense & autocomplete  
- ✅ Real-time error diagnostics
- ✅ Code formatting
- ✅ Hover documentation
- ✅ Go to definition

## 📚 Examples

### Variables & Types
```infra
let message: string = "Hello"
let count = 42                    // Type inferred as number
let items = [1, 2, 3, 4, 5]      // Type inferred as array<number>
```

### Functions
```infra
function greet(name: string, age?: number) -> string:
    if age:
        return "Hello " + name + ", you are " + age + " years old"
    return "Hello " + name

print(greet("Alice", 25))
print(greet("Bob"))
```

### Objects & Arrays
```infra
let person = {
    name: "Alice",
    age: 30,
    hobbies: ["reading", "coding"]
}

for hobby in person.hobbies:
    print("Hobby: " + hobby)
```

### Error Handling
```infra
function divide(a: number, b: number) -> number:
    if b == 0:
        throw "Division by zero"
    return a / b

try:
    let result = divide(10, 0)
    print(result)
catch error:
    print("Error: " + error)
```

## 🔧 Build from Source (Optional)

If you prefer to build from source:

```bash
git clone https://github.com/infra-lang/infra.git
cd infra
cargo build --release
```

The binary will be at `target/release/infra`.

## 📖 Documentation

For detailed documentation, installation guides, and examples, see:
- **[Complete Language Documentation](DOCUMENTATION.md)** - Comprehensive guide covering all language features
- **[Quick Reference](QUICK_REFERENCE.md)** - Syntax cheat sheet and common patterns
- **[Installation Guide](dist/INSTALL.md)** - Installation instructions for all platforms
- **[Basic Examples](examples.if)** - Core language features and syntax
- **[Advanced Examples](advanced_examples.if)** - Complex patterns and algorithms

## 🤝 Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📄 License

This project is licensed under the MIT License - see [LICENSE](LICENSE) for details.

---

**Ready to try Infra?** [Download now](https://github.com/infra-lang/infra/releases/latest) and start coding! 🚀
