# Mutable and Immutable Reference Conflict in Rust

This repository demonstrates a common error in Rust programming:  conflicting mutable and immutable references to the same data.

The `bug.rs` file contains code that attempts to create both mutable and immutable references to the same variable simultaneously.  This violates Rust's borrowing rules and results in a compile-time error.

The `bugSolution.rs` file shows how to correct this error by restructuring the code to avoid the conflict.