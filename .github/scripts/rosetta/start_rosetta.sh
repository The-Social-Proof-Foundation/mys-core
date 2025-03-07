#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

echo "Start Rosetta online server"
mys-rosetta start-online-server --data-path ./data &

echo "Start Rosetta offline server"
mys-rosetta start-offline-server &
