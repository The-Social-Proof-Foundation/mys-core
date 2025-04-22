# Copyright (c) Mysten Labs, Inc.
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0

# simple test just to make sure the test runner works with the network
mys client --client.config $CONFIG objects --json | jq 'length'
