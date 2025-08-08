#! /bin/bash

set -e

cargo build --release

# URI=/1kb.json ./target/release/alpha-11
# URI=/2kb.json ./target/release/alpha-11
# URI=/4kb.json ./target/release/alpha-11
# URI=/8kb.json ./target/release/alpha-11
# URI=/16kb.json ./target/release/alpha-11 
URI=/32kb.json ./target/release/alpha-11
URI=/63kb.json ./target/release/alpha-11
URI=/64kb.json ./target/release/alpha-11
URI=/128kb.json ./target/release/alpha-11
# URI=/256kb.json ./target/release/alpha-11
# URI=/512kb.json ./target/release/alpha-11
# URI=/1024kb.json ./target/release/alpha-11
