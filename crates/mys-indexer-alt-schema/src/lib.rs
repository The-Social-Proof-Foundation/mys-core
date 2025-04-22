// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel_migrations::{embed_migrations, EmbeddedMigrations};

pub mod checkpoints;
pub mod displays;
pub mod epochs;
pub mod events;
pub mod objects;
pub mod packages;
pub mod schema;
pub mod transactions;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
