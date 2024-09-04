#!/bin/sh

APP_NAME=$(grep -m1 -oP '(?<=name = ")[^"]*' Cargo.toml)
APP_VERSION=$(grep -m1 -oP '(?<=version = ")[^"]*' Cargo.toml)

check() {
  TARGET=$1
  if [ ! -f "./target/${TARGET}/release/${APP_NAME}" ]; then
    echo "Error: ${APP_NAME} binary not found."
    exit 1
  fi
}

cleanup() {
  echo "Cleaning up..."
  rm -rf ./release > /dev/null 2>&1
}

prepare() {
  TARGET=$1
  printf "\nPacking target %s...\n" "${TARGET}"
  check "${TARGET}"
  mkdir -p "release/${TARGET}"
  cp -a "./target/${TARGET}/release/${APP_NAME}" "./release/${TARGET}/${APP_NAME}"
  cp -a README.md "./release/${TARGET}/README.md"
  cp -a LICENSE "./release/${TARGET}/LICENSE"
  echo  "Making tarball archive..."
  TARGET_FOR_NAME=$(echo "${TARGET}" | sed 's/-unknown//')
  # going subshell to avoid changing dir in current ctx
  (
    cd ./release/"${TARGET}" || exit 2
    FILENAME="${APP_NAME}-${APP_VERSION}-${TARGET_FOR_NAME}"
    tar -czvf "../${FILENAME}.tar.gz" * > /dev/null
    cd ..
    echo "Generating sha256 checksum for tarball archive..."
    sha256sum "${FILENAME}.tar.gz" > "${FILENAME}.tar.gz.sha256"
    echo "Pack of target ${TARGET} completed."
  )
}

### main ###

cleanup
prepare 'x86_64-unknown-linux-gnu'
prepare 'aarch64-unknown-linux-gnu'
