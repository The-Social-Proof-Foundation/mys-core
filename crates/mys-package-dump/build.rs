// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

fn main() {
    cynic_codegen::register_schema("mys")
        .from_sdl_file("../mys-indexer-alt-graphql/schema.graphql")
        .unwrap()
        .as_default()
        .unwrap();
}
