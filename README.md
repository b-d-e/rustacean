# Rustacean 🦀

[![Rust](https://github.com/b-d-e/rustacean/actions/workflows/rust.yml/badge.svg)](https://github.com/b-d-e/rustacean/actions/workflows/rust.yml)

This repo is here to follow my progress learning the [Rust](https://www.rust-lang.org/) programming language.

It contains code for examples and exercises from [the Rust Book](https://doc.rust-lang.org/book/), split into folders corresponding to chapter numbers. Some bits might correspond to the [interactive version of the Rust Book too](https://rust-book.cs.brown.edu/).

I'm also working through exercises from [Rustlings](https://github.com/rust-lang/rustlings), roughly corresponding to the book order.

I'm writing these in vim (all being well), partially to force myself to not rely on VS Code's autocompletion & linting, and partially to finally memorise vim & tmux's keybindings.

![image](https://mir-s3-cdn-cf.behance.net/project_modules/disp/7df0bd42774743.57ee5f32bd76e.gif)

<sup>_'[Rustacean](https://www.behance.net/gallery/42774743/Rustacean)' animation by [Refracted Color](https://www.behance.net/refractedcolor) is licensed under [CC BY 4.0](https://creativecommons.org/licenses/by/4.0/)._</sup>

# Progress Tracker

## Rust Book

- [x] **1. [Getting Started](1/)**
  - [x] 1.1. Installation
  - [x] 1.2. [Hello, World!](book/1/hello_world/main.rs)
  - [x] 1.3. [Hello, Cargo!](book/1/hello_cargo/src/main.rs)
- [x] **2. [Programming a Guessing Game](book/2/guessing_game/src/main.rs)**
- [x] **3. Common Programming Concepts**
  - [x] 3.1. [Variables and Mutability](book/3/variables_and_mutability/src/main.rs)
  - [x] 3.2. [Data Types](book/3/data_types/src/main.rs)
  - [x] 3.3. [Functions](book/3/functions/src/main.rs)
  - [x] 3.4. [Comments](book/3/comments/src/main.rs)
  - [x] 3.5. [Control Flow](book/3/control_flow/src/main.rs)
- [ ] **4. Understanding Ownership**
  - [ ] 4.1. What is Ownership?
  - [ ] 4.2. References and Borrowing
  - [ ] 4.3. The Slice Type
- [ ] **5. Using Structs to Structure Related Data**
  - [ ] 5.1. Defining and Instantiating Structs
  - [ ] 5.2. An Example Program Using Structs
  - [ ] 5.3. Method Syntax
- [ ] **6. Enums and Pattern Matching**
  - [ ] 6.1. Defining an Enum
  - [ ] 6.2. The match Control Flow Construct
  - [ ] 6.3. Concise Control Flow with if let
- [ ] **7. Managing Growing Projects with Packages, Crates, and Modules**
  - [ ] 7.1. Packages and Crates
  - [ ] 7.2. Defining Modules to Control Scope and Privacy
  - [ ] 7.3. Paths for Referring to an Item in the Module Tree
  - [ ] 7.4. Bringing Paths Into Scope with the use Keyword
  - [ ] 7.5. Separating Modules into Different Files
- [ ] **8. Common Collections**
  - [ ] 8.1. Storing Lists of Values with Vectors
  - [ ] 8.2. Storing UTF-8 Encoded Text with Strings
  - [ ] 8.3. Storing Keys with Associated Values in Hash Maps
- [ ] **9. Error Handling**
  - [ ] 9.1. Unrecoverable Errors with panic!
  - [ ] 9.2. Recoverable Errors with Result
  - [ ] 9.3. To panic! or Not to panic!
- [ ] **10. Generic Types, Traits, and Lifetimes**
  - [ ] 10.1. Generic Data Types
  - [ ] 10.2. Traits: Defining Shared Behavior
  - [ ] 10.3. Validating References with Lifetimes
- [ ] **11. Writing Automated Tests**
  - [ ] 11.1. How to Write Tests
  - [ ] 11.2. Controlling How Tests Are Run
  - [ ] 11.3. Test Organization
- [ ] **12. An I/O Project: Building a Command Line Program**
  - [ ] 12.1. Accepting Command Line Arguments
  - [ ] 12.2. Reading a File
  - [ ] 12.3. Refactoring to Improve Modularity and Error Handling
  - [ ] 12.4. Developing the Library’s Functionality with Test Driven Development
  - [ ] 12.5. Working with Environment Variables
  - [ ] 12.6. Writing Error Messages to Standard Error Instead of Standard Output
- [ ] **13. Functional Language Features: Iterators and Closures**
  - [ ] 13.1. Closures: Anonymous Functions that Capture Their Environment
  - [ ] 13.2. Processing a Series of Items with Iterators
  - [ ] 13.3. Improving Our I/O Project
  - [ ] 13.4. Comparing Performance: Loops vs. Iterators
- [ ] **14. More about Cargo and Crates.io**
  - [ ] 14.1. Customizing Builds with Release Profiles
  - [ ] 14.2. Publishing a Crate to Crates.io
  - [ ] 14.3. Cargo Workspaces
  - [ ] 14.4. Installing Binaries from Crates.io with cargo install
  - [ ] 14.5. Extending Cargo with Custom Commands
- [ ] **15. Smart Pointers**
  - [ ] 15.1. Using Box<T> to Point to Data on the Heap
  - [ ] 15.2. Treating Smart Pointers Like Regular References with the Deref Trait
  - [ ] 15.3. Running Code on Cleanup with the Drop Trait
  - [ ] 15.4. Rc<T>, the Reference Counted Smart Pointer
  - [ ] 15.5. RefCell<T> and the Interior Mutability Pattern
  - [ ] 15.6. Reference Cycles Can Leak Memory
- [ ] **16. Fearless Concurrency**
  - [ ] 16.1. Using Threads to Run Code Simultaneously
  - [ ] 16.2. Using Message Passing to Transfer Data Between Threads
  - [ ] 16.3. Shared-State Concurrency
  - [ ] 16.4. Extensible Concurrency with the Sync and Send Traits
- [ ] **17. Object Oriented Programming Features of Rust**
  - [ ] 17.1. Characteristics of Object-Oriented Languages
  - [ ] 17.2. Using Trait Objects That Allow for Values of Different Types
  - [ ] 17.3. Implementing an Object-Oriented Design Pattern
- [ ] **18. Patterns and Matching**
  - [ ] 18.1. All the Places Patterns Can Be Used
  - [ ] 18.2. Refutability: Whether a Pattern Might Fail to Match
  - [ ] 18.3. Pattern Syntax
- [ ] **19. Advanced Features**
  - [ ] 19.1. Unsafe Rust
  - [ ] 19.2. Advanced Traits
  - [ ] 19.3. Advanced Types
  - [ ] 19.4. Advanced Functions and Closures
  - [ ] 19.5. Macros
- [ ] **20. Final Project: Building a Multithreaded Web Server**
  - [ ] 20.1. Building a Single-Threaded Web Server
  - [ ] 20.2. Turning Our Single-Threaded Server into a Multithreaded Server
  - [ ] 20.3. Graceful Shutdown and Cleanup

## Rustlings Exercises

- [ ] **0. Intro**
- [ ] **1. Variables**
- [ ] **2. Functions**
- [ ] **3. If**
- [ ] **4. Primitive Types**
- [ ] **5. Vecs**
- [ ] **6. Move Semantics**
- [ ] **7. Structs**
- [ ] **8. Enums**
- [ ] **9. Strings**
- [ ] **10. Modules**
- [ ] **11. Hashmaps**
- [ ] **12. Options**
- [ ] **13. Error Handling**
- [ ] **14. Generics**
- [ ] **15. Traits**
- [ ] **16. Lifetimes**
- [ ] **17. Tests**
- [ ] **18. Iterators**
- [ ] **19. Smart Pointers**
- [ ] **20. Threads**
- [ ] **21. Macros**
- [ ] **22. Clippy**
- [ ] **23. Conversions**
