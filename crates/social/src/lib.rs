// Copyright (c) The Social Proof Foundation
// SPDX-License-Identifier: Apache-2.0

//! Social networking features for the Mys blockchain.
//!
//! This crate contains the Rust components for the social networking features,
//! while the on-chain functionality is implemented in the Move package.

/// Returns the package directory for the Social Move package.
pub fn package_dir() -> &'static str {
    concat!(env!("CARGO_MANIFEST_DIR"), "/Move")
}