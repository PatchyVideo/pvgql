#!/bin/bash
export PKG_CONFIG_ALLOW_CROSS=1
export OPENSSL_STATIC=true
export OPENSSL_DIR=/musl
cargo build --target x86_64-unknown-linux-musl --release
docker build --no-cache -t pvgql .
docker save -o pvgql.tar pvgql
