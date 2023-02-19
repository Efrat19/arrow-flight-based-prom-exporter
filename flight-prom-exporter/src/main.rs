
use tonic::transport::Server;
use prometheus_client::registry::Registry;
use arrow_flight::{
    flight_service_server::FlightServiceServer};
use prometheus_client::metrics::counter::{Counter};

mod flight_service;
mod prom_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut prometheus_client = prom_client::PromClient {
        registry: Registry::default(),
    };
    let counter: Counter = Counter::default();
    counter.inc();
    prometheus_client.registry.register(
        "my_counter",
        "This is my counter",
        counter.clone(),
    );
      
    let addr = "[::1]:50052".parse()?;
    let service = flight_service::PromClientFlightService {
        prom_client: prometheus_client
    };

    let svc = FlightServiceServer::new(service);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}