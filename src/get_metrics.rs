use std::collections::HashSet;

use crate::{collector::collect_metrics, parser::parse_metrics};

pub async fn get_metrics(host: &str, port: u32) -> Vec<String> {
    let metrics = collect_metrics(host, port).await.unwrap();
    let measurement = parse_metrics(&metrics).unwrap();
    let set = measurement
        .metrics
        .iter()
        .map(|metric| metric.name.to_owned())
        .collect::<HashSet<String>>();

    Vec::from_iter(set)
}
