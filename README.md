# SDL2 Template

This is a template project for creating SDL2 projects with Rust.
The project is split into a binary and a library.
The library has a native Rust entry point, and an FFI adapter that accepts
C-style argc and argv.
This design allows the program to be ported to various systems by linking
the library to the native entry point, even if it is a C or C++ program.

# Usage Examples

```sh
cargo run
```

