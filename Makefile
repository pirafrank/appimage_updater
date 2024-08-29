# Makefile

# Default number of threads
J := 4

# Target to build the project
build:
	cargo build

# Target to build the project in release mode
release:
	cargo build --release

# Target to run the project with the specified number of threads
run:
	cargo run -- -j $(J)

# Phony targets to avoid conflicts with files named 'build' or 'run'
.PHONY: build run
