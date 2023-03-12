mod collector;
mod get_metrics;
mod graph_render;
mod parser;
mod plain_render;
mod query;

use crossterm::{cursor, execute, terminal, QueueableCommand};
use plain_render::render_plain;
use std::io::{stdout, Stdout, Write};
use std::{
    collections::HashMap,
    sync::mpsc,
    time::{Duration, SystemTime},
};

use collector::collect_metrics;
use parser::{parse_metrics, Measurement};
use tokio::time::sleep;

use crate::query::{query_measurements, MetricQuery};

fn current_timestamp_ns() -> u128 {
    let duration_since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    duration_since_epoch.as_nanos()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = "http://localhost";
    let port = 3000;
    let scrape_period = 2000;
    let now_timestamp_ns = current_timestamp_ns();

    let mut stdout = stdout();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    let query = MetricQuery {
        name: "nodejs_eventloop_lag_p99_seconds".to_string(),
        labels: HashMap::new(),
    };

    let (measurement_channel_sender, measurement_channel_receiver) = mpsc::channel::<Measurement>();

    tokio::spawn(async move {
        loop {
            let metrics = collect_metrics(host, port).await.unwrap();
            let measurement = parse_metrics(&metrics).unwrap();
            let _ = measurement_channel_sender.send(measurement);
            sleep(Duration::from_millis(scrape_period)).await;
        }
    });

    let mut measurements: Vec<Measurement> = vec![];

    while let Ok(measurement) = measurement_channel_receiver.recv() {
        measurements.push(measurement);

        let filtered_measurements = query_measurements(&query, &measurements);
        let data = render_plain(&filtered_measurements, now_timestamp_ns);
        redraw_stdout(data, &mut stdout);
    }

    Ok(())
}

fn redraw_stdout(data: String, mut stdout: &Stdout) {
    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::All)
    ).unwrap();

    stdout.write_all(data.as_bytes()).unwrap();
    stdout.flush().unwrap();
}
