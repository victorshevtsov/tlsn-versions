#! /bin/bash

set -e

cargo build --release --examples

export RUST_LOG=none

URI=https://mock.verity.usher.so/1kb.json ./target/release/examples/prove
URI=https://mock.verity.usher.so/2kb.json ./target/release/examples/prove
URI=https://mock.verity.usher.so/4kb.json ./target/release/examples/prove
URI=https://mock.verity.usher.so/8kb.json ./target/release/examples/prove
URI=https://mock.verity.usher.so/16kb.json ./target/release/examples/prove 
URI=https://mock.verity.usher.so/32kb.json ./target/release/examples/prove
URI=https://mock.verity.usher.so/64kb.json ./target/release/examples/prove
URI=https://mock.verity.usher.so/128kb.json ./target/release/examples/prove
URI=https://mock.verity.usher.so/256kb.json ./target/release/examples/prove
URI=https://mock.verity.usher.so/512kb.json ./target/release/examples/prove
URI=https://mock.verity.usher.so/1024kb.json ./target/release/examples/prove
