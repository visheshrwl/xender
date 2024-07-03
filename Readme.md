# Xender Rust

Xender Rust is a file transfer application built using Rust, providing super-fast file transfers over TCP. This application supports both high-end and low-end devices and offers a simple GUI for ease of use.

## Features

- Transfer any file format
- Super-fast transfer speeds
- Support for high-end and low-end devices
- Simple and intuitive GUI

## Getting Started

### Prerequisites

- Rust: Install from [rust-lang.org](https://www.rust-lang.org/)

### Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/visheshrwl/xender.git xender_rust
   cd xender_rust
    ```

2. Build the project:

    ```sh
    cargo build --release
    ```

## Usage

1. Start the Server:

    ```sh
    cargo run --bin server
    ```

2. Start the client:

    ```sh
    cargo run--bin client
    ```

3. Use the Gui to send and receive files.

