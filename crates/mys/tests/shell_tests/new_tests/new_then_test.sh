# Copyright (c) Mysten Labs, Inc.
# SPDX-License-Identifier: Apache-2.0

# check that mys move new followed by mys move test succeeds
mys move new example

# we mangle the generated toml file to replace the framework dependency with a local dependency
FRAMEWORK_DIR=$(echo $CARGO_MANIFEST_DIR | sed 's#/crates/mys##g')
cat example/Move.toml \
  | sed 's#\(MySocial = .*\)git = "[^"]*", \(.*\)#\1\2#' \
  | sed 's#\(MySocial = .*\), rev = "[^"]*"\(.*\)#\1\2#' \
  | sed 's#\(MySocial = .*\)subdir = "\([^"]*\)"\(.*\)#\1local = "FRAMEWORK/\2"\3#' \
  | sed "s#\(MySocial = .*\)FRAMEWORK\(.*\)#\1$FRAMEWORK_DIR\2#" \
  > Move.toml
mv Move.toml example/Move.toml

cd example && mys move test
