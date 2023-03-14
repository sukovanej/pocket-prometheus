use std::collections::HashSet;
use std::error::Error;

use crate::{collector::collect_metrics, parser::parse_metrics};

pub async fn get_metrics(host: &str, port: u32) -> Result<Vec<String>, Box<dyn Error>> {
    let metrics = collect_metrics(host, port).await?;
    let measurement = parse_metrics(&metrics)?;
    let set = measurement
        .metrics
        .iter()
        .map(|metric| metric.name.to_owned())
        .collect::<HashSet<String>>();

    Ok(Vec::from_iter(set))
}
