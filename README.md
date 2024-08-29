# AppImage Updater

 A CLI tool to look for AppImages in your $PATH and call appimageupdatetool to update them

## Installation

1. Build the binary (read below)

2. Move it to a dir in `$PATH`

## Build it

```sh
git clone https://github.com/pirafrank/appimage_updater.git
cd appimage_updater
make release
# add to path, e.g.:
# cp ./target/release/appimage_updater ~/.local/bin/
```

## Cross-compilation

1. Add target to rustup:

```sh
rustup target add aarch64-unknown-linux-gnu
```

2. then cross-compile via `cargo`:

```sh
sudo apt-get install gcc-aarch64-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

or via `cross`:

```sh
cargo install cross
just release_all
```
