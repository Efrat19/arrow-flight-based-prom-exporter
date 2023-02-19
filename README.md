# Arrow-flight based Promethus Exporter
For the sake of learning rust and Apache Arrow/flight (and for fun) I wrote an experimental flight-based exporter, ready for the day Prometheus will support flight-based gRPC.

## Why
[Arrow-flight](https://arrow.apache.org/blog/2019/10/13/introducing-arrow-flight) is a gRPC based protocol, originally developed as a more efficient alternative to JDBC/ODBC.
With the adoption of monitoring best practices and the rising popularity of prometheus and scraping-based metrics collection, metrics network volume is expected to grow heavier and require some optimizations.
Luckily flight comes with 2 effeciency advantages:
- Data is streamed over gRPC instead of openmetrics HTTP
- While supporting regular gRPC clients as well, a flight server + flight client will perform de-deuplications on the data using flight requests. This is a significant optimization for openmetrics exporters sending lots of redundant metrics over scrape intervals. 

## Run
```bash
# Open server on the #1 terminal window:
cargo run --bin flight-prom-exporter --manifest-path flight-prom-exporter/Cargo.toml 
# View results on the #2 terminal window:
cargo run --bin flight-client --manifest-path flight-client/Cargo.toml
```

## Helpful sources
- [prometheus rust client](https://github.com/prometheus/client_rust)
- [flight rust src](https://github.com/apache/arrow-rs/tree/master/arrow-flight)
- [some example](https://github.com/apache/arrow/pull/6308/files)
