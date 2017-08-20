# C3: A tree of C nodes

This crate parses C files and exposes them as an abstract syntax tree.

The AST is a relaxed version of C's usual structure (e.g. everything pretends to be an expression), but contains enough information to rebuild complete source code. The AST also allows for some Rust features not found in C files. This is enables [Citrus](https://gitlab.com/citrus-rs/citrus) to convert C to Rust.

It uses LLVM 4 and Clang's unstable C++ API (and therefore it's unlikely to work with any other version than LLVM 4.0).

The stable Clang API does not expose a real AST, but a flattened, incomplete and ambiguous view of it. This crate works around the bad parts to extract more complete view of C files from Clang.

On the Rust side it's based on [bindgen](https://github.com/rust-lang-nursery/rust-bindgen).

## Building

[Follow these instructions](https://gitlab.com/citrus-rs/c3/blob/master/c3_clang_extensions/README.md).
