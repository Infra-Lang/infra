# Infra Standard Library Reference

The Infra Standard Library provides a comprehensive set of modules and functions for common programming tasks. This document serves as a complete reference for all available modules, functions, and their usage.

## Table of Contents

- [Core Modules](#core-modules)
  - [math](#math-module)
  - [string](#string-module)
  - [array](#array-module)
  - [io](#io-module)
  - [json](#json-module)
  - [date](#date-module)
  - [random](#random-module)
- [Async Modules](#async-modules)
  - [async](#async-module)
  - [http](#http-module)
- [System Modules](#system-modules)
  - [os](#os-module)
  - [path](#path-module)
  - [env](#env-module)
- [Utility Modules](#utility-modules)
  - [collections](#collections-module)
  - [regex](#regex-module)
  - [base64](#base64-module)

## Core Modules

### math Module

Mathematical operations and constants.

#### Constants

```infra
math.pi        // 3.141592653589793
math.e         // 2.718281828459045
math.tau       // 6.283185307179586
math.inf       // Infinity
math.nan       // Not a Number
```

#### Basic Operations

```infra
math.abs(x: number): number
// Returns the absolute value of x
math.abs(-5)        // 5
math.abs(3.14)      // 3.14

math.max(...values: number[]): number
// Returns the maximum value
math.max(1, 5, 3)   // 5
math.max([1, 5, 3]) // 5

math.min(...values: number[]): number
// Returns the minimum value
math.min(1, 5, 3)   // 1
math.min([1, 5, 3]) // 1

math.round(x: number): number
// Rounds to the nearest integer
math.round(3.7)     // 4
math.round(3.2)     // 3
math.round(3.5)     // 4

math.floor(x: number): number
// Rounds down to the nearest integer
math.floor(3.7)     // 3
math.floor(3.2)     // 3

math.ceil(x: number): number
// Rounds up to the nearest integer
math.ceil(3.7)      // 4
math.ceil(3.2)      // 4

math.trunc(x: number): number
// Truncates decimal part
math.trunc(3.7)     // 3
math.trunc(-3.7)    // -3
```

#### Powers and Roots

```infra
math.sqrt(x: number): number
// Square root
math.sqrt(16)       // 4.0
math.sqrt(2)        // 1.4142135623730951

math.pow(base: number, exponent: number): number
// Power function
math.pow(2, 3)      // 8.0
math.pow(4, 0.5)    // 2.0

math.exp(x: number): number
// e^x
math.exp(1)         // 2.718281828459045
math.exp(0)         // 1.0

math.log(x: number): number
// Natural logarithm
math.log(math.e)    // 1.0
math.log(1)         // 0.0

math.log10(x: number): number
// Base-10 logarithm
math.log10(100)     // 2.0
math.log10(10)      // 1.0

math.log2(x: number): number
// Base-2 logarithm
math.log2(8)        // 3.0
math.log2(2)        // 1.0
```

#### Trigonometry

```infra
math.sin(x: number): number
// Sine function (x in radians)
math.sin(math.pi / 2)    // 1.0
math.sin(0)              // 0.0

math.cos(x: number): number
// Cosine function (x in radians)
math.cos(0)              // 1.0
math.cos(math.pi)        // -1.0

math.tan(x: number): number
// Tangent function (x in radians)
math.tan(math.pi / 4)    // 1.0
math.tan(0)              // 0.0

math.asin(x: number): number
// Arc sine (returns radians)
math.asin(1)             // 1.5707963267948966 (π/2)

math.acos(x: number): number
// Arc cosine (returns radians)
math.acos(0)             // 1.5707963267948966 (π/2)

math.atan(x: number): number
// Arc tangent (returns radians)
math.atan(1)             // 0.7853981633974483 (π/4)

math.atan2(y: number, x: number): number
// Arc tangent of y/x (returns radians)
math.atan2(1, 1)         // 0.7853981633974483 (π/4)
```

#### Hyperbolic Functions

```infra
math.sinh(x: number): number     // Hyperbolic sine
math.cosh(x: number): number     // Hyperbolic cosine
math.tanh(x: number): number     // Hyperbolic tangent
```

#### Special Functions

```infra
math.factorial(n: number): number
// Factorial function
math.factorial(5)       // 120
math.factorial(0)       // 1

math.gcd(a: number, b: number): number
// Greatest common divisor
math.gcd(48, 18)        // 6

math.lcm(a: number, b: number): number
// Least common multiple
math.lcm(4, 6)          // 12

math.is_finite(x: number): boolean
// Check if number is finite
math.is_finite(42)      // true
math.is_finite(math.inf) // false

math.is_infinite(x: number): boolean
// Check if number is infinite
math.is_infinite(math.inf)  // true
math.is_infinite(42)        // false

math.is_nan(x: number): boolean
// Check if number is NaN
math.is_nan(math.nan)    // true
math.is_nan(42)          // false
```

#### Statistics

```infra
math.average(numbers: number[]): number
// Arithmetic mean
math.average([1, 2, 3, 4, 5])    // 3.0

math.median(numbers: number[]): number
// Median value
math.median([1, 3, 5])          // 3
math.median([1, 2, 3, 4])       // 2.5

math.mode(numbers: number[]): number[]
// Most frequent values
math.mode([1, 2, 2, 3])         // [2]

math.variance(numbers: number[]): number
// Sample variance
math.variance([1, 2, 3, 4, 5])  // 2.5

math.std_dev(numbers: number[]): number
// Standard deviation
math.std_dev([1, 2, 3, 4, 5])   // 1.5811388300841898
```

### string Module

String manipulation and analysis functions.

#### Case Operations

```infra
string.upper(s: string): string
// Convert to uppercase
string.upper("hello")    // "HELLO"
string.upper("Hello")    // "HELLO"

string.lower(s: string): string
// Convert to lowercase
string.lower("WORLD")    // "world"
string.lower("World")    // "world"

string.capitalize(s: string): string
// Capitalize first letter
string.capitalize("hello world")  // "Hello world"
string.capitalize("HELLO")        // "Hello"

string.title(s: string): string
// Title case (first letter of each word)
string.title("hello world")       // "Hello World"
string.title("HELLO WORLD")       // "Hello World"
```

#### String Information

```infra
string.length(s: string): number
// String length
string.length("hello")    // 5
string.length("")         // 0

string.is_empty(s: string): boolean
// Check if string is empty
string.is_empty("")       // true
string.is_empty("hello")  // false

string.contains(s: string, substring: string): boolean
// Check if substring exists
string.contains("hello", "el")    // true
string.contains("hello", "world") // false

string.starts_with(s: string, prefix: string): boolean
// Check if string starts with prefix
string.starts_with("hello", "he")    // true
string.starts_with("hello", "lo")     // false

string.ends_with(s: string, suffix: string): boolean
// Check if string ends with suffix
string.ends_with("hello", "lo")       // true
string.ends_with("hello", "he")       // false
```

#### Searching and Replacing

```infra
string.index_of(s: string, substring: string): number
// First occurrence index (-1 if not found)
string.index_of("hello", "l")     // 2
string.index_of("hello", "z")     // -1

string.last_index_of(s: string, substring: string): number
// Last occurrence index (-1 if not found)
string.last_index_of("hello", "l")    // 3
string.last_index_of("hello", "z")    // -1

string.replace(s: string, old: string, new: string): string
// Replace all occurrences
string.replace("hello world", "world", "there")  // "hello there"
string.replace("aaaa", "a", "b")                 // "bbbb"

string.replace_first(s: string, old: string, new: string): string
// Replace first occurrence only
string.replace_first("hello world hello", "hello", "hi")  // "hi world hello"
```

#### Splitting and Joining

```infra
string.split(s: string, delimiter: string): string[]
// Split string by delimiter
string.split("a,b,c", ",")    // ["a", "b", "c"]
string.split("hello world", " ")  // ["hello", "world"]

string.join(parts: string[], delimiter: string): string
// Join array of strings with delimiter
string.join(["a", "b", "c"], ",")  // "a,b,c"
string.join(["hello", "world"], " ")  // "hello world"

string.lines(s: string): string[]
// Split string into lines
string.lines("line1\nline2\nline3")  // ["line1", "line2", "line3"]

string.words(s: string): string[]
// Split string into words
string.words("hello world there")    // ["hello", "world", "there"]
```

#### Trimming and Padding

```infra
string.trim(s: string): string
// Remove whitespace from both ends
string.trim("  hello  ")    // "hello"

string.ltrim(s: string): string
// Remove whitespace from left
string.ltrim("  hello")     // "hello"

string.rtrim(s: string): string
// Remove whitespace from right
string.rtrim("hello  ")     // "hello"

string.pad_left(s: string, length: number, char?: string): string
// Pad string to length on the left
string.pad_left("5", 3, "0")    // "005"
string.pad_left("hello", 10)    // "     hello"

string.pad_right(s: string, length: number, char?: string): string
// Pad string to length on the right
string.pad_right("hello", 10, "-")  // "hello-----"

string.center(s: string, length: number, char?: string): string
// Center string in field of specified length
string.center("hello", 10, "=")  // "==hello=="
```

#### Substring Operations

```infra
string.substring(s: string, start: number, end?: number): string
// Extract substring
string.substring("hello", 1, 3)   // "el"
string.substring("hello", 1)      // "ello"

string.slice(s: string, start: number, end?: number): string
// Similar to substring but supports negative indices
string.slice("hello", -3)         // "llo"
string.slice("hello", 1, -1)      // "ell"

string.left(s: string, n: number): string
// Get first n characters
string.left("hello", 3)           // "hel"

string.right(s: string, n: number): string
// Get last n characters
string.right("hello", 3)          // "llo"
```

#### Character Operations

```infra
string.char_at(s: string, index: number): string
// Get character at index
string.char_at("hello", 1)        // "e"

string.char_code(s: string, index: number): number
// Get Unicode code point at index
string.char_code("A", 0)          // 65

string.from_char_code(...codes: number[]): string
// Create string from Unicode codes
string.from_char_code(65, 66, 67) // "ABC"
```

#### Pattern Matching

```infra
string.count(s: string, substring: string): number
// Count occurrences of substring
string.count("hello world hello", "hello")  // 2

string.repeat(s: string, n: number): string
// Repeat string n times
string.repeat("abc", 3)        // "abcabcabc"

string.reverse(s: string): string
// Reverse string
string.reverse("hello")        // "olleh"
```

### array Module

Array manipulation and operations.

#### Basic Operations

```infra
array.length(arr: array): number
// Get array length
array.length([1, 2, 3])     // 3

array.is_empty(arr: array): boolean
// Check if array is empty
array.is_empty([])          // true
array.is_empty([1])         // false

array.copy(arr: array): array
// Create shallow copy
let original = [1, 2, 3]
let copy = array.copy(original)
```

#### Adding and Removing Elements

```infra
array.push(arr: array, element): void
// Add element to end
let arr = [1, 2, 3]
array.push(arr, 4)          // [1, 2, 3, 4]

array.pop(arr: array): any
// Remove and return last element
let arr = [1, 2, 3]
let last = array.pop(arr)   // last = 3, arr = [1, 2]

array.unshift(arr: array, element): void
// Add element to beginning
let arr = [2, 3, 4]
array.unshift(arr, 1)       // [1, 2, 3, 4]

array.shift(arr: array): any
// Remove and return first element
let arr = [1, 2, 3]
let first = array.shift(arr) // first = 1, arr = [2, 3]
```

#### Insertion and Deletion

```infra
array.insert(arr: array, index: number, element): void
// Insert element at index
let arr = [1, 3, 4]
array.insert(arr, 1, 2)     // [1, 2, 3, 4]

array.remove(arr: array, element): boolean
// Remove first occurrence of element
let arr = [1, 2, 3, 2, 4]
array.remove(arr, 2)        // [1, 3, 2, 4], returns true

array.remove_at(arr: array, index: number): any
// Remove element at index
let arr = [1, 2, 3, 4]
let removed = array.remove_at(arr, 1)  // removed = 2, arr = [1, 3, 4]

array.clear(arr: array): void
// Remove all elements
let arr = [1, 2, 3]
array.clear(arr)            // []
```

#### Searching and Testing

```infra
array.contains(arr: array, element): boolean
// Check if array contains element
array.contains([1, 2, 3], 2)    // true
array.contains([1, 2, 3], 5)    // false

array.index_of(arr: array, element): number
// Find first index of element (-1 if not found)
array.index_of([1, 2, 3, 2], 2) // 1

array.last_index_of(arr: array, element): number
// Find last index of element (-1 if not found)
array.last_index_of([1, 2, 3, 2], 2)  // 3

array.find(arr: array, predicate: function): any
// Find first element matching predicate
array.find([1, 2, 3, 4], x => x > 2)  // 3

array.find_index(arr: array, predicate: function): number
// Find index of first element matching predicate
array.find_index([1, 2, 3, 4], x => x > 2)  // 2
```

#### Transformation

```infra
array.map(arr: array, transform: function): array
// Transform each element
array.map([1, 2, 3], x => x * 2)    // [2, 4, 6]

array.filter(arr: array, predicate: function): array
// Filter elements matching predicate
array.filter([1, 2, 3, 4], x => x > 2)  // [3, 4]

array.reduce(arr: array, accumulator: function, initial?: any): any
// Reduce array to single value
array.reduce([1, 2, 3, 4], (sum, x) => sum + x, 0)  // 10
array.reduce([1, 2, 3, 4], (sum, x) => sum + x)     // 10

array.flat_map(arr: array, transform: function): array
// Map and flatten results
array.flat_map([1, 2, 3], x => [x, x * 2])  // [1, 2, 2, 4, 3, 6]
```

#### Sorting and Reversing

```infra
array.sort(arr: array, compare?: function): void
// Sort array in place
let arr = [3, 1, 4, 1, 5]
array.sort(arr)                    // [1, 1, 3, 4, 5]

// With custom compare function
array.sort(arr, (a, b) => b - a)   // [5, 4, 3, 1, 1] (descending)

array.sorted(arr: array, compare?: function): array
// Return sorted copy
array.sorted([3, 1, 4])            // [1, 3, 4]

array.reverse(arr: array): void
// Reverse array in place
let arr = [1, 2, 3]
array.reverse(arr)                 // [3, 2, 1]

array.reversed(arr: array): array
// Return reversed copy
array.reversed([1, 2, 3])          // [3, 2, 1]
```

#### Mathematical Operations

```infra
array.sum(arr: number[]): number
// Sum of numeric elements
array.sum([1, 2, 3, 4])       // 10

array.min(arr: array): any
// Minimum element
array.min([3, 1, 4, 1, 5])    // 1

array.max(arr: array): any
// Maximum element
array.max([3, 1, 4, 1, 5])    // 5

array.average(arr: number[]): number
// Arithmetic mean
array.average([1, 2, 3, 4, 5])    // 3.0

array.product(arr: number[]): number
// Product of all elements
array.product([1, 2, 3, 4])    // 24
```

#### Set Operations

```infra
array.unique(arr: array): array
// Remove duplicates
array.unique([1, 2, 2, 3, 1])  // [1, 2, 3]

array.union(arr1: array, arr2: array): array
// Union of two arrays
array.union([1, 2, 3], [3, 4, 5])    // [1, 2, 3, 4, 5]

array.intersection(arr1: array, arr2: array): array
// Intersection of two arrays
array.intersection([1, 2, 3], [2, 3, 4])  // [2, 3]

array.difference(arr1: array, arr2: array): array
// Elements in arr1 but not in arr2
array.difference([1, 2, 3, 4], [2, 3])   // [1, 4]
```

#### Array Creation

```infra
array.range(start: number, end?: number, step?: number): number[]
// Create range of numbers
array.range(5)              // [0, 1, 2, 3, 4]
array.range(1, 5)           // [1, 2, 3, 4]
array.range(0, 10, 2)       // [0, 2, 4, 6, 8]

array.fill(value: any, count: number): array
// Create array filled with value
array.fill(0, 5)            // [0, 0, 0, 0, 0]

array.of(...elements): array
// Create array from arguments
array.of(1, 2, 3)           // [1, 2, 3]
```

### io Module

Input/output operations for files and console.

#### File Operations

```infra
io.read_file(path: string): string
// Read entire file as string
let content = io.read_file("data.txt")

io.write_file(path: string, content: string, append?: boolean): void
// Write string to file
io.write_file("output.txt", "Hello, World!")
io.write_file("log.txt", "New entry\n", append=true)

io.append_file(path: string, content: string): void
// Append to file (same as write_file with append=true)
io.append_file("log.txt", "Another entry\n")
```

#### File Information

```infra
io.file_exists(path: string): boolean
// Check if file exists
if io.file_exists("config.txt"):
    print("Config file found")

io.file_size(path: string): number
// Get file size in bytes
let size = io.file_size("large_file.txt")

io.file_modified(path: string): number
// Get last modified timestamp
let modified = io.file_modified("data.txt")

io.is_file(path: string): boolean
// Check if path is a file
io.is_file("data.txt")      // true
io.is_file("folder")        // false

io.is_directory(path: string): boolean
// Check if path is a directory
io.is_directory("folder")   // true
io.is_directory("data.txt") // false
```

#### Directory Operations

```infra
io.create_dir(path: string): void
// Create directory
io.create_dir("my_folder")

io.create_dirs(path: string): void
// Create directory and parent directories if needed
io.create_dirs("path/to/deep/folder")

io.delete_dir(path: string): void
// Delete empty directory
io.delete_dir("empty_folder")

io.delete_dir_recursive(path: string): void
// Delete directory and all contents
io.delete_dir_recursive("folder_with_contents")

io.list_files(path: string): string[]
// List files in directory
let files = io.list_files(".")     // Files in current directory

io.list_dirs(path: string): string[]
// List subdirectories
let dirs = io.list_dirs(".")       // Subdirectories in current directory
```

#### File and Directory Management

```infra
io.copy_file(source: string, destination: string): void
// Copy file
io.copy_file("source.txt", "backup.txt")

io.move_file(source: string, destination: string): void
// Move/rename file
io.move_file("old_name.txt", "new_name.txt")

io.delete_file(path: string): void
// Delete file
io.delete_file("temp.txt")

io.copy_dir(source: string, destination: string): void
// Copy directory recursively
io.copy_dir("src", "backup")

io.move_dir(source: string, destination: string): void
// Move/rename directory
io.move_dir("old_folder", "new_folder")
```

#### Console I/O

```infra
io.print(...args): void
// Print arguments to console
io.print("Hello", "World")    // "Hello World"

io.println(...args): void
// Print with newline
io.println("Line 1")          // "Line 1\n"

io.input(prompt?: string): string
// Read input from user
let name = io.input("Enter your name: ")
print(f"Hello, {name}")

io.read_line(): string
// Read a line of input
let line = io.read_line()
```

#### Path Operations

```infra
io.join_path(...parts: string[]): string
// Join path components
io.join_path("home", "user", "documents")  // "home/user/documents" (OS-specific)

io.dirname(path: string): string
// Get directory part of path
io.dirname("/home/user/file.txt")  // "/home/user"

io.basename(path: string): string
// Get filename part of path
io.basename("/home/user/file.txt") // "file.txt"

io.extension(path: string): string
// Get file extension
io.extension("file.txt")           // ".txt"

io.remove_extension(path: string): string
// Remove extension from path
io.remove_extension("file.txt")    // "file"
```

#### Working Directory

```infra
io.current_dir(): string
// Get current working directory
let cwd = io.current_dir()

io.change_dir(path: string): void
// Change current working directory
io.change_dir("/home/user")

io.temp_dir(): string
// Get system temporary directory
let temp = io.temp_dir()
```

### json Module

JSON serialization and deserialization.

```infra
json.parse(text: string): any
// Parse JSON string to native types
let data = json.parse('{"name": "Alice", "age": 30}')
print(data.name)           // "Alice"
print(data.age)            // 30

json.dumps(value: any, indent?: number): string
// Convert value to JSON string
let obj = {"name": "Bob", "age": 25}
let json_str = json.dumps(obj)
print(json_str)            // {"name":"Bob","age":25}

// With pretty printing
let pretty = json.dumps(obj, 2)
print(pretty)
// {
//   "name": "Bob",
//   "age": 25
// }

json.is_valid(text: string): boolean
// Check if string is valid JSON
json.is_valid('{"name": "test"}')    // true
json.is_valid('invalid')             // false
```

### date Module

Date and time operations.

#### Creating Dates

```infra
date.now(): number
// Current timestamp in milliseconds
let timestamp = date.now()

date.new(year?: number, month?: number, day?: number, 
         hour?: number, minute?: number, second?: number): number
// Create timestamp for specific date
let birthday = date.new(1990, 5, 15)  // May 15, 1990

date.from_string(s: string): number
// Parse date from string
let parsed = date.from_string("2023-05-15T10:30:00")
```

#### Formatting Dates

```infra
date.format(timestamp: number, format: string): string
// Format timestamp as string
let now = date.now()
date.format(now, "%Y-%m-%d")           // "2023-05-15"
date.format(now, "%Y-%m-%d %H:%M:%S") // "2023-05-15 10:30:00"

// Format specifiers:
// %Y - 4-digit year
// %m - 2-digit month (01-12)
// %d - 2-digit day (01-31)
// %H - 2-digit hour (00-23)
// %M - 2-digit minute (00-59)
// %S - 2-digit second (00-59)

date.iso_string(timestamp: number): string
// ISO 8601 format
date.iso_string(date.now())  // "2023-05-15T10:30:00.123Z"
```

#### Date Components

```infra
date.year(timestamp: number): number
date.month(timestamp: number): number
date.day(timestamp: number): number
date.hour(timestamp: number): number
date.minute(timestamp: number): number
date.second(timestamp: number): number

let now = date.now()
print(date.year(now))   // 2023
print(date.month(now))  // 5
print(date.day(now))    // 15
```

#### Date Arithmetic

```infra
date.add_years(timestamp: number, years: number): number
date.add_months(timestamp: number, months: number): number
date.add_days(timestamp: number, days: number): number
date.add_hours(timestamp: number, hours: number): number
date.add_minutes(timestamp: number, minutes: number): number
date.add_seconds(timestamp: number, seconds: number): number

let now = date.now()
let next_week = date.add_days(now, 7)
let next_month = date.add_months(now, 1)
```

#### Date Comparison

```infra
date.is_same_day(ts1: number, ts2: number): boolean
date.is_same_week(ts1: number, ts2: number): boolean
date.is_same_month(ts1: number, ts2: number): boolean
date.is_same_year(ts1: number, ts2: number): boolean

let today = date.now()
let tomorrow = date.add_days(today, 1)
date.is_same_day(today, tomorrow)  // false
```

### random Module

Random number generation.

```infra
random.float(): number
// Random float in [0, 1)
random.float()                    // 0.723847...

random.float_range(min: number, max: number): number
// Random float in [min, max)
random.float_range(1.0, 10.0)     // 3.14159...

random.int(max: number): number
// Random integer in [0, max)
random.int(10)                    // 7

random.int_range(min: number, max: number): number
// Random integer in [min, max)
random.int_range(1, 10)           // 5

random.choice(array: array): any
// Random element from array
random.choice([1, 2, 3, 4, 5])   // 3

random.shuffle(array: array): void
// Shuffle array in place
let arr = [1, 2, 3, 4, 5]
random.shuffle(arr)               // [3, 1, 5, 2, 4] (random)

random.sample(array: array, count: number): array
// Random sample without replacement
random.sample([1, 2, 3, 4, 5], 3) // [2, 5, 1] (random)

random.seed(value: number): void
// Set random seed
random.seed(12345)
```

## Async Modules

### async Module

Asynchronous operations utilities.

```infra
async.sleep(ms: number): void
// Sleep for specified milliseconds
await async.sleep(1000)  // Wait 1 second

async.all(promises: Promise[]): Promise
// Wait for all promises to resolve
let promises = [
    async.sleep(100),
    async.sleep(200),
    async.sleep(150)
]
await async.all(promises)

async.race(promises: Promise[]): Promise
// Wait for first promise to resolve or reject
let result = await async.race([
    async.sleep(1000),
    async.sleep(500)
])  // Resolves after 500ms

async.timeout(promise: Promise, ms: number): Promise
// Reject promise if it doesn't resolve within timeout
try:
    let result = await async.timeout(
        async.sleep(2000),
        1000
    )
except TimeoutError:
    print("Operation timed out")

async.create_promise(): Promise
// Create manually controlled promise
let promise = async.create_promise()
// Later:
promise.resolve("success")

async.defer(func: function): Promise
// Defer function execution to next event loop tick
await async.defer(() => {
    print("This runs later")
})
```

### async File Operations

```infra
async.read_file(path: string): Promise<string>
// Asynchronously read file
let content = await async.read_file("data.txt")

async.write_file(path: string, content: string, append?: boolean): Promise<void>
// Asynchronously write file
await async.write_file("output.txt", "Hello, Async!")

async.append_file(path: string, content: string): Promise<void>
// Asynchronously append to file
await async.append_file("log.txt", "New log entry\n")

async.file_exists(path: string): Promise<boolean>
// Asynchronously check if file exists
let exists = await async.file_exists("data.txt")
```

### http Module

HTTP client for making web requests.

```infra
async.http_get(url: string, headers?: object): Promise<string>
// Make GET request
let response = await async.http_get("https://api.example.com/data")
let data = json.parse(response)

async.http_post(url: string, body: string, headers?: object): Promise<string>
// Make POST request
let payload = json.dumps({"name": "Alice", "age": 30})
let response = await async.http_post(
    "https://api.example.com/users",
    payload,
    {"Content-Type": "application/json"}
)

async.http_put(url: string, body: string, headers?: object): Promise<string>
// Make PUT request
async.http_delete(url: string, headers?: object): Promise<string>
// Make DELETE request

async.http_request(method: string, url: string, options?: object): Promise<object>
// Make custom HTTP request
let response = await async.http_request("GET", "https://api.example.com", {
    "headers": {"Authorization": "Bearer token"},
    "timeout": 5000
})
print(response.status)  // 200
print(response.body)    // Response body
print(response.headers) // Response headers
```

## System Modules

### os Module

Operating system interface.

```infra
os.platform(): string
// Get OS platform
os.platform()  // "windows", "linux", "macos"

os.arch(): string
// Get system architecture
os.arch()      // "x64", "arm64", etc.

os.version(): string
// Get OS version
os.version()   // "10.0.19041" etc.

os.hostname(): string
// Get system hostname
os.hostname()  // "my-computer"

os.environment(): object
// Get all environment variables
let env = os.environment()
print(env.PATH)      // System PATH
print(env.HOME)      // Home directory

os.get_env(name: string): string | null
// Get specific environment variable
let path = os.get_env("PATH")

os.set_env(name: string, value: string): void
// Set environment variable
os.set_env("MY_VAR", "value")

os.unset_env(name: string): void
// Unset environment variable
os.unset_env("MY_VAR")
```

### path Module

Path manipulation utilities.

```infra
path.normalize(p: string): string
// Normalize path separators
path.normalize("folder\\subfolder/file")  // "folder/subfolder/file"

path.is_absolute(p: string): boolean
// Check if path is absolute
path.is_absolute("/home/user")  // true
path.is_absolute("folder/file") // false

path.is_relative(p: string): boolean
// Check if path is relative
path.is_relative("folder/file") // true

path.join(...parts: string[]): string
// Join path components
path.join("home", "user", "documents")  // "home/user/documents"

path.dirname(p: string): string
path.basename(p: string): string
path.extension(p: string): string
path.remove_extension(p: string): string

path.resolve(...parts: string[]): string
// Resolve to absolute path
path.resolve("folder", "file.txt")  // "/current/dir/folder/file.txt"

path.relative(from: string, to: string): string
// Get relative path from to
path.relative("/home/user/docs", "/home/user/pics/file.jpg")
// "../pics/file.jpg"
```

### env Module

Environment and configuration utilities.

```infra
env.args(): string[]
// Get command line arguments
env.args()  // ["program.if", "arg1", "arg2"]

env.exec_path(): string
// Get path to current executable
env.exec_path()  // "/path/to/infra"

env.cwd(): string
// Get current working directory
env.cwd()  // "/home/user/project"

env.exit(code?: number): void
// Exit program
env.exit(0)   // Success
env.exit(1)   // Error
```

## Utility Modules

### collections Module

Advanced collection data structures.

```infra
collections.Stack()
// Stack data structure (LIFO)
let stack = collections.Stack()
stack.push(1)
stack.push(2)
print(stack.pop())  // 2
print(stack.peek()) // 1

collections.Queue()
// Queue data structure (FIFO)
let queue = collections.Queue()
queue.enqueue("first")
queue.enqueue("second")
print(queue.dequeue())  // "first"
print(queue.peek())     // "second"

collections.Set()
// Set data structure
let set = collections.Set()
set.add(1)
set.add(2)
set.add(1)  // Duplicate ignored
print(set.has(1))  // true
print(set.has(3))  // false

collections.Map()
// Map data structure
let map = collections.Map()
map.set("key1", "value1")
map.set("key2", "value2")
print(map.get("key1"))  // "value1"
print(map.has("key2"))  // true
```

### regex Module

Regular expression support.

```infra
regex.compile(pattern: string): RegExp
// Compile regular expression
let pattern = regex.compile(r"\d+")

regex.test(pattern: RegExp, string: string): boolean
// Test if string matches pattern
regex.test(pattern, "123")     // true
regex.test(pattern, "abc")     // false

regex.match(pattern: RegExp, string: string): array?
// Find matches
let matches = regex.match(pattern, "123 abc 456")
// ["123", "456"]

regex.replace(pattern: RegExp, string: string, replacement: string): string
// Replace matches
let result = regex.replace(pattern, "123 abc 456", "#")
// "# abc #"

regex.split(pattern: RegExp, string: string): string[]
// Split by pattern
let parts = regex.split(pattern, "123,abc,456")
// ["", ",", "abc,", ""]
```

### base64 Module

Base64 encoding and decoding.

```infra
base64.encode(input: string): string
// Encode string to base64
base64.encode("Hello, World!")  // "SGVsbG8sIFdvcmxkIQ=="

base64.decode(input: string): string
// Decode base64 string
base64.decode("SGVsbG8sIFdvcmxkIQ==")  // "Hello, World!"

base64.encode_url(input: string): string
// URL-safe base64 encoding (no +, /, padding)
base64.encode_url("data+value/")  // "ZGF0YSt2YWx1ZQ=="

base64.decode_url(input: string): string
// URL-safe base64 decoding
base64.decode_url("ZGF0YSt2YWx1ZQ==")  // "data+value/"
```

## Usage Examples

### File Processing Example

```infra
import io
import json
import async

async function process_json_files():
    let files = io.list_files(".")
    
    for file in files:
        if string.ends_with(file, ".json"):
            print(f"Processing {file}")
            
            try:
                let content = await async.read_file(file)
                let data = json.parse(content)
                
                // Process data
                data.processed = true
                data.timestamp = date.now()
                
                // Write back
                let output = json.dumps(data, 2)
                await async.write_file("processed_" + file, output)
                
            except error:
                print(f"Error processing {file}: {error}")

await process_json_files()
```

### Data Analysis Example

```infra
import array
import math
import io

function analyze_numbers(filename: string):
    let content = io.read_file(filename)
    let lines = string.split(content, "\n")
    
    let numbers = []
    for line in lines:
        if not string.is_empty(string.trim(line)):
            array.push(numbers, number(line))
    
    print(f"Count: {array.length(numbers)}")
    print(f"Sum: {array.sum(numbers)}")
    print(f"Average: {array.average(numbers)}")
    print(f"Min: {array.min(numbers)}")
    print(f"Max: {array.max(numbers)}")
    print(f"Std Dev: {math.std_dev(numbers)}")

analyze_numbers("numbers.txt")
```

### Web API Example

```infra
import async
import http
import json
import io

async function fetch_user_data(user_id: number):
    let url = f"https://jsonplaceholder.typicode.com/users/{user_id}"
    
    try:
        let response = await async.http_get(url)
        let user = json.parse(response)
        
        print(f"User: {user.name}")
        print(f"Email: {user.email}")
        print(f"Company: {user.company.name}")
        
        // Save to file
        let filename = f"user_{user_id}.json"
        await async.write_file(filename, json.dumps(user, 2))
        print(f"Saved to {filename}")
        
    except error:
        print(f"Failed to fetch user {user_id}: {error}")

// Fetch multiple users concurrently
async function main():
    let user_ids = [1, 2, 3, 4, 5]
    let promises = []
    
    for id in user_ids:
        array.push(promises, fetch_user_data(id))
    
    await async.all(promises)
    print("All users fetched")

await main()
```

This comprehensive reference covers all the modules and functions available in the Infra Standard Library. Each module is designed to provide intuitive, powerful functionality for common programming tasks while maintaining consistency with the language's design philosophy.