# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# basic test that mys move new outputs correct files
mys move new example
echo ==== files in project ====
ls -A example
echo ==== files in sources ====
ls -A example/sources
echo ==== files in tests =====
ls -A example/tests
