# AppImage Updater

[![CI](https://github.com/pirafrank/appimage_updater/actions/workflows/ci.yml/badge.svg)](https://github.com/pirafrank/appimage_updater/actions/workflows/ci.yml)

A CLI tool to look for AppImages in your $PATH and call appimageupdatetool to update them

 ![GIF Image with demo of the tool](./assets/appimageupdater.gif)

## Install

Download, build, and install to `$HOME/.cargo/bin` in one command:

```sh
cargo install --locked --git https://github.com/pirafrank/appimage_updater
```

Alternatively, you can build the binary and move it to `$PATH` by yourself.

## Update

Re-run the `cargo install` command.

## Build

```sh
git clone https://github.com/pirafrank/appimage_updater.git
cd appimage_updater
make release
# add to path, e.g.:
# cp ./target/release/appimage_updater ~/.local/bin/
```

## Run

```sh
# 4 threads by default
appimage_updater
```

```sh
# 2 threads
appimage_updater -j 2
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
