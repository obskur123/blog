# Rust for Beginners: Understanding Ownership

Rust is a systems programming language that emphasizes safety, concurrency, and performance. One of its most distinctive features is its ownership system. Let's explore this concept with some code examples.

## What is Ownership?

Ownership is Rust's unique approach to memory management. It allows Rust to make memory safety guarantees without needing a garbage collector. Here are the key rules:

1. Each value in Rust has a variable that's called its owner.
2. There can only be one owner at a time.
3. When the owner goes out of scope, the value will be dropped.

## Example: String Ownership

Let's look at a simple example of ownership with strings:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); // This line would cause an error
}
```