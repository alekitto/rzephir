#!/usr/bin/env bash

set -eo pipefail

rm -rf target/debug/deps/libzephir-*
cargo test
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
