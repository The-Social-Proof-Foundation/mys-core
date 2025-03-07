// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

fn main() {
    cynic_codegen::register_schema("mys")
        .from_sdl_file("../mys-graphql-rpc/schema.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
