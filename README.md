# plutovg-sys

[![Crates.io](https://img.shields.io/crates/v/plutovg-sys)](https://crates.io/crates/plutovg-sys)
[![Documentation](https://docs.rs/plutovg-sys/badge.svg)](https://docs.rs/plutovg-sys)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

FFI bindings to [PlutoVG](https://github.com/sammycage/plutovg), a 2D vector graphics library written in C.

## About PlutoVG

PlutoVG is a standalone 2D vector graphics library that provides:
- Vector path rendering
- Text rendering with font support
- Various paint types (solid colors, gradients, patterns)
- Transformations and clipping
- Export to PNG format

## Features

- Raw FFI bindings to the complete PlutoVG C API
- Safe Rust constants for mathematical values (π, √2, etc.)
- Automatic building and linking of the PlutoVG C library
- Cross-platform support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
plutovg-sys = "0.0.1"
```

## Usage

This crate provides low-level FFI bindings. All functions are `unsafe` and require manual memory management.

### Basic Example

```rust
use plutovg_sys::*;
use std::ffi::CString;

unsafe {
    // Create a 400x400 surface
    let surface = plutovg_surface_create(400, 400);
    let canvas = plutovg_canvas_create(surface);
    
    // Draw a red circle
    plutovg_canvas_arc(canvas, 200.0, 200.0, 100.0, 0.0, PLUTOVG_TWO_PI, false);
    plutovg_canvas_set_rgb(canvas, 1.0, 0.0, 0.0); // Red
    plutovg_canvas_fill(canvas);
    
    // Save to PNG
    let filename = CString::new("circle.png").unwrap();
    plutovg_surface_write_to_png(surface, filename.as_ptr());
    
    // Clean up
    plutovg_canvas_destroy(canvas);
    plutovg_surface_destroy(surface);
}
```

### Smiley Face Example

Run the included example to generate a smiley face:

```bash
cargo run --example basic
```

This creates a `smiley.png` file with a yellow smiley face.

## Constants

The crate provides mathematical constants as `f32` values:

- `PLUTOVG_PI` - π (3.141592...)
- `PLUTOVG_TWO_PI` - 2π
- `PLUTOVG_HALF_PI` - π/2
- `PLUTOVG_SQRT2` - √2
- `PLUTOVG_KAPPA` - κ (0.552284...)

## Building

This crate uses `bindgen` to generate Rust bindings and `cc` to compile the PlutoVG C library. The build process:

1. Compiles all PlutoVG C source files
2. Generates Rust bindings from the C headers
3. Links everything together

### Requirements

- A C compiler (GCC, Clang, or MSVC)
- Rust 2024 edition

## Safety

⚠️ **All functions in this crate are `unsafe`**

You must ensure:
- Proper initialization and cleanup of surfaces and canvases
- Valid pointer management
- Correct order of operations (create surface before canvas, etc.)
- No use-after-free errors

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

The bundled PlutoVG library is also MIT licensed.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Links

- [PlutoVG GitHub Repository](https://github.com/sammycage/plutovg)
- [Crates.io](https://crates.io/crates/plutovg-sys)
- [Documentation](https://docs.rs/plutovg-sys)