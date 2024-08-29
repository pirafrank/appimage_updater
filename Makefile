# Makefile

# Default number of threads
J := 4

# Target to build the project
build:
	cargo build

# Task to build the project for all targets
build_all:
    cross build --target x86_64-unknown-linux-gnu
    cross build --target aarch64-unknown-linux-gnu

# Target to build the project in release mode
release:
	cargo build --release

# Task to build the project in release mode for all targets
release_all:
    cross build --release --target x86_64-unknown-linux-gnu
    cross build --release --target aarch64-unknown-linux-gnu

# Target to run the project with the specified number of threads
run:
	cargo run -- -j $(J)

# Phony targets to avoid conflicts with files named 'build' or 'run'
.PHONY: build run
