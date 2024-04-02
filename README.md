# Rust Fundamentals

This repository is dedicated to learning and mastering Rust, a systems programming language that runs blazingly fast, prevents segfaults, and guarantees thread safety. Here you'll find commands, key concepts, and resources to get you started with Rust.

## Getting Started with Cargo

Cargo is Rust's build system and package manager. Here are some basic commands to get you started:

- **Create a new project**: `cargo new <project_name>`
- **Build your project**: `cargo build`
- **Run your project**: `cargo run`
- **Check for errors without building**: `cargo check`
- **Create a library package**: `cargo init --lib .`

## Key Terms in Rust

- **Struct**: Organize similar data in a structure, akin to objects in JavaScript or Python dictionaries.
- **Field**: Values within a struct, such as a person's first and last name.
- **Debug**: An attribute for printing the whole struct instead of specific fields.
- **Type**: The kind of value a field can hold, e.g., `String` or `u8`.
- **Instance**: A created struct with data in its fields.
- **Option**: Represents either a value of a specific type or the absence of a value (`None`).
- **Implementation (`impl`)**: Used to extend structs or enums with additional functions.
- **Associated Function**: Functions associated with a struct or enum, like constructors.
- **Constructor**: A function for creating an instance of a struct with predefined fields.
- **Immutable**: Cannot be changed once initialized. Rust variables are immutable by default.
- **String**: A growable, mutable sequence of characters. Rust has two types: slices (`&str`) and owned strings (`String`).
- **Vector (`Vec<T>`)**: A growable array of elements of type `T`.
- **Borrowing**: Temporarily accessing data without taking ownership, crucial for memory safety.
- **Ownership**: A value can have only one owner at a time. Transferring ownership frees the previous owner from the value.
- **Slice (`[T]`)**: A view into a sequence of elements without owning them.
- **Mutable References**: Allows for data modification, but Rust enforces a single mutable reference to ensure safety.
- **Enum**: Defines a set of possible values (variants) for a type, with exhaustive matching via the `match` keyword.
- **Variant**: A specific option within an enum that can contain data.
- **Match Keyword**: Used for pattern matching against enums, allowing for handling of various cases.
- **Sum Type**: An enum that can hold different types or combinations of data within its variants.

## Learning Resources

To dive deeper into Rust, here are some invaluable resources:

- **[The Rust Book](https://doc.rust-lang.org/nightly/book/)**: Comprehensive guide covering Rust in depth.
- **[Rustlings](https://github.com/rust-lang/rustlings)**: Small exercises for practicing Rust syntax.
- **[Exercism Rust Track](https://exercism.org/tracks/rust)**: Practice Rust by solving exercises.
- **[Learning Rust](https://learning-rust.github.io/)**: Additional learning materials and examples.
- **[This Week In Rust](https://this-week-in-rust.org/)**: Weekly newsletter about the Rust ecosystem.
- **[Rust By Example](https://doc.rust-lang.org/stable/rust-by-example/)**: Learn Rust through annotated example programs.
- **[Rust First Steps by Microsoft](https://docs.microsoft.com/en-gb/learn/paths/rust-first-steps/)**: A beginner's tutorial to Rust.
- **[Tour of Rust](https://tourofrust.com/)**: Step-by-step guide through Rust features.
- **[The Little Book of Rust Macros](https://veykril.github.io/tlborm/)**: Detailed guide on Rust macros.
- **[Learn Rust](https://www.rust-lang.org/learn)**: Index of Rust's documentation, including the Edition Guide and Cargo Book.
- **[Awesome Rust](https://github.com/kud1ing/awesome-rust)**: A curated list of Rust libraries and resources.
- **[Rust FFI Omnibus](https://jakegoulding.com/rust-ffi-omnibus/)**: Guide on using Rust with other languages.
- **[Rust Cheatsheet](https://cheats.rs/)**: Quick reference for common Rust concepts.

Explore these resources to build your understanding and expertise in Rust. Happy coding!
