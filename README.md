# Open Computing Club QR and Recruitment Generator

This Rust-based command-line tool generates QR codes and recruitment posters for the Open Computing Club. It automates the creation of consistent recruitment assets, reducing manual design work.

## Prerequisites

- Rust toolchain installed. You can install Rust via [rustup](https://www.rust-lang.org/tools/install).

## Getting Started

Clone the repository and build the project:

```sh
git clone https://github.com/cosineFox/OCC-club-recruitment-gen.git
cd OCC-club-recruitment-gen
cargo build --release
```

Run the application:

```sh
cargo run --release
```

Alternatively, use the provided `run.sh` script:

```sh
./run.sh
```

## Features

- Generates QR codes for recruitment materials.
- Creates ASCII-art poster assets using templated designs.
- Provides a simple terminal user interface (TUI) for customizing output.

## Why this project exists

Designing recruitment posters manually can be repetitive and time-consuming. This tool automates the process, allowing the club to produce consistent materials quickly.

## License

This project is licensed under the MIT License. See the `LICENSE` file for details.
