#!/bin/bash

set -eu
source $HOME/.cargo/env

cargo +nightly build
cargo +nightly build --manifest-path ./melons/Cargo.toml
cargo +nightly build --manifest-path ./pumpkins/Cargo.toml
