# Data Race in Rust with Raw Pointers

This repository demonstrates a potential data race in Rust when using raw pointers without proper synchronization. The `bug.rs` file contains code that directly modifies a vector using a raw pointer within an `unsafe` block. This can lead to undefined behavior if the vector is accessed by another thread concurrently.

The `bugSolution.rs` file provides a solution to this issue using proper synchronization mechanisms to prevent data races and ensure data consistency.