// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[derive(thiserror::Error, Debug)]
pub(super) enum Error {
    #[error("Domain not found: {0}")]
    NotFound(String),

    #[error(transparent)]
    NameService(mys_name_service::NameServiceError),
}
