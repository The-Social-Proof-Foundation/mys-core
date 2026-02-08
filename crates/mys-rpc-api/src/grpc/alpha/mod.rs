// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

pub mod event_service;
pub mod list_authenticated_events;
pub mod proof_service;

pub mod event_service_proto {
    include!("../../proto/generated/mys.rpc.alpha.rs");
}

pub mod proof_service_proto {
    include!("../../proto/generated/mys.rpc.alpha.rs");
}
