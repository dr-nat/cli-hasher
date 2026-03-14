# Hash CLI

A light weight command line interface (CLI) tool  built in rust for generating cryptographically secure SHA-256 hashes from string input.

## Description 
Hash CLI is designed for developers who need a quick way to generate checksums or unique identifiers from the terminal. It takes a string argument, processes it through the SHA-256 algorithm, and outputs a standard 64-character hexadecimal string.

## Features
* **Standard SHA-256: ** Uses the industry standard secure Hash Algorithm 2.
* **Documentation-First Logic: **Built with a focus on core systems programming principles.
* **Hexadecimal Output: ** Built with a focus on core systems programming principles.

## Prerequisites
To build and run this tool, you must have the following installed:
* **Rust Toolchain** (rustc, cargo)
* **Ubuntu/Linux environment**

## Installation
1. **Clone the repository:**
   `git clone https://github.com/your-username/hash_cli.git`
2. **Navigate to the project root:**
   `cd hash_cli`
3. **Build the project:**
   `cargo build --release`

## Usage
Run the program by passing a string as an argument:

```bash
cargo run <your_input_string>
