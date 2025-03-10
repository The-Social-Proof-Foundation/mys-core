#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

echo "Install binaries"
cargo install --locked --bin mys --path crates/mys
cargo install --locked --bin mys-rosetta --path crates/mys-rosetta

echo "run MySocial genesis"
mys genesis

echo "generate rosetta configuration"
mys-rosetta generate-rosetta-cli-config --online-url http://127.0.0.1:9002 --offline-url http://127.0.0.1:9003

echo "install rosetta-cli"
curl -sSfL https://raw.githubusercontent.com/coinbase/rosetta-cli/master/scripts/install.sh | sh -s