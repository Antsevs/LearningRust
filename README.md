# Learning Rust 🦀

This repository documents my journey of learning the Rust programming language, following along with [*The Rust Programming Language Book*](https://doc.rust-lang.org/book/). I try my best to avoid the assistance of generative AI, such that I can develop a strong and foundational understanding of the language. Many of my scripts are riddled with notes, please feel free to take a read!

---
![Rust Image](https://miro.medium.com/v2/resize:fit:709/0*Eqqrv9zVpH99X726.png)
---

## 📂 Project Structure

```
LearningRust/
│
├── fundamentals/                # Core Rust concepts
│   ├── control_flow/            # if, else, loops (from Ch.3.5 Control Flow)
│   ├── data_types/              # scalar & compound types
│   ├── functions/               # defining and calling functions
│   ├── hello_cargo/             # introduction to Cargo package manager
│   ├── hello_world/             # first Rust program
│   ├── practice/                # exercises & challenges
│   └── variables_and_mutability/# variables, immutability, shadowing
│
├── guessing_game/               # Ch.1 Project example
│
└── README.md                    # this file
```

---

## ⚙️ Development Environment

- **Compiler:** `rustc 1.89.0`
- **Cargo:** `cargo 1.89.0`
- **Editor:** VS Code with [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) (v0.3.2593)
- **Version Control:** Git `2.43.0.windows.1`
- **OS:** Windows (Git Bash for terminal)

---

## 🚀 Running Code

Each folder is a self-contained Rust project managed by Cargo.

1. Navigate into a project:
   ```bash
   cd fundamentals/control_flow
   ```

2. Build and run:
   ```bash
   cargo run
   ```

---

## 📖 Topics Covered So Far

### Hello World
First Rust program using `println!`.

```bash
$ cargo run
   Compiling hello_world v0.1.0 (file:///projects/hello_world)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running `target/debug/hello_world`
Hello, world!
```

---

### Cargo
Introduction to `cargo` for project management.

```bash
$ cargo run
   Compiling hello_cargo v0.1.0 (file:///projects/hello_cargo)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/hello_cargo`
Hello, world!
```

---

### Variables & Mutability
Immutable by default, use `mut` to allow mutation.

```bash
$ cargo run
   Compiling variables_and_mutability v0.1.0 (file:///projects/variables_and_mutability)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.24s
     Running `target/debug/variables_and_mutability`
The value of x is: 5
The value of x is: 6
```

---

### Data Types
Covers scalar and compound types (integers, floats, booleans, tuples, arrays).

```bash
$ cargo run
   Compiling data_types v0.1.0 (file:///projects/data_types)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/data_types`
The value of y is: 3.2
The value of z is: 4.5
```

---

### Functions
Defining and calling functions, parameters, return values.

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running `target/debug/functions`
Hello, world!
Another function.
The value of x is: 5
```

---

### Control Flow
Using `if`, `else if`, `else`, `loop`, `while`, and `for` loops.

```bash
$ cargo run
   Compiling control_flow v0.1.0 (file:///projects/control_flow)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/control_flow`
condition was true
```

Example loop with a result:

```bash
$ cargo run
   Compiling control_flow v0.1.0 (file:///projects/control_flow)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/control_flow`
The result is 20
```

Example countdown:

```bash
$ cargo run
   Compiling control_flow v0.1.0 (file:///projects/control_flow)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/control_flow`
3!
2!
1!
LIFTOFF!!!
```

---

### Practice Projects
- Convert temperatures between Fahrenheit and Celsius
- Generate the nth Fibonacci number
- Print the lyrics to “The Twelve Days of Christmas”

---

## 📝 References

- [The Rust Programming Language Book](https://doc.rust-lang.org/book/)
- [Control Flow (Ch.3.5)](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

---

## 🎯 Next Steps

- Chapter 4: Ownership
- Chapter 5: Structs
- Continue expanding `practice/` with coding challenges
