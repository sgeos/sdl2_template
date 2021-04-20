# SDL2 Template

This is a template project for creating SDL2 projects with Rust.
The project is split into a binary and a library.
The library has a native Rust entry point, and an FFI adapter that accepts
C-style argc and argv.
This design allows the program to be ported to various systems by linking
the library to the native entry point, even if it is a C or C++ program.

# Usage Examples

This template accepts command line arguments.
Press ESC to exit the program.

```sh
cargo run
cargo run -- --help
cargo run -- --version
cargo run -- --fullscreen
cargo run -- -t "Window Title" -w 200 -h200
cargo run -- -f21 -i1 -d7
```

