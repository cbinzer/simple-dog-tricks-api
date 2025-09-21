# Simple Dog Tricks API 🐕

A simple example REST API built with **Rust** and **[Axum](https://github.com/tokio-rs/axum)**.  
This project is part of a blog post that demonstrates how to build a backend in Rust step by step.

## Features

- Create, list, find, replace, and delete tricks
- Each trick has a name, description, and instructions
- In-memory storage using a repository pattern
- JSON input and output using [Serde](https://serde.rs/)

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)
- Cargo (comes with Rust)

### Run the server

```bash
cargo run
