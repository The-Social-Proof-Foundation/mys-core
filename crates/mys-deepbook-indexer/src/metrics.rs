// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use prometheus::{
    register_int_counter_vec_with_registry, register_int_counter_with_registry,
    register_int_gauge_vec_with_registry, register_int_gauge_with_registry, IntCounter, IntCounterVec, IntGauge, IntGaugeVec, Registry,
};
use mys_indexer_builder::metrics::IndexerMetricProvider;

#[derive(Clone, Debug)]
pub struct DeepBookIndexerMetrics {
    pub(crate) total_deepbook_transactions: IntCounter,
    pub(crate) backfill_tasks_remaining_checkpoints: IntGaugeVec,
    pub(crate) tasks_processed_checkpoints: IntCounterVec,
    pub(crate) inflight_live_tasks: IntGaugeVec,
    pub(crate) tasks_latest_retrieved_checkpoints: IntGaugeVec,
    // TimescaleDB-specific metrics
    pub(crate) timescale_chunks_total: IntGaugeVec,
    pub(crate) timescale_compressed_chunks: IntGaugeVec,
    pub(crate) timescale_insert_rate: IntCounterVec,
    pub(crate) timescale_query_performance: IntGauge,
}

impl DeepBookIndexerMetrics {
    pub fn new(registry: &Registry) -> Self {
        Self {
            total_deepbook_transactions: register_int_counter_with_registry!(
                "deepbook_indexer_total_deepbook_transactions",
                "Total number of deepbook transactions",
                registry,
            )
            .unwrap(),
            backfill_tasks_remaining_checkpoints: register_int_gauge_vec_with_registry!(
                "deepbook_indexer_backfill_tasks_remaining_checkpoints",
                "The remaining checkpoints for the currently running backfill task",
                &["task_name"],
                registry,
            )
            .unwrap(),
            tasks_processed_checkpoints: register_int_counter_vec_with_registry!(
                "deepbook_indexer_tasks_processed_checkpoints",
                "Total processed checkpoints for each task",
                &["task_name", "task_type"],
                registry,
            )
            .unwrap(),
            inflight_live_tasks: register_int_gauge_vec_with_registry!(
                "deepbook_indexer_inflight_live_tasks",
                "Number of inflight live tasks",
                &["task_name"],
                registry,
            )
            .unwrap(),
            tasks_latest_retrieved_checkpoints: register_int_gauge_vec_with_registry!(
                "deepbook_indexer_tasks_latest_retrieved_checkpoints",
                "latest retrieved checkpoint for each task",
                &["task_name", "task_type"],
                registry,
            )
            .unwrap(),
            // TimescaleDB-specific metrics
            timescale_chunks_total: register_int_gauge_vec_with_registry!(
                "deepbook_indexer_timescale_chunks_total",
                "Total number of TimescaleDB chunks per hypertable",
                &["hypertable"],
                registry,
            )
            .unwrap(),
            timescale_compressed_chunks: register_int_gauge_vec_with_registry!(
                "deepbook_indexer_timescale_compressed_chunks",
                "Number of compressed chunks per hypertable",
                &["hypertable"],
                registry,
            )
            .unwrap(),
            timescale_insert_rate: register_int_counter_vec_with_registry!(
                "deepbook_indexer_timescale_insert_rate",
                "Rate of inserts into TimescaleDB hypertables",
                &["hypertable"],
                registry,
            )
            .unwrap(),
            timescale_query_performance: register_int_gauge_with_registry!(
                "deepbook_indexer_timescale_query_performance_ms",
                "Average query performance for time-series queries in milliseconds",
                registry,
            )
            .unwrap(),
        }
    }

    pub fn new_for_testing() -> Self {
        let registry = Registry::new();
        Self::new(&registry)
    }

    // Helper methods for TimescaleDB metrics
    pub fn record_timescale_insert(&self, hypertable: &str, count: u64) {
        self.timescale_insert_rate
            .with_label_values(&[hypertable])
            .inc_by(count);
    }

    pub fn update_chunk_metrics(&self, hypertable: &str, total_chunks: i64, compressed_chunks: i64) {
        self.timescale_chunks_total
            .with_label_values(&[hypertable])
            .set(total_chunks);
        self.timescale_compressed_chunks
            .with_label_values(&[hypertable])
            .set(compressed_chunks);
    }

    pub fn record_query_performance(&self, duration_ms: u64) {
        self.timescale_query_performance.set(duration_ms as i64);
    }
}

impl IndexerMetricProvider for DeepBookIndexerMetrics {
    fn get_tasks_latest_retrieved_checkpoints(&self) -> &IntGaugeVec {
        &self.tasks_latest_retrieved_checkpoints
    }

    fn get_tasks_remaining_checkpoints_metric(&self) -> &IntGaugeVec {
        &self.backfill_tasks_remaining_checkpoints
    }

    fn get_tasks_processed_checkpoints_metric(&self) -> &IntCounterVec {
        &self.tasks_processed_checkpoints
    }

    fn get_inflight_live_tasks_metrics(&self) -> &IntGaugeVec {
        &self.inflight_live_tasks
    }
}
