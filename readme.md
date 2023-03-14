# Pocket prometheus

Simple TUI application for local debugging of prometheus application metrics.

## Install

```bash
cargo install pocket-prometheus
```

## Usage

```bash
$ pocket-prometheus help
Simple TUI for prometheus metrics scraping

Usage: pocket-prometheus <COMMAND>

Commands:
  run          Simple TUI for prometheus metrics scraping
  get-metrics  Simple TUI for prometheus metrics scraping
  help         Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version```
```

Run `pocket-prometheus run --port [PORT]`. This will start a TUI with metrics collector on the background. 
In the TUI, you can query metrics by their name.

Press `escape` to exit the application.

## Example nodejs application

Start the example application.

```bash
$ git clone https://github.com/sukovanej/pocket-prometheus/
$ cd pocket-prometheus/nodejs-example-app/
$ pnpm install
$ pnpm start
```

In another terminal window, trigger `pocket-prometheus run --port 3000` and try to type *lag_secong*.
This will query for `nodejs_eventloop_lag_seconds` metric which is exposed from the nodejs 
application.

```
┌────────────────────────────────────────────────────────────────┐
│ Query: lag_se                                                  │
└────────────────────────────────────────────────────────────────┘
  Help: <UP> / <DOWN> to move around, <ESC> to quit; Offset: 0

after 0s:
 - nodejs_eventloop_lag_seconds: 0
after 2s:
 - nodejs_eventloop_lag_seconds: 0.008147666
after 4s:
 - nodejs_eventloop_lag_seconds: 0.005280334
after 6s:
 - nodejs_eventloop_lag_seconds: 0.003070375
after 8s:
 - nodejs_eventloop_lag_seconds: 0.005113959
```

Run `pocket-prometheus get-metrics --port 3000` to get a list of all metrics.
