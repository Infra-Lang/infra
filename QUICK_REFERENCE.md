# Infra Programming Language - Quick Reference

## Syntax Cheat Sheet

### Variables & Types
```infra
let name = "Alice"                    # String (Python-style comment)
let age = 30                          // Number (C-style comment)
let is_active = true                  # Boolean
let scores = [95, 87, 92]            // Array
let person = {name: "Alice", age: 30} # Object

# With type annotations
let name: string = "Alice"
let scores: [number] = [95, 87, 92]
```

### Functions
```infra
# Basic function
function greet(name) {
    print("Hello, " + name)
}

# With types
function add(a: number, b: number) -> number {
    return a + b
}

# Alternative syntax
def multiply(x, y) {
    return x * y
}
```

### Control Flow
```infra
# If statement
if age >= 18 {
    print("Adult")
} else {
    print("Minor")
}

# While loop
while count < 10 {
    count = count + 1
}

# For loop
for i in 0..5 {
    print(i)
}
```

### Operators
```infra
# Arithmetic: +, -, *, /, %
# Comparison: ==, !=, <, <=, >, >=
# Logical: and, or, !
```

### Standard Library
```infra
import math
math.sqrt(16)           # 4
math.pow(2, 3)          # 8
math.max(1, 5, 3)       # 5

import string
string.length("hello")   # 5
string.upper("hello")    # "HELLO"
string.split("a,b", ",") # ["a", "b"]

import array
array.length([1,2,3])    # 3
array.push(arr, item)    # Add item
array.sort(arr)          # Sort array
```

### Error Handling
```infra
try {
    risky_operation()
} catch error {
    print("Error: " + error)
}
```

### Modules
```infra
# Import module
import math

# Import specific functions
import { sqrt, pow } from math

# Import with alias
import math as m
```

### File Extensions
- `.if` - Standard Infra files
- `.infra` - Alternative extension

### Running Code
```bash
infra script.if          # Run file
infra                    # Interactive REPL
infra --version          # Show version
```

## Common Patterns

### Array Processing
```infra
let numbers = [1, 2, 3, 4, 5]

# Sum array
let sum = 0
for num in numbers {
    sum = sum + num
}

# Find maximum
let max_val = numbers[0]
for num in numbers {
    if num > max_val {
        max_val = num
    }
}
```

### Object Creation
```infra
function create_user(name, email) {
    return {
        name: name,
        email: email,
        greet: function() {
            print("Hello, " + this.name)
        }
    }
}
```

### Input Validation
```infra
function safe_divide(a, b) {
    if b == 0 {
        return null
    }
    return a / b
}

let result = safe_divide(10, 2)
if result != null {
    print("Result: " + result)
}
```
