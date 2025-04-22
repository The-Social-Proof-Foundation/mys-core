# Copyright (c) Mysten Labs, Inc.
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0

# test that we get an error if we supply both `--skip-dependency-verification` and `--verify-deps`

echo "=== publish (should fail) ===" | tee /dev/stderr
mys client --client.config $CONFIG publish example --skip-dependency-verification --verify-deps

echo "=== upgrade (should fail) ===" | tee /dev/stderr
mys client --client.config $CONFIG upgrade example --upgrade-capability 0x1234 --skip-dependency-verification --verify-deps
