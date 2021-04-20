# SDL2 Template

This is a template project for creating SDL2 projects with Rust.
The project is split into a binary and a library.
The library has a native Rust entry point, and an FFI adapter that accepts
C-style argc and argv.
This design allows the program to be ported to various systems by linking
the library to the native entry point, even if it is a C or C++ program.

# Usage Examples

This template accepts command line arguments and environment variables.
Press ESC to exit the program.

```sh
# Basic Usage
cargo run

# Help and Version
cargo run -- --help
cargo run -- --version

# Command Line Arguments
cargo run -- --fullscreen
cargo run -- -t "Window Title" -w 200 -h200
cargo run -- -f21 -i1 -d7

# Environment Variables
FULLSCREEN=true cargo run
WINDOW_TITLE="Window Title" WINDOW_WIDTH=200 WINDOW_HEIGHT=200 cargo run
FLASH_INTERVAL=1 FLASH_DURATION=7 FPS=21 cargo run
```

