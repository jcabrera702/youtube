# 🦀 Rust Lesson: Compound Data Types

Compound types group multiple values into one type. Rust has two primitive compound types: **Tuples** and **Arrays**, plus the highly useful **Slices**.

## 1. Arrays `[T; N]`
Arrays have a **fixed length** known at compile time. All elements must be of the same type.
* **Initialization:** `let fruits: [&str; 3] = ["Apple", "Banana", "Cherry"];`
* **Accessing Elements:** Use zero-based indexing like `fruits[0]` or `fruits[1]`.
* **Use Case:** Best for small, fixed lists of data (e.g., months of the year).

## 2. Tuples `(T1, T2, ...)`
Tuples group together multiple values of **different types**.
* **Initialization:** ```rust
  let human: (String, i32, bool) = ("Alice".to_string(), 30, false);

## 3. Slices &[T]
A slice is a reference to a contiguous sequence of elements in a collection rather than the whole collection.
* **Key Concept:** Slices do not own the data; they are "views" into memory.
* Initialization: 
  let number_slices: &[i32] = &[1, 2, 3, 4, 5];
* Key Concept: Slices do not own the data; they are "views" into memory.
* Printing: Use the debug formatter {:?} to print slices: println!("{:?}", number_slices);

## 4. String Types & Memory
Rust distinguishes between owned strings and string references, which impacts where they are stored.

### Owned String (`String`)
* **Storage:** The actual character data is stored on the **Heap**.
* **Characteristics:** Growable, mutable, and owned by the variable.
* **Usage:** `let s = String::from("Hello");` or `"Hello".to_string();`

### String Slice (`&str`)
* **Storage:** The slice (pointer and length) is stored on the **Stack**. 
* **Characteristics:** An immutable reference to string data. The data it points to can be in the program binary (literals) or on the heap.
* **Usage:** `let slice: &str = "Hello";`

### Summary Table
| Type | Name | Data Location | Size |
| :--- | :--- | :--- | :--- |
| `String` | Owned String | Heap | Dynamic |
| `&str` | String Slice | Stack (points to data) | Fixed (Pointer + Length) |

---