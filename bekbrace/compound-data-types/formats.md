# Rust Formatting: `{}` vs `{:?}`

## 1. `{}` — The Display Trait
* **Purpose:** Intended for **user-facing** output.
* **Formatting:** Provides a "pretty" or natural representation of the data.
* **Requirements:** The type must implement the `std::fmt::Display` trait.
* **Common Use:** Printing simple types like integers, floats, and strings.
* **Limitation:** Complex types like **arrays**, **vectors**, and **custom structs** do not implement `Display` by default. Using them with `{}` will cause a compiler error.

## 2. `{:?}` — The Debug Trait
* **Purpose:** Intended for **developer-facing** output (debugging).
* **Formatting:** Shows the internal structure and technical details of the data.
* **Requirements:** The type must implement or derive the `std::fmt::Debug` trait.
* **Common Use:** Inspecting **arrays**, **tuples**, **enums**, or any collection during development.
* **Pro Tip:** Use `{:#?}` for "pretty-print" debug output, which adds indentation and newlines to complex structures.

---

## Comparison Summary

| Feature | `{}` (Display) | `{:?}` (Debug) |
| :--- | :--- | :--- |
| **Audience** | End-users | Developers |
| **Default Types** | `i32`, `f64`, `String`, `&str` | Almost all types |
| **Custom Structs** | Must manually implement | Can use `#[derive(Debug)]` |
| **Collections** | Generally not supported | Supported (Arrays, Tuples, etc.) |

---

## Code Example
```rust
fn main() {
    let list = [1, 2, 3];
    
    // This would fail to compile:
    // println!("{}", list); 

    // This works:
    println!("{:?}", list); // Output: [1, 2, 3]
    
    // Pretty-print version:
    println!("{:#?}", list);
}
```

## Code Examples from Lesson 2

### Simple Tuple Example (`human`)
A tuple containing a `String`, an integer (`i32`), and a boolean (`bool`) requires the `{:?}` formatter.
```rust
let human: (String, i32, bool) = ("Alice".to_string(), 30, false);

// This works because {:?} handles tuples:
println!("Human Tuple: {:?}", human); 
// Output: Human Tuple: ("Alice", 30, false)
```
### Complex/Mixed Tuple Example (my_mix_tuple)
The debug formatter can even handle "mixed" tuples that contain other compound types like an array.
```rust
let my_mix_tuple = ("Kratos", 23, true, [1, 2, 3, 4, 5]);

// This outputs the entire structure, including the inner array:
println!("My Mix Tuple: {:?}", my_mix_tuple);
// Output: My Mix Tuple: ("Kratos", 23, true, [1, 2, 3, 4, 5])
```