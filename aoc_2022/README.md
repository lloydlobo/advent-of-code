
# Advent of Code 2022

[Source](https://github.com/zookini/aoc-2022/tree/master/src/bin)


## Structure

### [Package Layout](https://doc.rust-lang.org/cargo/guide/project-layout.html#package-layout)

> If a binary, example, bench, or integration test consists of multiple source
> files, place a main.rs file along with the extra modules within a subdirectory
> of the src/bin, examples, benches, or tests directory. The name of the
> executable will be the directory name.

- Cargo.toml and Cargo.lock are stored in the root of your package (package root).
- Source code goes in the src directory.
- The default library file is src/lib.rs.
- The default executable file is src/main.rs.
- Other executables can be placed in src/bin/.
- Benchmarks go in the benches directory.
- Examples go in the examples directory.
- Integration tests go in the tests directory.

You can learn more about Rust's module system in the [book](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html).

## Dev

### Usage

You can run individual binaries with the `cargo run` command
with the `--bin <bin-name>` option.

`cargo install` can be used to copy the executable to a common location.





