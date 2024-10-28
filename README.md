[![CI](https://github.com/nogibjj/Jay_Liu_week7_rust/actions/workflows/ci.yml/badge.svg)](https://github.com/nogibjj/Jay_Liu_week7_rust/actions/workflows/ci.yml)
# JAY_LIU_WEEK7_RUST - Command-Line Tool in Rust



## Project Purpose

This project showcases the development of a command-line tool using the Rust programming language. The tool is designed to process input files and optionally generate output files based on user-specified parameters. It demonstrates command-line argument parsing, error handling, and efficient execution in Rust.


## Project Structure

The project is organized as follows:

bash
Copy code
JAY_LIU_WEEK7_RUST/
├── Cargo.toml             # Rust project manifest file (dependencies and metadata)
├── src/
│   └── main.rs            # Main Rust source code (tool logic and command-line argument parsing)
├── .github/
│   └── workflows/
│       └── ci.yml         # CI/CD pipeline configuration using GitHub Actions
├── README.md              # Project documentation (this file)
├── LICENSE                # Project license (MIT License)
└── .gitignore             # Specifies files and directories to ignore in version control

## File Descriptions:
### Cargo.toml: 
Contains the project's dependencies and metadata. It defines the crate name, version, authors, and external crates used, such as clap for command-line argument parsing.

### src/main.rs: 
The main Rust source file implementing the tool's functionality. It includes the logic for processing input files and handling command-line arguments.

### .github/workflows/ci.yml: 
Configuration file for the CI/CD pipeline using GitHub Actions. It automates building, testing, and releasing the tool, and uploads the compiled binary as an artifact.

### README.md: 
Provides an overview of the project, installation instructions, usage examples, and other essential documentation.

### LICENSE: 
Specifies the license under which the project is distributed (MIT License).

### .gitignore: 
Lists files and directories to be ignored by Git, such as build artifacts and temporary files.

## Command-Line Tool Functionality
The command-line tool processes input files provided by the user and performs specified operations. It supports optional output file generation and provides helpful command-line options for flexibility.

## Available Options:
--input <INPUT>: Specifies the input file to process (required).
--output <OUTPUT>: Specifies the optional output file.
--help: Displays help information about the tool.
--version: Displays the tool's version information.
## Command Syntax:
bash
Copy code
./jay_liu_week7_rust --input <INPUT> [--output <OUTPUT>]
<INPUT>: The path to the input file to be processed.
<OUTPUT>: (Optional) The path where the output will be saved.

## Example Usage
### Processing an Input File:
bash
Copy code
./jay_liu_week7_rust --input data/input.txt
### Output:
lua
Copy code
Processing input file: data/input.txt
No output file specified.
Operation completed successfully.
### Processing with an Output File:
bash
Copy code
./jay_liu_week7_rust --input data/input.txt --output results/output.txt
### Output:
lua
Copy code
Processing input file: data/input.txt
Output will be saved to: results/output.txt
Operation completed successfully.
## Installation

### Prerequisites

- **Rust and Cargo**: [Install Rust](https://www.rust-lang.org/tools/install)

### Building from Source

```bash
git clone https://github.com/nogibjj/Jay_Liu_week7_rust.git
cd JAY_LIU_WEEK7_RUST
cargo build --release

