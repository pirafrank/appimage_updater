# AppImage Updater

[![CI](https://github.com/pirafrank/appimage_updater/actions/workflows/ci.yml/badge.svg)](https://github.com/pirafrank/appimage_updater/actions/workflows/ci.yml)
[![Release](https://github.com/pirafrank/appimage_updater/actions/workflows/release.yml/badge.svg)](https://github.com/pirafrank/appimage_updater/actions/workflows/release.yml)

A CLI tool to look for AppImages in your $PATH and call appimageupdatetool to update them

 ![GIF Image with demo of the tool](./assets/appimageupdater.gif)

## Install

Build, and install to `$HOME/.cargo/bin` in one command via `cargo`:

```sh
cargo install --locked --git https://github.com/pirafrank/appimage_updater
```

Or download binary from the [latest release](https://github.com/pirafrank/appimage_updater/releases/latest).

## Update

Download the new binary version and overwrite old one.

Or, if installed via `cargo`, re-run the `cargo install` command.

## Build

```sh
git clone https://github.com/pirafrank/appimage_updater.git
cd appimage_updater
cargo build
```

### Build a release

Build a release for your current platform triple.

Currenly supported triples are listed in `rust-toolchain.toml` file.

```sh
just release
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

### Option 1 (via `cross`)

```sh
cargo install cross
just release_all
```

### Option 2 (via `cargo`)

1. Add target to rustup:

```sh
rustup target add aarch64-unknown-linux-gnu
```

2. then cross-compile via `cargo`:

```sh
sudo apt-get install gcc-aarch64-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

