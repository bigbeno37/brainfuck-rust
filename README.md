# Brainfuck Rust
An implementation of the Brainfuck programming language in Rust.
To use without a compiled file, run `cargo run -- <brainfuck here>` or `cargo run -- -i <path to file>`.
This listens to stdin for the `,` operator, so pipe away! e.g. `echo "ABC" | cargo run -- ,.,.,.`