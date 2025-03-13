// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

#[derive(Error, Debug)]
pub enum SocialError {
    #[error("Social API error: {0}")]
    Generic(String),
}