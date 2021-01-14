# Unrusty

Unrusty is a personal project to implement some git commands in Rust. I started this project to learn Rust and get to know the inner workings of git.

The project is currently ongoing. In particular, I have implemented the underlying database, the index, and some low-level commands (so called "plumbing" commands in the git lingo).

Please note that the purpose of this project is my personal entertainement, and the project is not intended to be used. Thus, best practices are not necessarely followed.

## Installation

Follow the [instructions](https://doc.rust-lang.org/book/ch01-01-installation.html#installation) to install Rust and its package manager Cargo. You can now run `cargo run help` to see all available commands. Cargo will take care of installing all dependencies and compiling the project.

## Usage

Run `cargo run help` for available commands and then `cargo run <command> --help` for details for a particular command.

## License
[MIT](https://choosealicense.com/licenses/mit/)

