# Async/Await Programming in Infra

Infra provides powerful asynchronous programming capabilities through `async/await` syntax, promises, and an event loop. This guide covers everything you need to write modern asynchronous code in Infra.

## Table of Contents

- [Quick Start](#quick-start)
- [Basic Concepts](#basic-concepts)
- [Async Functions](#async-functions)
- [Promises](#promises)
- [Await Expression](#await-expression)
- [Error Handling](#error-handling)
- [Standard Library Functions](#standard-library-functions)
- [Advanced Patterns](#advanced-patterns)
- [Examples](#examples)

## Quick Start

```infra
# Basic async function
async function fetch_data():
    return "Data fetched!"

# Using async functions
async function main():
    let data = await fetch_data()
    print(data)  # "Data fetched!"
```

## Basic Concepts

### Async Functions
Functions marked with `async` always return promises and can use the `await` keyword inside them.

```infra
async function calculate(a: number, b: number) -> number:
    await async.sleep(100)  # Simulate async work
    return a + b
```

### Promises
Promises represent values that may not be available yet. They can be in one of three states:
- **Pending**: Still working on the value
- **Resolved**: Successfully completed with a value
- **Rejected**: Failed with an error

### Event Loop
Infra uses an event loop to manage asynchronous operations, ensuring non-blocking execution.

## Async Functions

### Declaration
```infra
async function function_name(param1: type1, param2: type2) -> return_type:
    # Async work here
    return result
```

### Return Types
Async functions automatically return promises. The return type annotation specifies the resolved value type.

```infra
async function get_user(id: number) -> string:
    # Async user lookup
    return "User " + id.toString()
```

## Promises

### Creating Promises
```infra
# Create a resolved promise
let promise = async.create_promise("Hello World")

# Create a rejected promise
let error_promise = async.create_rejected_promise("Something went wrong")
```

### Promise Chaining
```infra
let result = await async.then(promise, func(value): return "Processed: " + value)
```

### Promise Utilities

#### All
Wait for all promises to resolve:
```infra
let promises = [promise1, promise2, promise3]
let results = await async.all(promises)
```

#### Race
Return the first promise that resolves or rejects:
```infra
let winner = await async.race([fast_promise, slow_promise])
```

## Await Expression

The `await` keyword suspends execution until a promise resolves.

### Basic Usage
```infra
async function example():
    let data = await fetch_data()
    print(data)
```

### Multiple Awaits
```infra
async function fetch_and_process():
    let data = await fetch_data()
    let processed = await process_data(data)
    print(processed)
```

## Error Handling

### Try-Catch with Async
```infra
async function safe_operation():
    try:
        let result = await risky_operation()
        return "Success: " + result
    catch error:
        return "Error: " + error
```

### Error Promises
```infra
async function handle_errors():
    let promise = async.create_rejected_promise("Network error")
    
    try:
        let result = await promise
        return result
    catch error:
        return "Caught error: " + error
```

## Standard Library Functions

### Timing Functions

#### Sleep
Pause execution for a specified number of milliseconds:
```infra
async function delayed_greeting():
    await async.sleep(1000)  # Wait 1 second
    print("Hello after delay!")
```

#### Timeout
Create a promise that rejects after a specified duration:
```infra
async function with_timeout():
    try:
        let result = await async.race([
            slow_operation(),
            async.timeout(2000)  # Timeout after 2 seconds
        ])
        return result
    catch error:
        return "Operation timed out"
```

### File Operations

#### Read File Asynchronously
```infra
async function read_config():
    let content = await async.read_file("config.json")
    return "Config: " + content
```

#### Write File Asynchronously
```infra
async function save_data():
    let data = "Important information"
    await async.write_file("output.txt", data)
    return "Data saved successfully"
```

### HTTP Operations

#### HTTP GET Request
```infra
async function fetch_api_data():
    let response = await async.http_get("https://api.example.com/data")
    return response.body
```

The HTTP response object includes:
- `status`: HTTP status code (number)
- `body`: Response body (string)
- `ok`: Boolean indicating success

## Advanced Patterns

### Parallel Operations
```infra
async function parallel_processing():
    let tasks = [
        fetch_user_data(),
        fetch_posts(),
        fetch_comments()
    ]
    
    let results = await async.all(tasks)
    return results
```

### Sequential Processing
```infra
async function pipeline():
    # Step 1
    let raw_data = await fetch_data()
    
    # Step 2 (depends on step 1)
    let processed_data = await process_data(raw_data)
    
    # Step 3 (depends on step 2)
    let result = await save_data(processed_data)
    
    return result
```

### Error Recovery
```infra
async function robust_operation():
    let max_retries = 3
    let attempts = 0
    
    while attempts < max_retries:
        try:
            return await risky_operation()
        catch error:
            attempts = attempts + 1
            if attempts < max_retries:
                await async.sleep(1000 * attempts)  # Exponential backoff
    
    return "Operation failed after " + max_retries.toString() + " attempts"
```

### Async Iteration
```infra
async function process_items(items):
    let results = []
    
    for item in items:
        let processed = await process_item(item)
        results.push(processed)
    
    return results
```

### Async Generators (Conceptual)
While Infra doesn't have async generators yet, you can simulate them:
```infra
async function async_generator(items):
    let index = 0
    
    func next_item():
        if index >= items.length:
            return null
        else:
            let item = items[index]
            index = index + 1
            return await process_item(item)
    
    return next_item
```

## Examples

### Web Server Simulation
```infra
async function handle_request(request):
    print("Handling request:", request)
    
    # Simulate database query
    await async.sleep(50)
    let data = "Database result for " + request
    
    # Simulate processing
    await async.sleep(25)
    let response = "Processed: " + data
    
    return response

async function web_server():
    let requests = ["GET /home", "GET /about", "POST /api/data"]
    
    for request in requests:
        let response = await handle_request(request)
        print("Response:", response)
```

### File Processing Pipeline
```infra
async function process_files(filenames):
    let results = []
    
    for filename in filenames:
        # Read file
        let content = await async.read_file(filename)
        
        # Process content
        let processed = "Processed: " + content.upper()
        
        # Save processed file
        let output_file = "processed_" + filename
        await async.write_file(output_file, processed)
        
        results.push(output_file)
    
    return results
```

### Data Aggregation
```infra
async function aggregate_data(sources):
    let promises = []
    
    # Create fetch promises for all sources
    for source in sources:
        let promise = async.http_get(source)
        promises.push(promise)
    
    # Wait for all to complete
    let responses = await async.all(promises)
    
    # Aggregate results
    let total_length = 0
    for response in responses:
        total_length = total_length + response.body.length()
    
    return "Total data: " + total_length.toString() + " characters"
```

## Best Practices

### 1. Always Handle Errors
```infra
# Good: Always include error handling
async function safe_operation():
    try:
        return await risky_operation()
    catch error:
        print("Error:", error)
        return null

# Bad: No error handling
async function unsafe_operation():
    return await risky_operation()  # May crash if rejected
```

### 2. Use Appropriate Timeouts
```infra
async function network_call_with_timeout():
    try:
        return await async.race([
            http_get("https://api.example.com"),
            async.timeout(5000)  # 5 second timeout
        ])
    catch error:
        return "Request timed out"
```

### 3. Prefer Parallel Operations When Possible
```infra
# Good: Parallel operations
async function fetch_all_data():
    let urls = ["url1", "url2", "url3"]
    let promises = urls.map(url => http_get(url))
    return await async.all(promises)

# Less efficient: Sequential operations
async function fetch_all_data_sequential():
    let urls = ["url1", "url2", "url3"]
    let results = []
    for url in urls:
        results.push(await http_get(url))
    return results
```

### 4. Use Meaningful Variable Names
```infra
# Good: Clear naming
async function fetch_user_profile(user_id):
    let user_data = await async.http_get("/api/users/" + user_id.toString())
    let user_profile = await async.http_get("/api/profiles/" + user_id.toString())
    return { user: user_data, profile: user_profile }

# Less clear: Vague naming
async function get_data(id):
    let data1 = await async.http_get("/api/data1/" + id.toString())
    let data2 = await async.http_get("/api/data2/" + id.toString())
    return [data1, data2]
```

## Performance Considerations

### 1. Avoid Unnecessary Sequential Awaits
```infra
# Avoid this:
async function bad_example():
    let result1 = await operation1()
    let result2 = await operation2()
    let result3 = await operation3()
    return [result1, result2, result3]

# Prefer this:
async function good_example():
    let promises = [operation1(), operation2(), operation3()]
    return await async.all(promises)
```

### 2. Use Timeouts Appropriately
Set reasonable timeouts based on expected operation duration.

### 3. Handle Memory Usage
Be mindful of creating too many promises simultaneously for large datasets.

## Limitations

### Current Limitations
1. No async generators or iterators
2. Limited promise cancellation support
3. No built-in async stream processing
4. No native async/await for loops (use regular loops with await)

### Future Enhancements
1. Async generators and iterators
2. Promise cancellation
3. Async stream processing
4. Worker threads for CPU-bound tasks
5. WebAssembly async integration

## Troubleshooting

### Common Issues

#### "await can only be used with promises"
```infra
# Error: await non_promise_value
async function bad():
    return await "not a promise"

# Fix: Don't await non-promise values
async function good():
    return "not a promise"
```

#### "Function not marked as async"
```infra
# Error: Using await in non-async function
function bad():
    return await async.create_promise("value")

# Fix: Mark function as async
async function good():
    return await async.create_promise("value")
```

#### "Promise rejected without error handling"
```infra
# Error: Unhandled rejection
async function bad():
    let rejected = await async.create_rejected_promise("error")
    return rejected  # This will panic

# Fix: Handle rejections
async function good():
    try:
        let result = await async.create_rejected_promise("error")
        return result
    catch error:
        return "Error handled: " + error
```

## Summary

Infra's async/await system provides:
- Clean, readable asynchronous code
- Comprehensive standard library functions
- Error handling with try/catch
- Promise utilities for complex workflows
- Event loop for non-blocking execution

With these tools, you can write efficient, modern asynchronous applications in Infra!