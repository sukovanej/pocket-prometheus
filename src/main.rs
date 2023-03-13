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
    GetMetrics(GetMetricsArgs),
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct RunArgs {
    /// Port
    #[arg(short, long)]
    port: u32,

    /// Host
    #[arg(short, long, default_value = "http://localhost")]
    host: String,

    /// Scrape period
    #[arg(short, long, default_value_t = 2000)]
    scrape_period: u64,
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct GetMetricsArgs {
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

    let (measurements_tx, measurements_rx) = tokio::sync::mpsc::channel::<Vec<Measurement>>(32);
    let (user_input_tx, user_input_rx) = tokio::sync::mpsc::channel::<UserInput>(32);

    match args.action {
        Action::Run(args) => {
            manage_user_input(user_input_tx);
            manage_measurements(args.host, args.port, args.scrape_period, measurements_tx);
            controller(measurements_rx, user_input_rx).await;
        }
        Action::GetMetrics(args) => {
            let metrics = get_metrics(&args.host, args.port).await;
            println!("{:#?}", metrics);
        }
    }

    Ok(())
}

fn current_timestamp_ns() -> u128 {
    let duration_since_epoch = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    duration_since_epoch.as_nanos()
}

async fn controller(
    mut measurements_rx: Receiver<Vec<Measurement>>,
    mut user_input_rx: Receiver<UserInput>,
) {
    enable_raw_mode().unwrap();

    let now_timestamp_ns = current_timestamp_ns();
    let mut stdout = stdout();
    execute!(stdout, crossterm::terminal::EnterAlternateScreen).unwrap();

    let mut measurements = vec![];
    let mut query = MetricQuery::empty();
    let mut scroll_offset: i64 = 0;

    loop {
        let mut incomming_query = None;
        let mut incomming_measurements = None;

        tokio::select! {
            query = user_input_rx.recv() => incomming_query = query,
            measurements = measurements_rx.recv() => incomming_measurements = measurements,
        };

        if let Some(incomming_query) = incomming_query {
            match incomming_query {
                UserInput::MetricQuery(incomming_query) => query = incomming_query,
                UserInput::Exit => {
                    execute!(stdout, crossterm::terminal::LeaveAlternateScreen).unwrap();
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

        if let Some(incomming_measurements) = incomming_measurements {
            measurements = incomming_measurements;
        }

        let filtered_measurements = query_measurements(&query, &measurements);
        let data = render_plain(&filtered_measurements, now_timestamp_ns);
        redraw_stdout(&query, data, &stdout, scroll_offset as u32);
    }
}

fn manage_measurements(
    host: String,
    port: u32,
    scrape_period: u64,
    measurements_tx: Sender<Vec<Measurement>>,
) {
    tokio::spawn(async move {
        let mut all_measurements: Vec<Measurement> = vec![];

        loop {
            let metrics = collect_metrics(&host, port).await.unwrap();
            let measurement = parse_metrics(&metrics).unwrap();
            all_measurements.push(measurement);
            measurements_tx
                .send(all_measurements.clone())
                .await
                .unwrap();
            sleep(Duration::from_millis(scrape_period)).await;
        }
    });
}
