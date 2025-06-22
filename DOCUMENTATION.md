# Infra Programming Language - Complete Documentation

## Table of Contents
- [Introduction](#introduction)
- [Installation](#installation)
- [Language Basics](#language-basics)
- [Data Types](#data-types)
- [Variables and Constants](#variables-and-constants)
- [Operators](#operators)
- [Control Flow](#control-flow)
- [Functions](#functions)
- [Arrays](#arrays)
- [Objects](#objects)
- [Type System](#type-system)
- [Standard Library](#standard-library)
- [Module System](#module-system)
- [Error Handling](#error-handling)
- [File Extensions](#file-extensions)
- [Examples](#examples)
- [Command Line Interface](#command-line-interface)
- [VS Code Integration](#vs-code-integration)

---

## Introduction

**Infra** is a modern, Python-like programming language designed for performance and competitive programming. It combines the simplicity of Python with static typing capabilities and efficient execution.

### Key Features
- **Simple Syntax**: Python-like syntax that's easy to learn and read
- **Static Typing**: Optional type annotations for better code quality
- **Performance**: Built in Rust for fast execution
- **Rich Standard Library**: Built-in modules for math, strings, arrays, and I/O
- **Modern Features**: Error handling, module system, and advanced data structures
- **IDE Support**: Full VS Code integration with syntax highlighting and IntelliSense

---

## Installation

### Quick Install

**Linux/macOS:**
```bash
curl -fsSL https://github.com/infra-lang/infra/releases/latest/download/install-linux.sh | bash
```

**Windows (PowerShell as Administrator):**
```powershell
iex ((New-Object System.Net.WebClient).DownloadString('https://github.com/infra-lang/infra/releases/latest/download/install-windows.ps1'))
```

### Verify Installation
```bash
infra --version
```

### VS Code Extension
```bash
code --install-extension MdAshiquzzamanKhan.infra-lang-support
```

---

## Language Basics

### Hello World
```infra
print("Hello, World!")
```

### Comments
```infra
# This is a single-line comment (Python style)
// This is also a single-line comment (C/JavaScript style)

# Both comment styles work in Infra
// You can use whichever style you prefer

# Multi-line comments are achieved with multiple comment lines
// Or multiple // lines like this
// Each line needs its own comment marker
```

### Running Programs
```bash
# Save your code in a .if file
echo 'print("Hello!")' > hello.if

# Run with infra
infra hello.if
```

---

## Data Types

Infra supports several built-in data types:

### Numbers
```infra
let integer = 42
let float = 3.14159
let negative = -10
```

### Strings
```infra
let name = "Alice"
let message = 'Hello, World!'
let multiline = "This is a
multi-line string"
```

### Booleans
```infra
let is_valid = true
let is_complete = false
```

### Null
```infra
let empty_value = null
```

### Arrays
```infra
let numbers = [1, 2, 3, 4, 5]
let mixed = [1, "hello", true, null]
let nested = [[1, 2], [3, 4]]
```

### Objects
```infra
let person = {
    name: "Alice",
    age: 30,
    is_student: false
}
```

---

## Variables and Constants

### Variable Declaration
```infra
# Basic variable declaration
let x = 10
let name = "Alice"

# Variables can be reassigned
x = 20
name = "Bob"
```

### Type Annotations (Optional)
```infra
# Explicit type annotations
let age: number = 25
let username: string = "alice_dev"
let is_active: boolean = true
let scores: [number] = [95, 87, 92]
```

### Variable Scope
```infra
let global_var = "I'm global"

{
    let local_var = "I'm local"
    print(global_var)  # Accessible
    print(local_var)   # Accessible
}

# print(local_var)   # Error: not accessible
print(global_var)    # Accessible
```

---

## Operators

### Arithmetic Operators
```infra
let a = 10
let b = 3

print(a + b)    # Addition: 13
print(a - b)    # Subtraction: 7
print(a * b)    # Multiplication: 30
print(a / b)    # Division: 3.333...
print(a % b)    # Modulo: 1
```

### Comparison Operators
```infra
let x = 5
let y = 10

print(x == y)   # Equal: false
print(x != y)   # Not equal: true
print(x < y)    # Less than: true
print(x <= y)   # Less than or equal: true
print(x > y)    # Greater than: false
print(x >= y)   # Greater than or equal: false
```

### Logical Operators
```infra
let a = true
let b = false

print(a and b)  # Logical AND: false
print(a or b)   # Logical OR: true
print(!a)       # Logical NOT: false
```

### Assignment Operators
```infra
let x = 10
x = x + 5       # Basic assignment
# Note: Compound assignment operators (+=, -=, etc.) may be added in future versions
```

---

## Control Flow

### If Statements
```infra
let age = 18

if age >= 18 {
    print("You are an adult")
} else {
    print("You are a minor")
}

# Multi-condition if-else
let score = 85

if score >= 90 {
    print("Grade: A")
} else if score >= 80 {
    print("Grade: B")
} else if score >= 70 {
    print("Grade: C")
} else {
    print("Grade: F")
}
```

### While Loops
```infra
let count = 0

while count < 5 {
    print("Count: " + count)
    count = count + 1
}
```

### For Loops
```infra
# For loop with range
for i in 0..5 {
    print("Iteration: " + i)
}

# For loop with array
let fruits = ["apple", "banana", "orange"]
for fruit in fruits {
    print("Fruit: " + fruit)
}
```

### Block Statements
```infra
{
    let local_variable = "This is local"
    print(local_variable)
    
    {
        let nested_variable = "This is nested"
        print(nested_variable)
    }
}
```

---

## Functions

### Basic Function Declaration
```infra
function greet(name):
    print("Hello, " + name + "!")

# Alternative syntax
def greet(name):
    print("Hello, " + name + "!")

# Call the function
greet("Alice")
```

### Functions with Return Values
```infra
function add(a, b):
    return a + b

let result = add(5, 3)
print(result)  # 8
```

### Functions with Type Annotations
```infra
function multiply(a: number, b: number) -> number:
    return a * b

function format_name(first: string, last: string) -> string:
    return first + " " + last
```

### Functions with Default Parameters
```infra
function greet(name: string, greeting: string = "Hello") -> string {
    return greeting + ", " + name + "!"
}

print(greet("Alice"))              # "Hello, Alice!"
print(greet("Bob", "Hi"))          # "Hi, Bob!"
```

### Anonymous Functions (Lambda)
```infra
let square = function(x) {
    return x * x
}

print(square(5))  # 25
```

---

## Arrays

### Array Creation and Access
```infra
let numbers = [1, 2, 3, 4, 5]
let fruits = ["apple", "banana", "orange"]

# Access elements
print(numbers[0])    # 1
print(fruits[1])     # "banana"

# Modify elements
numbers[0] = 10
print(numbers)       # [10, 2, 3, 4, 5]
```

### Array Methods (via Standard Library)
```infra
import { length, push, pop, sort, reverse } from "array"

let numbers = [3, 1, 4, 1, 5]

print(array.length(numbers))    # 5
array.push(numbers, 9)          # [3, 1, 4, 1, 5, 9]
let last = array.pop(numbers)   # Returns 9, array becomes [3, 1, 4, 1, 5]
array.sort(numbers)             # [1, 1, 3, 4, 5]
array.reverse(numbers)          # [5, 4, 3, 1, 1]
```

### Multi-dimensional Arrays
```infra
let matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]

print(matrix[1][2])  # 6
```

---

## Objects

### Object Creation and Access
```infra
let person = {
    name: "Alice",
    age: 30,
    city: "New York"
}

# Access properties
print(person.name)      # "Alice"
print(person["age"])    # 30

# Modify properties
person.age = 31
person["city"] = "Boston"
```

### Nested Objects
```infra
let user = {
    personal: {
        name: "Alice",
        age: 30
    },
    contact: {
        email: "alice@example.com",
        phone: "+1-555-0123"
    }
}

print(user.personal.name)        # "Alice"
print(user["contact"]["email"])  # "alice@example.com"
```

### Object Methods
```infra
let calculator = {
    add: function(a, b) {
        return a + b
    },
    multiply: function(a, b) {
        return a * b
    }
}

print(calculator.add(5, 3))      # 8
print(calculator.multiply(4, 7)) # 28
```

---

## Type System

Infra supports both dynamic typing (like Python) and optional static typing (like TypeScript).

### Basic Types
```infra
let name: string = "Alice"
let age: number = 30
let is_student: boolean = true
let data: any = "can be anything"
```

### Array Types
```infra
let numbers: [number] = [1, 2, 3, 4, 5]
let names: [string] = ["Alice", "Bob", "Charlie"]
let mixed: [any] = [1, "hello", true]
```

### Object Types
```infra
let person: {name: string, age: number} = {
    name: "Alice",
    age: 30
}
```

### Function Types
```infra
let operation: (number, number) -> number = function(a, b) {
    return a + b
}
```

### Union Types
```infra
let id: number | string = 123
id = "ABC123"  # Valid

function process(input: string | number) -> string {
    if typeof(input) == "string" {
        return "String: " + input
    } else {
        return "Number: " + input
    }
}
```

---

## Standard Library

### Math Module
```infra
import math

print(math.sqrt(16))        # 4
print(math.abs(-10))        # 10
print(math.max(5, 10, 3))   # 10
print(math.min(5, 10, 3))   # 3
print(math.pow(2, 3))       # 8
print(math.floor(3.7))      # 3
print(math.ceil(3.2))       # 4
print(math.round(3.6))      # 4
```

### String Module
```infra
import string

let text = "Hello, World!"

print(string.length(text))              # 13
print(string.upper(text))               # "HELLO, WORLD!"
print(string.lower(text))               # "hello, world!"
print(string.split(text, ", "))         # ["Hello", "World!"]
print(string.contains(text, "World"))   # true
print(string.substring(text, 0, 5))     # "Hello"
print(string.replace(text, "World", "Infra"))  # "Hello, Infra!"
print(string.starts_with(text, "Hello"))       # true
print(string.ends_with(text, "!"))             # true
print(string.repeat("Hi ", 3))                 # "Hi Hi Hi "
print(string.trim("  hello  "))               # "hello"
```

### Array Module
```infra
import array

let numbers = [3, 1, 4, 1, 5, 9]

print(array.length(numbers))           # 6
array.push(numbers, 2)                 # [3, 1, 4, 1, 5, 9, 2]
let last = array.pop(numbers)          # Returns 2
array.sort(numbers)                    # [1, 1, 3, 4, 5, 9]
array.reverse(numbers)                 # [9, 5, 4, 3, 1, 1]
print(array.join(numbers, ", "))       # "9, 5, 4, 3, 1, 1"
```

### I/O Module
```infra
import io

# Read from file
let content = io.read_file("input.txt")
print(content)

# Write to file
io.write_file("output.txt", "Hello, File!")

# Append to file
io.append_file("log.txt", "New log entry\n")

# Check if file exists
if io.file_exists("config.txt") {
    print("Config file found")
}
```

---

## Module System

### Importing Modules
```infra
# Import entire module
import math
print(math.sqrt(16))

# Import specific functions
import { sqrt, pow } from math
print(sqrt(16))
print(pow(2, 3))

# Import with alias
import math as m
print(m.sqrt(16))

# Import specific functions with alias
import { sqrt as square_root } from math
print(square_root(16))
```

### Creating Custom Modules

**math_utils.if:**
```infra
export function factorial(n: number) -> number {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

export function fibonacci(n: number) -> number {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

export let PI = 3.14159
```

**main.if:**
```infra
import { factorial, fibonacci, PI } from "math_utils"

print(factorial(5))    # 120
print(fibonacci(10))   # 55
print(PI)              # 3.14159
```

---

## Error Handling

### Try-Catch Statements
```infra
try {
    let result = 10 / 0
    print(result)
} catch error {
    print("An error occurred: " + error)
}
```

### Function Error Handling
```infra
function safe_divide(a: number, b: number) -> number | null {
    if b == 0 {
        return null
    }
    return a / b
}

let result = safe_divide(10, 2)
if result != null {
    print("Result: " + result)
} else {
    print("Division by zero!")
}
```

---

## File Extensions

Infra supports two file extensions:

- **`.if`** - Standard Infra source files
- **`.infra`** - Alternative extension for Infra source files

Both extensions work identically:
```bash
infra hello.if
infra hello.infra
```

---

## Examples

### Example 1: Calculator
```infra
function calculator() {
    print("Simple Calculator")
    print("1. Add")
    print("2. Subtract")
    print("3. Multiply")
    print("4. Divide")
    
    let choice = input("Enter choice (1-4): ")
    let a = input("Enter first number: ")
    let b = input("Enter second number: ")
    
    if choice == "1" {
        print("Result: " + (a + b))
    } else if choice == "2" {
        print("Result: " + (a - b))
    } else if choice == "3" {
        print("Result: " + (a * b))
    } else if choice == "4" {
        if b != 0 {
            print("Result: " + (a / b))
        } else {
            print("Error: Division by zero!")
        }
    } else {
        print("Invalid choice!")
    }
}

calculator()
```

### Example 2: Array Processing
```infra
import array
import math

function analyze_numbers(numbers: [number]) -> object {
    let sum = 0
    let max_val = numbers[0]
    let min_val = numbers[0]
    
    for num in numbers {
        sum = sum + num
        if num > max_val {
            max_val = num
        }
        if num < min_val {
            min_val = num
        }
    }
    
    let average = sum / array.length(numbers)
    
    return {
        sum: sum,
        average: average,
        max: max_val,
        min: min_val,
        count: array.length(numbers)
    }
}

let data = [85, 92, 78, 96, 87, 91, 89]
let stats = analyze_numbers(data)

print("Statistics:")
print("Count: " + stats.count)
print("Sum: " + stats.sum)
print("Average: " + stats.average)
print("Max: " + stats.max)
print("Min: " + stats.min)
```

### Example 3: Object-Oriented Style
```infra
function create_person(name: string, age: number) -> object {
    return {
        name: name,
        age: age,
        
        greet: function() {
            print("Hello, I'm " + this.name)
        },
        
        birthday: function() {
            this.age = this.age + 1
            print(this.name + " is now " + this.age + " years old")
        },
        
        info: function() -> string {
            return this.name + " (" + this.age + " years old)"
        }
    }
}

let alice = create_person("Alice", 25)
let bob = create_person("Bob", 30)

alice.greet()      # "Hello, I'm Alice"
alice.birthday()   # "Alice is now 26 years old"
print(alice.info()) # "Alice (26 years old)"
```

---

## Command Line Interface

### Basic Usage
```bash
# Run a file
infra script.if

# Interactive REPL
infra

# Show version
infra --version

# Show help
infra --help
```

### REPL (Interactive Mode)
```bash
$ infra
Infra Programming Language REPL
Type 'exit' to quit.

> let x = 10
> let y = 20
> print(x + y)
30
> 
```

---

## VS Code Integration

The Infra VS Code extension provides:

### Features
- **Syntax Highlighting**: Full syntax highlighting for `.if` and `.infra` files
- **IntelliSense**: Auto-completion for keywords, functions, and variables
- **Error Detection**: Real-time error highlighting
- **Code Formatting**: Automatic code formatting
- **Snippets**: Code snippets for common patterns
- **Language Server**: Full language server support

### Installation
```bash
code --install-extension MdAshiquzzamanKhan.infra-lang-support
```

### Configuration
Add to your VS Code `settings.json`:
```json
{
    "infra.enableLanguageServer": true,
    "infra.formatOnSave": true,
    "infra.linting.enabled": true
}
```

---

## Best Practices

### Code Style
```infra
# Use meaningful variable names
let user_age = 25  # Good
let a = 25         # Bad

# Use type annotations for function parameters and return types
function calculate_area(width: number, height: number) -> number {
    return width * height
}

# Use consistent indentation (4 spaces recommended)
if condition {
    do_something()
    if nested_condition {
        do_nested_action()
    }
}
```

### Performance Tips
```infra
# Prefer local variables in loops
let items = [1, 2, 3, 4, 5]
let length = array.length(items)  # Calculate once

for i in 0..length {
    process(items[i])
}

# Use appropriate data structures
let lookup = {}  # For key-value pairs
let list = []    # For ordered collections
```

### Error Handling
```infra
# Always handle potential errors
try {
    let result = risky_operation()
    process_result(result)
} catch error {
    print("Error: " + error)
    use_fallback()
}
```

---

This comprehensive documentation covers all aspects of the Infra programming language. For more examples and advanced usage, check the official repository at https://github.com/infra-lang/infra.
