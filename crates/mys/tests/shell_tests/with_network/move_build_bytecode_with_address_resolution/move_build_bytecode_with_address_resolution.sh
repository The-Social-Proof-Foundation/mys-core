# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

mys client --client.config $CONFIG \
  publish simple --verify-deps \
  --json | jq '.effects.status'

mys move --client.config $CONFIG \
  build --path depends_on_simple
