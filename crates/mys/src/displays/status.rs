// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::displays::Pretty;
use std::fmt::{Display, Formatter};
use mys_json_rpc_types::MysExecutionStatus::{self, Failure, Success};

impl<'a> Display for Pretty<'a, MysExecutionStatus> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let Pretty(status) = self;

        let output = match status {
            Success => "success".to_string(),
            Failure { error } => format!("failed due to {error}"),
        };

        write!(f, "{}", output)
    }
}
