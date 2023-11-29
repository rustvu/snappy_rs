# snappy_rs

Example code for Rust FFI (Foreign Function Interface).
The crate creates a Rust wrapper for the Google [Snappy](https://github.com/google/snappy) compression library implemented in C/C++.

## Requirements

- [CMake](https://cmake.org/) needs to be installed, as required by <https://github.com/google/snappy>

## Instructions

This repository uses `snappy` as a git submodule. You need to clone this locally with:

```sh
git clone  --recurse-submodule https://github.com/rustvu/snappy_rs 
```

Then you can use the regular `cargo build/test` commands.