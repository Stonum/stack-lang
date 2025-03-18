#!/bin/bash

cd server

mkdir -p ./bin/win32
cargo build --release --bins
cp ./target/release/*.exe ./bin/win32/

mkdir -p ./bin/linux
cross build --target x86_64-unknown-linux-gnu --release --bins
cp ./target/x86_64-unknown-linux-gnu/release/stack-lang-server ./bin/linux/

cd ..