```shell
██╗██████╗  ██████╗ ███╗   ██╗     ██████╗██╗   ██╗██████╗ ██╗     
██║██╔══██╗██╔═══██╗████╗  ██║    ██╔════╝██║   ██║██╔══██╗██║     
██║██████╔╝██║   ██║██╔██╗ ██║    ██║     ██║   ██║██████╔╝██║     
██║██╔══██╗██║   ██║██║╚██╗██║    ██║     ██║   ██║██╔══██╗██║     
██║██║  ██║╚██████╔╝██║ ╚████║    ╚██████╗╚██████╔╝██║  ██║███████╗
╚═╝╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═══╝     ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚══════╝
                                                                   
```

`iron_curl` is an asynchronous Rust cli tool designed to simplify HTTP request handling by providing a versatile interface for making requests, processing responses, and handling HTTP methods.

## Features

- Support for multiple HTTP methods, including GET, POST, PATCH, PUT, and DELETE.
- Customizable request headers and body content.
- Asynchronous execution for improved performance with `tokio`.
- User-friendly command-line interface leveraging `clap`.

## Getting Started

### Prerequisites

You should have the Rust toolchain installed, including `cargo`, Rust's package manager and build tool. This project requires Rust 2018 edition or newer.

### Installation

To use `iron_curl`, clone the repository and build the project:

```shell
git clone https://github.com/renatodinizc/iron_curl.git
cd iron_curl
cargo build --release
```

The executable will be located in `./target/release/`.

### Usage

`iron_curl` replicates GNU curl's functionality in a Rust environment. Use it from the command line to make HTTP requests:

```shell
./target/release/iron_curl https://example.com
```

Supports various command-line options to customize your requests:

- `--request` or `-X`: Specifies the HTTP method to use.
- `--header` or `-H`: Adds a header to the request.
- `--data` or `-d`: Sends data in a POST request.

For a full list of options, run:

```shell
./target/release/iron_curl --help
```

## Examples

Perform a GET request:

```shell
./target/release/iron_curl https://httpbin.org/get
```

Send a POST request with headers and body data:

```shell
./target/release/iron_curl -X POST https://httpbin.org/post -H "Content-Type: application/json" -d '{"key":"value"}'
```

## Contributing

We welcome contributions! If you've found a bug or have a feature request, please open an issue. If you're ready to contribute code, please submit a pull request with your changes.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.
