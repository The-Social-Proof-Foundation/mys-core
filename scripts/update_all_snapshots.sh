#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0
#
# Automatically update all snapshots. This is needed when the framework is changed or when protocol config is changed.

set -x
set -e

SCRIPT_PATH=$(realpath "$0")
SCRIPT_DIR=$(dirname "$SCRIPT_PATH")
ROOT="$SCRIPT_DIR/.."

cd "$ROOT/crates/mys-protocol-config" && cargo insta test --review
cd "$ROOT/crates/mys-swarm-config" && cargo insta test --review
cd "$ROOT/crates/mys-open-rpc" && cargo run --example generate-json-rpc-spec -- record
cd "$ROOT/crates/mys-core" && cargo run --example generate-format -- print > tests/staged/mys.yaml
UPDATE=1 cargo test -p mys-framework --test build-system-packages
