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
