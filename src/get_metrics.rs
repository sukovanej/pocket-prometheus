use std::collections::HashSet;

use crate::parser::Measurement;

pub fn get_metrics(measurement: &Measurement) -> Vec<String> {
    let set = measurement
        .metrics
        .iter()
        .map(|metric| metric.name.to_owned())
        .collect::<HashSet<String>>();

    Vec::from_iter(set)
}
