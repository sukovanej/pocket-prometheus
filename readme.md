# Pocket prometheus

Simple TUI application for local debugging of prometheus application metrics.

## Install

```bash
cargo install --path .
```

## Usage

```bash
$ pocket-prometheus --help
Usage: pocket-prometheus [OPTIONS] --port <PORT>

Options:
  -p, --port <PORT>                    Port
  -h, --host <HOST>                    Host [default: http://localhost]
  -s, --scrape-period <SCRAPE_PERIOD>  Scrape period [default: 2000]
  -h, --help                           Print help
  -V, --version                        Print version
```

Run `pocket-prometheus --port [PORT]`. This will start a TUI with metrics collector on the background. 
In the TUI, you can query metrics by their name.

Press `escape` to exit the application.

## Example nodejs application

Start the example application.

```bash
$ cd nodejs-example-app/
$ pnpm install
$ pnpm start
```

In another terminal window, trigger `pocket-prometheus --port 3000` and try to type *lag_secong*.
This will query for `nodejs_eventloop_lag_seconds` metric which is exposed from the nodejs 
application.

```
┌────────────────────────────────────────────────────────────────────────────────────────┐
│ Query: lag_second                                                                      │
└────────────────────────────────────────────────────────────────────────────────────────┘
 - nodejs_eventloop_lag_seconds: 0.003553917
after 84s:
 - nodejs_eventloop_lag_seconds: 0.004184417
after 86s:
 - nodejs_eventloop_lag_seconds: 0.004241709
after 88s:
 - nodejs_eventloop_lag_seconds: 0.002958708
after 90s:
 - nodejs_eventloop_lag_seconds: 0.003733917
after 92s:
 - nodejs_eventloop_lag_seconds: 0.0039005
after 94s:
 - nodejs_eventloop_lag_seconds: 0.003489375
after 96s:
 - nodejs_eventloop_lag_seconds: 0.002724208
after 98s:
 - nodejs_eventloop_lag_seconds: 0.003667542
after 100s:
 - nodejs_eventloop_lag_seconds: 0.003900584
after 102s:
 - nodejs_eventloop_lag_seconds: 0.004106667
after 104s:
```
