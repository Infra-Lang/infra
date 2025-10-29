# Infra Programming Language Guide

Welcome to Infra, a modern, Python-like programming language built in Rust that combines simplicity with performance. Infra features optional static typing, clean syntax, and supports both direct AST interpretation and bytecode compilation for high performance execution.

## Table of Contents

- [Quick Start](#quick-start)
- [Basic Syntax](#basic-syntax)
- [Data Types](#data-types)
- [Variables and Constants](#variables-and-constants)
- [Operators](#operators)
- [Control Flow](#control-flow)
- [Functions](#functions)
- [Object-Oriented Programming](#object-oriented-programming)
- [Async/Await Programming](#asyncawait-programming)
- [Error Handling](#error-handling)
- [Modules and Imports](#modules-and-imports)
- [Standard Library](#standard-library)
- [Type System](#type-system)
- [Advanced Features](#advanced-features)

## Quick Start

### Installation

```bash
# From source
git clone https://github.com/infra-lang/infra.git
cd infra
cargo build --release
cargo install --path .

# Or download pre-built binaries from releases
```

### Your First Program

Create a file `hello.if`:

```infra
// hello.if
print("Hello, World!")

// Variables and basic operations
let name = "Infra"
let version = 0.1
print(f"Welcome to {name} v{version}")

// Functions
function greet(person: string): string {
    return f"Hello, {person}!"
}

print(greet("World"))
```

Run it:

```bash
infra hello.if
```

Or use the interactive REPL:

```bash
infra --repl
```

## Basic Syntax

Infra's syntax is clean and similar to Python, with optional type annotations:

```infra
// Comments start with //
let x = 42           // Variable declaration
let y: number = 3.14 // With type annotation

const PI = 3.14159  // Constant declaration

// Multiple assignment
let a, b = 1, 2

// Semicolons are optional
let c = 3; let d = 4
```

### Code Blocks

Code blocks use indentation (like Python):

```infra
function example():
    if true:
        print("This is true")
        let nested = 42
    return nested
```

## Data Types

### Primitive Types

```infra
// Numbers
let integer = 42
let float = 3.14159

// Strings
let text = "Hello, World!"
let interpolated = f"The answer is {integer}"

// Booleans
let is_true = true
let is_false = false

// Nil (null/none)
let nothing = nil
```

### Collections

```infra
// Arrays
let numbers = [1, 2, 3, 4, 5]
let mixed = [1, "hello", true, nil]

// Objects (dictionaries)
let person = {
    "name": "Alice",
    "age": 30,
    "city": "New York"
}

// Access elements
print(numbers[0])        // 1
print(person["name"])    // "Alice"

// Array methods
numbers.push(6)
print(numbers.length())  // 6
```

## Variables and Constants

### Variable Declaration

```infra
let x = 42           // Mutable variable
let y: string = "hello"  // With explicit type

// Assignment
x = x + 1
y = y + " world"
```

### Constants

```infra
const PI = 3.14159  // Cannot be changed
const APP_NAME = "MyApp"  // Constant naming convention
```

### Type Inference

```infra
let num = 42        // Inferred as number
let txt = "hello"   // Inferred as string
let flag = true     // Inferred as boolean
```

## Operators

### Arithmetic Operators

```infra
let a = 10, b = 3

print(a + b)    // 13 (addition)
print(a - b)    // 7  (subtraction)
print(a * b)    // 30 (multiplication)
print(a / b)    // 3.333... (division)
print(a % b)    // 1  (modulo)
print(a ** b)   // 1000 (exponentiation)
```

### Comparison Operators

```infra
let x = 5, y = 10

print(x == y)   // false (equal)
print(x != y)   // true  (not equal)
print(x < y)    // true  (less than)
print(x <= y)   // true  (less than or equal)
print(x > y)    // false (greater than)
print(x >= y)   // false (greater than or equal)
```

### Logical Operators

```infra
let a = true, b = false

print(a and b)  // false (logical and)
print(a or b)   // true  (logical or)
print(not a)    // false (logical not)
```

### String Operations

```infra
let s1 = "Hello", s2 = "World"

print(s1 + " " + s2)           // "Hello World"
print(f"{s1}, {s2}!")         // "Hello, World!" (interpolation)
print(s1.length())            // 5
print(s1.upper())             // "HELLO"
print(s1.lower())             // "hello"
print(s1 * 3)                 // "HelloHelloHello"
```

## Control Flow

### If Statements

```infra
let age = 18

if age < 13:
    print("Child")
elif age < 18:
    print("Teenager")
elif age >= 18:
    print("Adult")
else:
    print("Unknown")
```

### Ternary Operator

```infra
let age = 20
let status = age >= 18 ? "Adult" : "Minor"
print(status)  // "Adult"
```

### Loops

#### While Loop

```infra
let i = 0
while i < 5:
    print(i)
    i = i + 1
```

#### For Loop

```infra
// Range loop
for i in 0..5:
    print(i)  // 0, 1, 2, 3, 4

// Array iteration
let fruits = ["apple", "banana", "orange"]
for fruit in fruits:
    print(fruit)

// With index
for i, fruit in fruits.enumerate():
    print(f"{i}: {fruit}")
```

#### Break and Continue

```infra
for i in 0..10:
    if i == 3:
        continue  // Skip 3
    if i == 7:
        break     // Stop at 7
    print(i)
```

## Functions

### Basic Functions

```infra
function greet(name: string): string {
    return "Hello, " + name
}

// Call the function
print(greet("Alice"))  // "Hello, Alice"
```

### Parameters and Return Types

```infra
// With type annotations
function add(a: number, b: number): number {
    return a + b
}

// Without return type (inferred)
function multiply(a, b) {
    return a * b
}

// No return value
function log(message: string): void {
    print("LOG:", message)
}
```

### Default Parameters

```infra
function greet(name: string = "World"): string {
    return "Hello, " + name
}

print(greet())        // "Hello, World"
print(greet("Alice")) // "Hello, Alice"
```

### Variable Arguments

```infra
function sum(...numbers: number[]): number {
    let total = 0
    for num in numbers:
        total = total + num
    return total
}

print(sum(1, 2, 3, 4, 5))  // 15
```

### Function Expressions

```infra
// Anonymous function
let add = function(a, b): a + b

// Arrow function (short syntax)
let multiply = (a, b) => a * b

print(add(2, 3))        // 5
print(multiply(4, 5))   // 20
```

### Higher-Order Functions

```infra
function apply_twice(f, x):
    return f(f(x))

let add_one = x => x + 1
print(apply_twice(add_one, 5))  // 7
```

## Object-Oriented Programming

### Class Declaration

```infra
class Person:
    // Constructor
    function init(name: string, age: number):
        this.name = name
        this.age = age
    
    // Method
    function greet(): string {
        return f"Hello, I'm {this.name} and I'm {this.age} years old"
    }
    
    // Property access method
    function get_age(): number {
        return this.age
    }
    
    function set_age(new_age: number):
        this.age = new_age
```

### Creating Objects

```infra
// Create instance
let person = Person("Alice", 30)

// Access properties
print(person.name)  // "Alice"
print(person.age)   // 30

// Call methods
print(person.greet())  // "Hello, I'm Alice and I'm 30 years old"
person.set_age(31)
print(person.get_age())  // 31
```

### Inheritance

```infra
class Student extends Person:
    function init(name: string, age: number, major: string):
        super.init(name, age)  // Call parent constructor
        this.major = major
    
    // Override method
    function greet(): string {
        let base_greeting = super.greet()
        return base_greeting + f" and I study {this.major}"
    }
    
    // New method
    function study():
        print(f"{this.name} is studying {this.major}")
```

### Using Inheritance

```infra
let student = Student("Bob", 20, "Computer Science")

print(student.name)   // "Bob"
print(student.major)  // "Computer Science"
print(student.greet())  // "Hello, I'm Bob and I'm 20 years old and I study Computer Science"
student.study()         // "Bob is studying Computer Science"
```

### Static Methods and Properties

```infra
class Math:
    const PI = 3.14159
    
    static function circle_area(radius: number): number {
        return Math.PI * radius ** 2
    }
    
    static function max(a: number, b: number): number {
        return a > b ? a : b
    }
```

## Async/Await Programming

Infra supports modern async/await programming for handling asynchronous operations:

### Basic Async Functions

```infra
async function fetch_data(url: string): string {
    print("Fetching data...")
    // Simulate network delay
    await async.sleep(1000)
    return "Data from " + url
}

async function main():
    let data = await fetch_data("https://api.example.com")
    print(data)
```

### Running Async Code

```infra
// Using async main
async function main():
    let data = await fetch_data("https://api.example.com")
    print(data)

// Or create and run promise
let promise = fetch_data("https://api.example.com")
promise.then(data => print(data))
```

### Async File Operations

```infra
async function read_config(): object {
    let content = await async.read_file("config.json")
    return json.parse(content)
}

async function write_log(message: string):
    let timestamp = date.now()
    let log_entry = f"[{timestamp}] {message}\n"
    await async.write_file("app.log", log_entry, append=true)
```

### Concurrent Operations

```infra
async function fetch_multiple():
    let urls = [
        "https://api.example.com/users",
        "https://api.example.com/posts", 
        "https://api.example.com/comments"
    ]
    
    // Fetch all concurrently
    let promises = []
    for url in urls:
        promises.push(async.http_get(url))
    
    let results = await async.all(promises)
    print("All data fetched:", results)
```

### Async Error Handling

```infra
async function safe_fetch(url: string): string | null {
    try:
        let response = await async.http_get(url)
        return response
    except error:
        print("Failed to fetch:", error)
        return null
}
```

### Async Iteration

```infra
async function process_stream():
    let stream = async.open_file_stream("large_file.txt")
    
    async for line in stream:
        // Process each line
        if line.contains("ERROR"):
            print("Found error:", line)
```

## Error Handling

### Try-Catch Blocks

```infra
function divide(a, b):
    try:
        return a / b
    except error:
        print("Division failed:", error)
        return nil

print(divide(10, 2))  // 5
print(divide(10, 0))  // "Division failed: Division by zero", nil
```

### Multiple Exception Types

```infra
function process_file(filename):
    try:
        let content = io.read_file(filename)
        let data = json.parse(content)
        return data
    except IOError as error:
        print("File error:", error)
        return nil
    except ParseError as error:
        print("JSON error:", error)
        return nil
    except:
        print("Unknown error occurred")
        return nil
```

### Throwing Exceptions

```infra
function validate_age(age):
    if age < 0:
        throw ValueError("Age cannot be negative")
    if age > 150:
        throw ValueError("Age seems unrealistic")
    return age

try:
    let valid_age = validate_age(-5)
except ValueError as error:
    print("Validation error:", error)
```

### Finally Blocks

```infra
function process_with_cleanup():
    let resource = open_resource()
    try:
        // Use resource
        process(resource)
    except error:
        print("Processing failed:", error)
    finally:
        // Always cleanup
        resource.close()
```

## Modules and Imports

### Creating Modules

Create a file `math_utils.if`:

```infra
// math_utils.if
export function factorial(n: number): number {
    if n <= 1:
        return 1
    return n * factorial(n - 1)
}

export function fibonacci(n: number): number {
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
}

const GOLDEN_RATIO = 1.618033988749895
export GOLDEN_RATIO
```

### Importing Modules

```infra
// main.if
import math_utils

// Use imported functions
print(math_utils.factorial(5))     // 120
print(math_utils.fibonacci(10))   // 55
print(math_utils.GOLDEN_RATIO)    // 1.618...

// Or import specific items
import { factorial, fibonacci } from math_utils
print(factorial(6))   // 720
```

### Standard Library Imports

```infra
import math
import io
import async

print(math.sqrt(16))  // 4
print(math.pi)        // 3.14159...

// Import specific functions
import { sqrt, sin, cos } from math
```

### Module Aliases

```infra
import math_utils as math
import "long_module_name" as short_name

print(math.factorial(5))
```

## Standard Library

Infra comes with a comprehensive standard library organized into modules:

### Math Module

```infra
import math

// Basic operations
print(math.abs(-5))        // 5
print(math.max(1, 5, 3))   // 5
print(math.min(1, 5, 3))   // 1
print(math.round(3.7))     // 4
print(math.floor(3.7))     // 3
print(math.ceil(3.2))      // 4

// Trigonometry
print(math.sin(math.pi / 2))  // 1.0
print(math.cos(0))             // 1.0
print(math.tan(math.pi / 4))  // 1.0

// Powers and roots
print(math.sqrt(16))          // 4.0
print(math.pow(2, 3))         // 8.0
print(math.exp(1))            // 2.718...
print(math.log(math.e))       // 1.0

// Constants
print(math.pi)                // 3.14159...
print(math.e)                 // 2.71828...
```

### String Module

```infra
import string

// Case operations
print(string.upper("hello"))      // "HELLO"
print(string.lower("WORLD"))      // "world"
print(string.capitalize("hello")) // "Hello"

// String information
print(string.length("hello"))     // 5
print(string.is_empty(""))        // true
print(string.contains("hello", "el")) // true

// Splitting and joining
let parts = string.split("a,b,c", ",")  // ["a", "b", "c"]
let joined = string.join(parts, "-")     // "a-b-c"

// Trimming
print(string.trim("  hello  "))      // "hello"
print(string.ltrim("  hello"))        // "hello"
print(string.rtrim("hello  "))        // "hello"
```

### Array Module

```infra
import array

let numbers = [3, 1, 4, 1, 5]

// Array operations
print(array.length(numbers))         // 5
print(array.sum(numbers))            // 14
print(array.min(numbers))            // 1
print(array.max(numbers))            // 5
print(array.average(numbers))        // 2.8

// Searching
print(array.contains(numbers, 4))    // true
print(array.index_of(numbers, 1))    // 1 (first occurrence)

// Modifying arrays
array.push(numbers, 9)              // [3, 1, 4, 1, 5, 9]
let popped = array.pop(numbers)     // 9
array.insert(numbers, 2, 99)        // [3, 1, 99, 4, 1, 5]
array.remove(numbers, 1)            // [3, 99, 4, 5] (removes first 1)

// Sorting and reversing
array.sort(numbers)                 // [3, 4, 5, 99]
array.reverse(numbers)              // [99, 5, 4, 3]
```

### I/O Module

```infra
import io

// File operations
let content = io.read_file("data.txt")
io.write_file("output.txt", "Hello, World!")

// File information
if io.file_exists("data.txt"):
    print("File size:", io.file_size("data.txt"))
    print("File modified:", io.file_modified("data.txt"))

// Directory operations
io.create_dir("my_folder")
io.delete_dir("my_folder")
let files = io.list_files(".")  // List files in current directory

// Console I/O
let name = io.input("Enter your name: ")
print("Hello, " + name)
```

### Async Module

```infra
import async

// Async operations
async function demo():
    // Sleep
    await async.sleep(1000)  // Wait 1 second
    
    // Async file I/O
    let content = await async.read_file("large_file.txt")
    await async.write_file("backup.txt", content)
    
    // HTTP requests
    let response = await async.http_get("https://api.example.com")
    let data = json.parse(response)
    
    // Concurrent operations
    let promises = [
        async.http_get("https://api.example.com/users"),
        async.http_get("https://api.example.com/posts")
    ]
    let results = await async.all(promises)
    
    // Timeout
    let result = await async.timeout(
        async.http_get("https://slow-api.com"),
        5000  // 5 second timeout
    )
```

## Type System

Infra features an optional static type system with type inference:

### Basic Types

```infra
let x: number = 42
let name: string = "Infra"
let is_ready: boolean = true
let nothing: nil = nil

// Array types
let numbers: number[] = [1, 2, 3]
let strings: string[] = ["a", "b", "c"]

// Object types
let person: {name: string, age: number} = {
    "name": "Alice",
    "age": 30
}
```

### Function Types

```infra
// Function signature
let add: (number, number) -> number = (a, b) => a + b

// Optional parameters
function greet(name: string, suffix?: string): string {
    if suffix:
        return name + " " + suffix
    return name
}

// Union types
let value: number | string = 42
value = "hello"  // Also valid
```

### Type Inference

```infra
// Types are inferred when not specified
let x = 42           // Inferred as number
let name = "Infra"   // Inferred as string
let numbers = [1, 2, 3]  // Inferred as number[]

// Function return types inferred
function add(a, b) {
    return a + b  // Return type inferred
}
```

### Custom Types with Type Aliases

```infra
type UserID = number
type UserName = string
type User = {id: UserID, name: UserName}

function create_user(id: UserID, name: UserName): User {
    return {id: id, name: name}
}
```

## Advanced Features

### Closures

```infra
function make_counter():
    let count = 0
    return function():
        count = count + 1
        return count

let counter1 = make_counter()
let counter2 = make_counter()

print(counter1())  // 1
print(counter1())  // 2
print(counter2())  // 1 (separate counter)
```

### Decorators (Planned)

```infra
@measure_time
function expensive_operation():
    // Some expensive computation
    return result
```

### Generics (Planned)

```infra
function identity<T>(value: T): T {
    return value
}

class Container<T> {
    function init(value: T) {
        this.value = value
    }
    
    function get(): T {
        return this.value
    }
}
```

### Metaprogramming (Planned)

```infra
// Macros (future feature)
macro debug_print(expr):
    print(f"DEBUG: {expr} = {expr}")

debug_print(2 + 3)  // DEBUG: 2 + 3 = 5
```

## Best Practices

### Code Style

- Use descriptive variable names
- Follow naming conventions:
  - Variables and functions: `snake_case`
  - Classes: `PascalCase`
  - Constants: `UPPER_SNAKE_CASE`
- Keep functions small and focused
- Use type annotations for public APIs

### Performance

- Prefer `let` over `var` when possible
- Use appropriate data structures
- Consider async/await for I/O operations
- Profile performance-critical code

### Error Handling

- Handle expected errors gracefully
- Use specific exception types
- Provide meaningful error messages
- Clean up resources in `finally` blocks

### Testing

- Write tests for all public functions
- Test edge cases and error conditions
- Use descriptive test names
- Keep tests independent and repeatable

## Examples

### Web Server Example

```infra
import async
import http

class SimpleServer:
    function init(port: number):
        this.port = port
        this.routes = {}
    
    function route(path: string, handler):
        this.routes[path] = handler
    
    async function start():
        let server = await http.create_server(this.port)
        print(f"Server running on port {this.port}")
        
        async for request in server:
            await this.handle_request(request)
    
    async function handle_request(request):
        let handler = this.routes[request.path]
        if handler:
            let response = await handler(request)
            await request.send(response)
        else:
            await request.send(404, "Not Found")

// Usage
let server = SimpleServer(8080)
server.route("/", async req => "Hello, World!")
server.route("/api/data", async req => json.dumps({"message": "API data"}))

await server.start()
```

### Data Processing Example

```infra
import async
import io
import json

async function process_log_file(filename: string):
    let content = await async.read_file(filename)
    let lines = string.split(content, "\n")
    
    let error_count = 0
    let warnings = []
    
    for line in lines:
        if string.contains(line, "ERROR"):
            error_count = error_count + 1
        elif string.contains(line, "WARNING"):
            array.push(warnings, line)
    
    let report = {
        "file": filename,
        "total_lines": array.length(lines),
        "errors": error_count,
        "warnings": array.length(warnings),
        "warning_messages": warnings
    }
    
    await async.write_file("report.json", json.dumps(report))
    print(f"Processed {filename}: {error_count} errors found")

async function main():
    let files = io.list_files("logs/")
    
    let processors = []
    for file in files:
        if string.ends_with(file, ".log"):
            processors.push(process_log_file("logs/" + file))
    
    await async.all(processors)
    print("All log files processed")

await main()
```

This comprehensive guide covers the core features and capabilities of the Infra programming language. For more specific information about any particular feature, check the other documentation files or explore the standard library reference.