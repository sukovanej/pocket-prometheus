mod collector;
mod get_metrics;
mod parser;
mod plain_render;
mod query;
mod stdout;
mod user_input;

use std::io::stdout;
use std::process::exit;
use std::time::{Duration, SystemTime};

use anyhow::Error;
use clap::Parser;
use crossterm::execute;
use crossterm::terminal::enable_raw_mode;
use get_metrics::get_metrics;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::sleep;

use crate::collector::collect_metrics;
use crate::parser::{parse_metrics, Measurement};
use crate::plain_render::render_plain;
use crate::query::{query_measurements, MetricQuery};
use crate::stdout::redraw_stdout;
use crate::user_input::{manage_user_input, UserInput};

#[derive(Debug, clap::Subcommand)]
enum Action {
    Run(RunArgs),
    GetMetrics(HostPortArgs),
}

#[derive(Parser, Debug)]
#[command(about = "Run interactive TUI")]
struct RunArgs {
    /// Port
    #[command(flatten)]
    host_port: HostPortArgs,

    /// Scrape period
    #[arg(short, long, default_value_t = 2000)]
    scrape_period: u64,
}

#[derive(Parser, Debug)]
#[command(about = "List all available metric names")]
struct HostPortArgs {
    /// Port
    #[arg(short, long)]
    port: u32,

    /// Host
    #[arg(short, long, default_value = "http://localhost")]
    host: String,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let (measurements_tx, measurements_rx) = tokio::sync::mpsc::channel::<MeasurementUpdate>(32);
    let (user_input_tx, user_input_rx) = tokio::sync::mpsc::channel::<UserInput>(32);

    match args.action {
        Action::Run(args) => {
            manage_user_input(user_input_tx);
            manage_measurements(
                args.host_port.host.to_owned(),
                args.host_port.port,
                args.scrape_period,
                measurements_tx,
            );
            controller(
                args.host_port.host,
                args.host_port.port,
                measurements_rx,
                user_input_rx,
            )
            .await?;
        }
        Action::GetMetrics(args) => {
            let metrics = get_metrics(&args.host, args.port).await;
            println!("{:#?}", metrics);
        }
    }

    Ok(())
}

async fn controller(
    host: String,
    port: u32,
    mut measurements_rx: Receiver<MeasurementUpdate>,
    mut user_input_rx: Receiver<UserInput>,
) -> Result<(), Error> {
    enable_raw_mode()?;

    let now_timestamp_ns = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)?
        .as_nanos();
    let mut stdout = stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;

    let mut measurements = vec![];
    let mut query = MetricQuery::default();
    let mut scroll_offset: i64 = 0;
    let mut collection_success = true;

    loop {
        let mut incomming_query = None;
        let mut incomming_measurement_update = None;

        tokio::select! {
            query = user_input_rx.recv() => incomming_query = query,
            measurement_update = measurements_rx.recv() => incomming_measurement_update = measurement_update,
        };

        if let Some(incomming_query) = incomming_query {
            match incomming_query {
                UserInput::MetricQuery(incomming_query) => query = incomming_query,
                UserInput::Exit => {
                    execute!(stdout, crossterm::terminal::LeaveAlternateScreen)?;
                    exit(0);
                }
                UserInput::ScrollDown => {
                    scroll_offset += 1;
                }
                UserInput::ScrollUp => {
                    scroll_offset -= 1.min(scroll_offset);
                }
                UserInput::ScrollPageDown => {
                    scroll_offset += 20;
                }
                UserInput::ScrollPageUp => {
                    scroll_offset -= 20.min(scroll_offset);
                }
            }
        }

        if let Some(incomming_measurement_update) = incomming_measurement_update {
            match incomming_measurement_update {
                MeasurementUpdate::Success(incomming_measurements) => {
                    measurements = incomming_measurements;
                    collection_success = true;
                }
                MeasurementUpdate::CollectError => collection_success = false,
            }
        }

        let filtered_measurements = query_measurements(&query, &measurements);

        let data = if collection_success {
            render_plain(&filtered_measurements, now_timestamp_ns)
        } else {
            format!(
                "Collection failed, {}:{}/metrics is not unavailable",
                host, port
            )
        };

        redraw_stdout(&query, data, &stdout, scroll_offset as u32)?;
    }
}

#[derive(Debug)]
enum MeasurementUpdate {
    CollectError,
    Success(Vec<Measurement>),
}

fn manage_measurements(
    host: String,
    port: u32,
    scrape_period: u64,
    measurements_tx: Sender<MeasurementUpdate>,
) {
    tokio::spawn(async move {
        let mut all_measurements: Vec<Measurement> = vec![];

        loop {
            match collect_metrics(&host, port).await {
                Ok(metrics) => {
                    let measurement = parse_metrics(&metrics)?;
                    all_measurements.push(measurement);
                    measurements_tx
                        .send(MeasurementUpdate::Success(all_measurements.clone()))
                        .await?;
                }
                Err(_) => {
                    measurements_tx
                        .send(MeasurementUpdate::CollectError)
                        .await?;
                }
            }
            sleep(Duration::from_millis(scrape_period)).await;

            if false {
                // handle app exit
                return Ok::<(), Error>(());
            }
        }
    });
}
