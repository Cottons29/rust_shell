# rust_shell

A lightweight shell implementation written in Rust. This project provides a custom command-line interface with basic shell functionality.

## Project Objective

The goal of this project is to create a functional shell environment using Rust, demonstrating systems programming concepts while providing a usable alternative to traditional shells. The shell supports basic command execution, directory navigation, and extensibility.

## Features

- Custom command parsing and execution
- Built-in commands for file system navigation
- Colorized output using the `colored` crate
- Terminal handling with `crossterm`
- Debug mode for development purposes

## Getting Started

### Prerequisites

- Rust 1.87.0 or later
- Cargo package manager

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/rust_shell.git
   cd rust_shell
   ```

2. Build the project:
   ```bash
   cargo build --release
   ```

3. Run the shell:
   ```bash
   ./target/release/shell
   ```

   For debug mode:
   ```bash
   ./target/release/shell --debug
   ```

## Adding to PATH

To make the shell easily accessible from anywhere in your system, you can add it to your PATH:

### For Linux/macOS

1. Create a symbolic link to a directory in your PATH:
   ```bash
   sudo ln -s $(pwd)/target/release/shell /usr/local/bin/cotsh
   ```

   Or add the following to your `~/.bashrc` or `~/.zshrc`:
   ```bash
   export PATH="$PATH:/path/to/rust_shell/target/release"
   ```

2. Reload your shell configuration:
   ```bash
   source ~/.bashrc  # or source ~/.zshrc
   ```

### For Windows

1. Add the full path to the `target\release` directory to your system PATH environment variable.
2. Restart your command prompt or PowerShell session.

## Usage

Once the shell is running, you can use it like any other shell:

```bash
$ ls                # List files in current directory
$ cd Documents      # Change directory
$ exit              # Exit the shell (or exit 1 to specify exit code)
```

## Contributing

Contributions to rust_shell are welcome! Here's how you can contribute:

1. Fork the repository
2. Create a new branch for your feature or bugfix:
   ```bash
   git checkout -b feature/your-feature-name
   ```
3. Make your changes
4. Write or update tests as necessary
5. Run tests to ensure everything is working:
   ```bash
   cargo test
   ```
6. Commit your changes:
   ```bash
   git commit -m "Add some feature"
   ```
7. Push to your branch:
   ```bash
   git push origin feature/your-feature-name
   ```
8. Create a Pull Request

### Coding Guidelines

- Follow Rust's standard coding style
- Use meaningful variable and function names
- Write documentation for public APIs
- Include tests for new functionality

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgements

- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation library
- [colored](https://github.com/mackwic/colored) - Coloring terminal output
- [once_cell](https://github.com/matklad/once_cell) - Single assignment cells for Rust
