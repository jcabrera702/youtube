# Working with Parameters

Parameters in Rust must have explicitly defined types. This allows the compiler to ensure type safety throughout your program.

```rust

fn main() {
    // Calling functions with arguments
    tell_height(182);
    human_id("Joel", 55, 182.2);
}

// Function with one integer parameter
fn tell_height(height: i32) {
    println!("My height is {} cm.", height);
}

// Function with multiple parameters of different types
fn human_id(name: &str, age: u32, height: f32) {
    println!("Name: {}, Age: {}, Height: {} cm", name, age, height);
}
```

#  Statements vs. Expressions <br>

Understanding the difference between these two is critical for writing concise Rust code:
- Statements: Perform an action and end with a semicolon ;. They do not return a value.
- Expressions: Evaluate to a value and do not end with a semicolon when used as a return value.

```rust
fn main() {
    // Using a scope block as an expression
    let x: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty // No semicolon: this value is "returned" to x
    };
    println!("The result of the expression block is: {}", x);
}
```
# Return Values

Functions that return data must declare the return type after an arrow ->. You can return values implicitly (using an expression) or explicitly using the return keyword.

```rust
fn main() {
    let result = add(10, 20);
    println!("10 + 20 = {}", result);
    
    // You can also call functions directly inside print statements
    println!("Direct call result: {}", add(4, 6));
}

// The last line is an expression (no semicolon) which serves as the return value
fn add(a: i32, b: i32) -> i32 {
    a + b 
}
```

# Practical Implementation: BMI Calculator

This example combines multiple concepts: using f64 for precision, passing multiple parameters, and returning a calculated floating-point value.
```rust
fn main() {
    // 1. Data Setup (Weight in kg, Height in meters)
    let weight: f64 = 92.22; // Approx 203 lbs
    let height: f64 = 1.7272; // Approx 5'8"
    
    // 2. Function Call
    // If using VS Code with rust-analyzer, you will see grayed-out 
    // inlay hints like 'weight_kg:' and 'height_m:' here.
    let bmi = calculate_bmi(weight, height);
    
    // 3. Output with 2-decimal precision
    println!("Calculated BMI: {:.2}", bmi);
}

/// Calculates Body Mass Index (BMI)
/// Formula: weight (kg) / height^2 (m)
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    weight_kg / (height_m * height_m)
}
```