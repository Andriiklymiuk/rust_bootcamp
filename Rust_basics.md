# Rust Programming Basics: A TypeScript Developer's Guide

Contents:
- [1. Hello World: Rust vs TypeScript](#1-hello-world-rust-vs-typescript)
- [2. Basic Types: Rust vs TypeScript](#2-basic-types-rust-vs-typescript)
- [3. Error Handling: Option and Result](#3-error-handling-option-and-result)
- [4. Structs and Implementations: Rust vs TypeScript](#4-structs-and-implementations-rust-vs-typescript)
- [5. Arrays and Loops](#5-arrays-and-loops)
- [6. Borrow Checker Basics](#6-borrow-checker-basics)
- [7. Basic Pointer Examples](#7-basic-pointer-examples)
- [8. &str vs String](#8-str-vs-string)
- [9. String Manipulation: Rust vs TypeScript](#9-string-manipulation-rust-vs-typescript)

## 1. Hello World: Rust vs TypeScript

### TypeScript
```typescript
function sayHello(): void {
    console.log("Hello, World!");
}

// or simply
console.log("Hello, World!");
```

### Rust
```rust
// main entrance to rust execution, like index.js in js
fn main() {
    println!("Hello, World!");
}

// Function with return type
fn say_hello() -> () {  // () is unit type, similar to void
    println!("Hello, World!");
}
```

</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>

## 2. Basic Types: Rust vs TypeScript

### TypeScript
```typescript
let number: number = 42;        // floating-point or integer
let float: number = 3.14;       // floating-point
let text: string = "hello";     // string
let isTrue: boolean = true;     // boolean
let maybe: null = null;         // null
let notDefined: undefined;      // undefined
let anything: any;              // any type
```

### Rust
```rust
let number: i32 = 42;          // 32-bit integer
let float: f64 = 3.14;         // 64-bit float
let string: String = String::from("hello"); // owned string
let text: &str = "hello";      // string slice
let is_true: bool = true;      // boolean
let maybe: Option<i32> = None; // Optional value
// Rust has no undefined or null!
// No "any" type - Rust is strictly typed
```

Common Rust number types:
- `i8, i16, i32, i64, i128` - signed integers (it can be positive or negative)
- `u8, u16, u32, u64, u128` - unsigned integers
- `f32, f64` - floating point numbers

</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>


## 3. Error Handling: Option and Result

### TypeScript Error Handling
```typescript
// TypeScript - Optional values
let maybeNumber: number | null = null;
let maybeString: string | undefined;

// TypeScript - Error handling
try {
    throw new Error("Something went wrong");
} catch (error) {
    console.error(error);
}

// TypeScript - Return type with possible error
function divide(a: number, b: number): number | Error {
    if (b === 0) return new Error("Division by zero");
    return a / b;
}
```

</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>

### Typescript with ts-result library
```typescript
import { Result, Ok, Err, Option, Some, None } from 'ts-results-es';

// Optional values using Option
const maybeNumber: Option<number> = new None();
const maybeString: Option<string> = new None();

// Error handling using Result
function parseJson<T>(json: string): Result<T, string> {
    try {
        const result = JSON.parse(json);
        return new Ok(result);
    } catch (error) {
        return new Err(`Failed to parse JSON: ${error}`);
    }
}

// Function that might fail using Result
function divide(a: number, b: number): Result<number, string> {
    if (b === 0) return new Err("Division by zero");
    return new Ok(a / b);
}

// Usage example
const result = divide(10, 2);
if (result.isOk()) {
    console.log("Result:", result.value);
}
if (result.isErr()) {
    console.log("Error:", result.error);
}

```
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>

### Rust Error Handling
```rust
// Option - for values that might not exist
let maybe_number: Option<i32> = None;
let definitely_number: Option<i32> = Some(5);

// Result - for operations that might fail
let result: Result<i32, String> = Ok(42);
let error_result: Result<i32, String> = Err("Something went wrong".to_string());

// Practical example
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Division by zero".to_string());
    }
    Ok(a / b)
}

// Using match with Result
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(error) => println!("Error: {}", error),
}

// Using ? operator for error propagation
fn complicated_operation() -> Result<i32, String> {
    let result = divide(10, 2)?;  // ? will return Err if divide fails
    Ok(result * 2)
}
```
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>
</br>

## 4. Structs and Implementations: Rust vs TypeScript

```typescript
// TypeScript Class
class Person {
    name: string;
    age: number;

    constructor(name: string, age: number) {
        this.name = name;
        this.age = age;
    }

    sayHello(): void {
        console.log(`Hello, I'm ${this.name}`);
    }
}
```

```rust
// Rust Struct and Impl
struct Person {
    name: String,
    age: u32,
}

impl Person {
    // Constructor (by convention called new)
    fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }

    // Method
    fn say_hello(&self) {
        println!("Hello, I'm {}", self.name);
    }
}

// Usage
let person = Person::new(String::from("Alice"), 30);
person.say_hello();
```

### Pattern Matching
```rust
let number = 42;

match number {
    0 => println!("Zero"),
    1..=50 => println!("Between 1 and 50"),
    _ => println!("Something else"),
}
```

### Enums
```rust
enum Status {
    Active,
    Inactive,
    Pending,
}

let status = Status::Active;
match status {
    Status::Active => println!("Active"),
    Status::Inactive => println!("Inactive"),
    Status::Pending => println!("Pending"),
}
```

### Traits (similar to interfaces)
```rust
trait Animal {
    fn make_sound(&self) -> String;
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) -> String {
        "Woof!".to_string()
    }
}
```


## 5. Arrays and Loops

### TypeScript Arrays and Loops
```typescript
// Array
let numbers: number[] = [1, 2, 3, 4, 5];

// For loop
for (let i = 0; i < numbers.length; i++) {
    console.log(numbers[i]);
}

// For-of loop
for (const num of numbers) {
    console.log(num);
}

// forEach
numbers.forEach(num => console.log(num));
```

### Rust Arrays and Loops
```rust
// Fixed-size array
let numbers: [i32; 5] = [1, 2, 3, 4, 5];

// Vector (dynamic array)
let mut vector: Vec<i32> = vec![1, 2, 3, 4, 5];

// For loop
for num in numbers.iter() {
    println!("{}", num);
}

// Range-based loop
for i in 0..5 {
    println!("{}", i);
}

// While loop
let mut i = 0;
while i < 5 {
    println!("{}", i);
    i += 1;
}
```

## 6. Borrow Checker Basics

```rust
fn main() {
    let mut s = String::from("hello");
    
    // Borrow immutably (multiple allowed)
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    
    // Borrow mutably (only one allowed)
    let r3 = &mut s;
    r3.push_str(" world");
    println!("{}", r3);
    
    // Can't use r1 or r2 here as s was mutably borrowed
    // println!("{}", r1); // This would not compile!
}
```

## 7. Basic Pointer Examples

```rust
fn main() {
    let mut x = 42;
    
    // Reference (&) - immutable borrow
    let ref_x = &x;
    println!("Reference value: {}", ref_x);
    
    // Mutable reference (&mut) - mutable borrow
    let mut_ref_x = &mut x;
    *mut_ref_x = 43;  // Dereference and modify
    println!("Modified value: {}", x);
}
```

## 8. &str vs String

```rust
// &str - string slice, immutable, borrowed
let slice: &str = "Hello";

// String - owned, mutable, growable
let string: String = String::from("Hello");
let also_string = "Hello".to_string();

// Convert String to &str
let slice_from_string: &str = &string;

// String methods
let mut s = String::from("Hello");
s.push_str(" World");  // Only String can be modified
```

## 9. String Manipulation: Rust vs TypeScript

```typescript
// TypeScript string manipulation
let str = "Hello";
str = str + " World";              // Concatenation
str = str.toLowerCase();           // Convert to lowercase
let length = str.length;           // Get length
let char = str[0];                 // Get character
let sub = str.substring(0, 5);     // Get substring
```

```rust
// Rust string manipulation
let mut string = String::from("Hello");
string.push_str(" World");         // Concatenation
string = string.to_lowercase();    // Convert to lowercase
let length = string.len();         // Get length in bytes
let char = string.chars().nth(0);  // Get character (returns Option)
let sub = &string[0..5];          // Get substring (be careful with UTF-8!)
```



Remember:
- Rust is memory safe without garbage collection
- No null values - use Option instead
- No exceptions - use Result for error handling
- Strong type system with type inference
- Ownership system prevents memory issues
- Pattern matching is powerful and widely used