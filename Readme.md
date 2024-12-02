# Rust Programming Basics: A TypeScript Developer's Guide

## Install
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo --version
```

Contents:
- [1. Hello World: Rust vs TypeScript](#1-hello-world-rust-vs-typescript)
- [2. String Manipulation: Rust vs TypeScript](#2-string-manipulation)
- [3. Basic Types: Rust vs TypeScript](#3-basic-types-rust-vs-typescript)
- [4. Pattern Matching](#4-pattern-matching)
- [5. Enums](#5-enums)
- [6. Error Handling: Option and Result](#6-error-handling-option-and-result)
- [7. Structs and Implementations: Rust vs TypeScript](#7-structs-and-implementations-rust-vs-typescript)
- [8. Traits (inheritance)](#8-traits-inheritance)
- [9. Arrays and Loops](#9-arrays-and-loops)
- [10. Borrow Checker Basics](#10-borrow-checker-basics)
- [11. Basic Pointer Examples](#11-basic-pointer-examples)
- [12. &str vs String](#12-str-vs-string)
- [Why](#why-bother)

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

fn return_hello() -> String {
    "Hello, World!".to_string()
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

## 2. String Manipulation

## TypeScript 
```typescript
let str = "Hello";
str = str + " World";              // Concatenation
str = str.toLowerCase();           // Convert to lowercase
let length = str.length;           // Get length
let char = str[0];                 // Get character
let sub = str.substring(0, 5);     // Get substring
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

### Rust
```rust
let mut string = String::from("Hello");
string.push_str(" World");         // Concatenation
string = string.to_lowercase();    // Convert to lowercase
let length = string.len();         // Get length in bytes
let char = string.chars().nth(0);  // Get character (returns Option, see error handling)
// Not safe: for just english letters or symbols
let sub = &string[0..5];         
// Safe: work with chars
let first_two: String = &string.chars().take(5).collect();
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

## 3. Basic Types: Rust vs TypeScript

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

## 4. Pattern Matching
```rust
let number = 42;

match number {
    0 => println!("Zero"),
    1..=50 => println!("Between 1 and 50"),
    _ => println!("Something else"),
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

### TypeScript
```typescript
const number = 42;

// Basic switching
switch (number) {
    case 0:
        console.log("Zero");
        break;
    default:
        if (number >= 1 && number <= 50) {
            console.log("Between 1 and 50");
        } else {
            console.log("Something else");
        }
}

// More modern approach using object literals
type Pattern = { [key: string]: (n: number) => void };

const patterns: Pattern = {
    isZero: (n) => n === 0 && console.log("Zero"),
    isBetween1And50: (n) => n >= 1 && n <= 50 && console.log("Between 1 and 50"),
    default: () => console.log("Something else")
};

// Execute patterns
if (patterns.isZero(number)) {}
else if (patterns.isBetween1And50(number)) {}
else patterns.default();
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

### 5. Enums
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

enum Option<T> {
    Some(T),    // Some is a variant that holds a value
    None        // None is a variant with no value
}
let some_number: Option<i32> = Some(5);
let no_number: Option<i32> = None;

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

## 6. Error Handling: Option and Result

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

### Rust Error Handling
```rust


// Option - for values that might not exist
enum Option<T> {
    Some(T),
    None,
}
let maybe_number: Option<i32> = None;
let definitely_number: Option<i32> = Some(5);


// Result - for operations that might fail
enum Result<T, E> {
    Ok(T),
    Err(E),
}
let result: Result<i32, String> = Ok(42);
let error_result: Result<i32, String> = Err("Something went wrong".to_string());

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        return Err("Division by zero".to_string());
    }
    Ok(a / b)
}















// 1. Using match
match divide(10, 2) {
    Ok(result) => println!("Result: {}", result),
    Err(error) => println!("Error: {}", error),
}









// 2. Using is_ok() and is_err()
let result = divide(10, 2);
if result.is_ok() {
    println!("Result: {}", result.unwrap());
}
if result.is_err() {
    println!("Error: {}", result.unwrap_err());
}






// 3. Using if let (when you care only about Ok)
if let Ok(result) = divide(10, 2) {
    println!("Result: {}", result);
}





// 4. Using unwrap_or (provides default value if error)
let result = divide(10, 0).unwrap_or(0);
println!("Result with default: {}", result);





// 5. Using expect (panic with custom message if error)
let result = divide(10, 2).expect("Division failed");
println!("Result: {}", result);






// 6. Using map (transform success value)
let result = divide(10, 2)
    .map(|x| x * 2);  // multiply by 2 if Ok





// 7. Using map_err (transform error value)
let result = divide(10, 0)
    .map_err(|e| format!("Error occurred: {}", e));





// 8. Using and_then (chain operations)
let result = divide(10, 2)
    .and_then(|x| divide(x, 2));  // divide result by 2





// 9. Using the ? operator in a function
fn complex_operation() -> Result<i32, String> {
    let x = divide(10, 2)?;  // returns Err if divide fails
    let y = divide(x, 2)?;   // otherwise continues
    Ok(y + 1)
}






// Using ? operator for error propagation
fn complicated_operation() -> Result<i32, String> {
    let result = divide(10, 2)?;  // ? will return Err if divide fails
    Ok(result * 2)
}





// r# is raw string literal, without it it would be
// "{\"name\": \"John\", \"age\": 30}
let json_str = r#"{"name": "John", "age": 30}"#;
    
// Parse into a JsonValue
let parsed = json::parse(json_str);

// json::parse returns Result automatically
match parsed {
    Ok(data) => println!("Name: {}", data["name"]),
    Err(error) => println!("Failed to parse: {}", error)
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

### Typescript with ts-result library (runtime check)
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

## 7. Structs and Implementations: Rust vs TypeScript
### TypeScript Class
```typescript

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
const person = new Person("Alice", 30);
person.sayHello(); 
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

### Rust Struct and Impl
```rust

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

### 8. Traits (inheritance)

### Typescript interface inheritence
```typescript
interface Animal {
    makeSound(): string;
}

class Dog implements Animal {
    name: string;

    constructor(name: string) {
        this.name = name;
    }

    makeSound(): string {
        return "Woof!";
    }
}

const dog = new Dog("Rex");
console.log(dog.makeSound());
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

### Rust trait

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

Dog.make_sound()
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

## 9. Arrays and Loops

### TypeScript
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

### Rust
```rust
// Arrays - Fixed size
let arr1: [i32; 5] = [1, 2, 3, 4, 5];         // Standard declaration
let arr2 = [3; 5];                            // [3, 3, 3, 3, 3]
let arr3: [i32; 5] = Default::default();      // [0, 0, 0, 0, 0]

// Vectors - Dynamic size
// Creation methods
let vec1: Vec<i32> = vec![1, 2, 3, 4, 5];    // Using vec! macro
let vec2 = Vec::new();                        // Empty vector
let mut vec3 = Vec::with_capacity(10);        // Preallocate space
let vec4 = (0..5).collect::<Vec<i32>>();      // From range
// Vector from array
let vec6 = vec1.to_vec();


// Adding elements to vector
let mut vec5 = Vec::new(); // or vec![]
vec5.push(1);
vec5.push(2);






// Common iterators examples
let numbers = vec![1, 2, 3, 4, 5];

// Map
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
// [2, 4, 6, 8, 10]

// Filter
let even: Vec<i32> = numbers.iter().filter(|x| *x % 2 == 0).collect();
// [2, 4]

// Find
let found = numbers.iter().find(|&&x| x > 3);
// Some(4)

// Any/All
let has_even = numbers.iter().any(|x| x % 2 == 0);    // true
let all_even = numbers.iter().all(|x| x % 2 == 0);    // false

// Fold (reduce)
let sum = numbers.iter().fold(0, |acc, x| acc + x);   // 15

// Chaining iterators
let result: Vec<i32> = numbers
    .iter()
    .filter(|x| *x % 2 == 0)  // keep even numbers
    .map(|x| x * 2)           // multiply by 2
    .collect();               // collect into vector

// Enumerate
for (index, value) in numbers.iter().enumerate() {
    println!("Index: {}, Value: {}", index, value);
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

## 10. Borrow Checker Basics

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

## 11. Basic Pointer Examples

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

## 12. &str vs String

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

### How they stored in memory
```
// &str (string slice)
Stack               Binary/Heap
[ptr|len] -------> ["Hello"]

// String
Stack               Heap
[ptr|len|cap] ---> ["Hello"]

// &String
Stack               Stack               Heap
[ptr] -----------> [ptr|len|cap] ---> ["Hello"]

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




## Why bother?
- Rust is memory safe without garbage collection
- No null values - use Option instead
- No exceptions - use Result for error handling
- Strong type system with type inference
- Ownership system prevents memory issues
- Pattern matching is powerful and widely used
