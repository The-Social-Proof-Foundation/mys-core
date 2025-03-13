// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use mysten_metrics::histogram::Histogram as HistogramExt;
use prometheus::{
    register_histogram_with_registry, register_int_counter_with_registry, Histogram, IntCounter,
};

#[derive(Clone)]
pub struct SocialApiMetrics {
    pub get_value_by_id_calls: IntCounter,
    pub get_value_by_id_latency: Histogram,
    pub get_values_by_owner_calls: IntCounter,
    pub get_values_by_owner_latency: Histogram,
    pub set_value_calls: IntCounter,
    pub set_value_latency: Histogram,
}

impl SocialApiMetrics {
    pub fn new(registry: &prometheus::Registry) -> Self {
        Self {
            get_value_by_id_calls: register_int_counter_with_registry!(
                "social_get_value_by_id_calls",
                "Number of get_value_by_id calls",
                registry
            )
            .unwrap(),
            get_value_by_id_latency: register_histogram_with_registry!(
                "social_get_value_by_id_latency",
                "Latency of get_value_by_id operations in seconds",
                mysten_metrics::LATENCY_SEC_BUCKETS.to_vec(),
                registry
            )
            .unwrap(),
            get_values_by_owner_calls: register_int_counter_with_registry!(
                "social_get_values_by_owner_calls",
                "Number of get_values_by_owner calls",
                registry
            )
            .unwrap(),
            get_values_by_owner_latency: register_histogram_with_registry!(
                "social_get_values_by_owner_latency",
                "Latency of get_values_by_owner operations in seconds",
                mysten_metrics::LATENCY_SEC_BUCKETS.to_vec(),
                registry
            )
            .unwrap(),
            set_value_calls: register_int_counter_with_registry!(
                "social_set_value_calls",
                "Number of set_value calls",
                registry
            )
            .unwrap(),
            set_value_latency: register_histogram_with_registry!(
                "social_set_value_latency",
                "Latency of set_value operations in seconds",
                mysten_metrics::LATENCY_SEC_BUCKETS.to_vec(),
                registry
            )
            .unwrap(),
        }
    }

    pub fn new_for_tests() -> Self {
        let registry = prometheus::Registry::new();
        Self::new(&registry)
    }
}