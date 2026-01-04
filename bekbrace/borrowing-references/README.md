# [Rust Basics: Borrowing & References](./src/main.rs)

This document summarizes the key lessons from **Rust Basics 2024 – Lesson 5** by **BekBrace**, focusing on Rust’s borrowing and reference system.

Rust’s ownership model is one of its most important features, enabling **memory safety without a garbage collector** while still achieving high performance.

---

## Overview

Rust prevents common bugs such as:
- Null pointer dereferencing
- Use-after-free errors
- Data races

It does this through a strict system of **ownership, borrowing, and references**, all enforced at compile time.

---

## Core Principles

### Ownership & Borrowing
- Every value in Rust has a single owner.
- Ownership can be transferred (moved) or temporarily borrowed.
- Borrowing allows access to data **without taking ownership**.

---

## Key Concepts

### References (`&`)
A reference lets you point to a value without owning it.

```rust
let x = 10;
let r = &x;
```
## Immutable References (&T)
- Allow read-only access to data
- Multiple immutable references are allowed at the same time
- No mutation is permitted
```rust
let x = 5;
let r1 = &x;
let r2 = &x; // OK
```
## Mutable References (&mut T)
- Allow modifying data
- Only one mutable reference is allowed at a time
- Cannot coexist with immutable references in the same scope
```rust
let mut x = 5;
let r = &mut x;
*r += 1;
```
## Dereferencing (*)
- Used to access or modify the value a reference points to.
```rust
let mut x = 5;
let r = &mut x;
*r += 1; // Dereference to modify value
```
## Underscore Prefix (_)
- Prefixing a variable with _ tells the compiler it is intentionally unused.
- This avoids compiler warnings.
```rust
let _unused_variable = 42;
```
## Code Implementation
- The following example demonstrates borrowing rules using a BankAccount struct with both immutable and mutable references.
### BankAccount Example
```rust
struct BankAccount {
    owner: String,
    balance: f64,
}

impl BankAccount {
    // Mutable borrow: allows modifying the balance
    fn withdraw(&mut self, amount: f64) {
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    // Immutable borrow: allows reading but not modifying data
    fn check_balance(&self) {
        println!(
            "Account owned by {} has a balance of {}",
            self.owner, self.balance
        );
    }
}

fn main() {
    // Basic Reference & Dereferencing
    let mut x = 5;
    let r = &mut x;     // Create mutable reference
    *r += 1;            // Dereference to increment the value
    println!("Value of x: {}", x); // x is now 6

    // Struct and Method Borrowing
    let mut account = BankAccount {
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    // 1. Immutable borrow to check balance
    account.check_balance();

    // 2. Mutable borrow to withdraw money
    account.withdraw(45.5);

    // 3. Immutable borrow to check final balance
    account.check_balance();
}

```
## Borrowing Rules in Action
- check_balance(&self) uses an immutable borrow
- withdraw(&mut self) uses a mutable borrow
- The compiler allows this because:
   - The immutable and mutable borrows occur in separate, non-overlapping scopes
   - No simultaneous mutable and immutable access exists
## Key Takeaways
- Rust enforces borrowing rules at compile time
- You can have:
    - Many immutable references OR
    - One mutable reference
- Dereferencing (*) is required to modify values behind references
- The underscore (_) prefix is a convention for unused variables
- Scope-based borrow checking prevents data races and undefined behavior
## Summary
Rust’s borrowing system may feel strict at first, but it guarantees:
- Memory safety
- Thread safety
- Zero runtime overhead <br>
By mastering references and borrowing, you unlock Rust’s full power while writing safe and predictable code.