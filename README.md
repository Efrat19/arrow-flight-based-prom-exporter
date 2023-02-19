# Arrow-flight based Promethus Exporter
For the sake of learning rust and Apache Arrow/flight (and for fun) I wrote an experimental flight-based exporter, ready for the day Prometheus will support flight-based gRPC.

## Why
Arrow-flight is a gRPC based protocol, originally developed as a more efficient alternative to JDBC/ODBC.
With the adoption of monitoring best practices and the rising of prometheus scraping-based metrics collection, metrics network volume is expected to grow heavier and require more effieciency.
Luckily flight comes with 2 effeciency advantages:
- Data is streamed over gRPC instead of openmetrics HTTP
- While supporting regular gRPC clients as well, a flight server + flight client will perform de-deuplication on the data using flight requests. This is a significant optimization for openmetrics exporters which send lots of redundant metrics over scrape intervals. 

## Run
