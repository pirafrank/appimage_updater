# justfile

# Default number of threads
default_threads := "4"

# Task to uild the project
build:
    cargo build

# Task to build the project in release mode
release:
    cargo build --release

# Task to run the project with the specified number of threads
run j=default_threads:
    cargo run -- -j {{j}}

# Default task to run the project with the default number of threads
default:
    just run
