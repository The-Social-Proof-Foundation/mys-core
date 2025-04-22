# Copyright (c) Mysten Labs, Inc.
# Copyright (c) The Social Proof Foundation, LLC.
# SPDX-License-Identifier: Apache-2.0

# mys move new example when `example/.gitignore` already contains build/*; it should be unchanged
mkdir example
echo "ignore1" >> example/.gitignore
echo "build/*" >> example/.gitignore
echo "ignore2" >> example/.gitignore
mys move new example
cat example/.gitignore
echo
echo ==== files in example/ ====
ls -A example
