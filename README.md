# brainrust

brainrust is a rust-based interpreter for the brainf\*\*\* programming language.
it features a full implementation, encompassing efficient parsing, operation
compression, and execution in a virtual machine environment.

## Overview of modules

### 'parser.rs'

- function: parses brainf\*\*\* source code into operations.
- details: utilizes the nom parsing library to translate brainf\*\*\* code into
  a sequence of operations. importantly, it compresses consecutive similar
  operations into single operations to increase memory and compute efficiency.

### 'ops.rs'

- function: defines the operations used in the brainf\*\*\* language.
- details: enumerates all brainf\*\*\* operations, such as increment, decrement,
  and loop controls. essential for both the parsing and execution stages.

### 'vm.rs'

- function: executes the brainf\*\*\* operations.
- details: implements a virtual machine using rust's typestate pattern, ensuring
  operations are executed only when the vm is correctly configured. this design
  showcases advanced rust programming and api design, with a focus on safety and
  efficiency.

### 'driver.rs'

- function: drives the overall execution of the interpreter.
- details: integrates the parser and vm, managing the execution flow and error
  handling. ensures seamless operation of the interpreter from start to finish.

### 'main.rs'

- function: entry point of the application.
- details: handles command-line argument parsing and initiates the
  interpretation process, linking all modules into a unified application.

## Installation

```
git clone https://github.com/dbolivar25/brainrust.git
cd brainrust
```

## Usage

### REPL Interpreter

```
cargo run
```

### File Interpreter

```
cargo run -- -f <your_file_name>
```
