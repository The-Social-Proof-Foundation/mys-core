#!/bin/bash
# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0
#
# Check whether the version of framework in the repo is compatible
# with the version on chain, as reported by the currently active
# environment, using the binary in environment variable $MYS.

set -e

MYS=${MYS:-mys}
REPO=$(git rev-parse --show-toplevel)

for PACKAGE in "$REPO"/crates/mys-framework/packages/*; do
    $MYS client verify-source "$PACKAGE"
done

