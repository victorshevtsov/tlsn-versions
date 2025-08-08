#! /bin/bash

set -e

cargo build --release

# URI=/1kb.json ./target/release/alpha-9
# URI=/2kb.json ./target/release/alpha-9
# URI=/4kb.json ./target/release/alpha-9
# URI=/8kb.json ./target/release/alpha-9
# URI=/16kb.json ./target/release/alpha-9 
URI=/32kb.json ./target/release/alpha-9
URI=/63kb.json ./target/release/alpha-9
URI=/64kb.json ./target/release/alpha-9
URI=/128kb.json ./target/release/alpha-9
# URI=/256kb.json ./target/release/alpha-9
# URI=/512kb.json ./target/release/alpha-9
# URI=/1024kb.json ./target/release/alpha-9
