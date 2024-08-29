#!/usr/bin/env -S just

# Default number of threads
default_threads := "4"

# Task to uild the project
build:
    cargo build

# Task to build the project for all targets
build_all:
    cross build --target x86_64-unknown-linux-gnu
    cross build --target aarch64-unknown-linux-gnu

# Task to build the project in release mode
release:
    cargo build --release

# Task to build the project in release mode for all targets
release_all:
    cross build --release --target x86_64-unknown-linux-gnu
    cross build --release --target aarch64-unknown-linux-gnu

# Task to run the project with the specified number of threads
run j=default_threads:
    cargo run -- -j {{j}}

# Default task to run the project with the default number of threads
default:
    just run
