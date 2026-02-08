// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use mys_types::TypeTag;

#[derive(thiserror::Error, Debug)]
pub(super) enum Error {
    #[error("Bad dynamic field name: {0}")]
    BadName(anyhow::Error),

    #[error("Invalid type {0}: {1}")]
    BadType(TypeTag, mys_package_resolver::error::Error),

    #[error("Could not serialize dynamic field name as {0}: {1}")]
    TypeMismatch(TypeTag, anyhow::Error),
}
