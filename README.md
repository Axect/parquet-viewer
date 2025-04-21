# Parquet Viewer

A simple command-line tool to read and display the contents of an Apache Parquet file.

## Description

This utility takes the path to a Parquet file as a command-line argument, reads the file using the `peroxide` crate, and prints the resulting DataFrame to the standard output.

## Installation

Ensure you have the Rust toolchain installed (see [rustup.rs](https://rustup.rs/)).

You can install `parquet-viewer` directly from crates.io using Cargo:

```bash
cargo install parquet-viewer
```

Alternatively, you can clone the repository and build it locally:

```bash
# git clone <your-repository-url> # Replace with your repo URL if you have one
# cd parquet-viewer
cargo build --release
# The executable will be in ./target/release/parquet-viewer
```

## Usage

Run the tool by providing the path to your Parquet file:

```bash
parquet-viewer <path/to/your/file.parquet>
```

Example:

```bash
parquet-viewer data/my_data.parquet
```

The contents of the Parquet file will be printed to your console in a tabular format.

## Dependencies

This tool relies on the following major Rust crate:

* [peroxide](https://crates.io/crates/peroxide): For reading Parquet files and handling DataFrames.

## License

This project is licensed under the [MIT license](./LICENSE).
