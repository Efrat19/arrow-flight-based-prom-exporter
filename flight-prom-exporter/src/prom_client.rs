
use prometheus_client::registry::Registry;
use prometheus_client::metrics::counter::{Counter};

fn get_registry() -> Registry {
    let mut registry = Registry::default();
    let counter: Counter = Counter::default();
    registry.register(
        "my_counter",
        "This is my counter",
        counter.clone(),
      );
      counter.inc();
    registry
}
