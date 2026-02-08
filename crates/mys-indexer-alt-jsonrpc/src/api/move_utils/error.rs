// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[derive(thiserror::Error, Debug)]
pub(super) enum Error {
    #[error("Invalid Move identifier: {0:?}")]
    BadIdentifier(String),

    #[error("{0}")]
    NotFound(mys_package_resolver::error::Error),

    #[error("Type resolution limit reached: {0}")]
    ResolutionLimit(mys_package_resolver::error::Error),
}
