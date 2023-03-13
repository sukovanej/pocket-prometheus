use crate::parser::Measurement;

pub fn render_plain(measurements: &[Measurement], now_timestamp_ns: u128) -> String {
    measurements
        .iter()
        .map(|measurement| format_measurement(measurement, now_timestamp_ns))
        .collect::<Vec<String>>()
        .join("\n")
}

fn format_measurement(measurement: &Measurement, now_timestamp_ns: u128) -> String {
    let seconds_diff = (measurement.timestamp_ns - now_timestamp_ns) / 1_000_000_000;
    let metrics = measurement
        .metrics
        .iter()
        .map(|metric| format!(" - {}: {}", metric.name, metric.value))
        .collect::<Vec<String>>();
    format!("after {}s:\n{}", seconds_diff, metrics.join("\n"))
}
