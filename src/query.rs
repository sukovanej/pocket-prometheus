use std::collections::HashMap;

use crate::parser::{Measurement, Metric};

#[derive(Clone, Debug)]
pub struct MetricQuery {
    pub name: String,
    pub labels: HashMap<String, String>,
}

impl MetricQuery {
    pub fn empty() -> MetricQuery {
        MetricQuery {
            name: "".into(),
            labels: HashMap::new(),
        }
    }
}

pub fn query_measurements(query: &MetricQuery, measurements: &[Measurement]) -> Vec<Measurement> {
    measurements
        .iter()
        .map(|measurement| query_measurement(query, measurement))
        .collect()
}

pub fn query_measurement(query: &MetricQuery, measurement: &Measurement) -> Measurement {
    Measurement {
        metrics: measurement
            .metrics
            .iter()
            .filter(|metric| is_metric_matching_query(query, metric))
            .map(|metric| metric.to_owned())
            .collect(),
        timestamp_ns: measurement.timestamp_ns,
    }
}

fn is_metric_matching_query(query: &MetricQuery, metric: &Metric) -> bool {
    metric.name.contains(&query.name) && is_metric_matching_labels(&query.labels, &metric.labels)
}

fn is_metric_matching_labels(
    query_labels: &HashMap<String, String>,
    metric_labels: &HashMap<String, String>,
) -> bool {
    query_labels
        .iter()
        .all(|(query_label_name, query_label_value)| {
            metric_labels
                .get(query_label_name)
                .map(|metric_label_value| metric_label_value == query_label_value)
                .unwrap_or(false)
        })
}
