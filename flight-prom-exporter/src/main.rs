
use tonic::transport::Server;
use arrow_flight::{
    flight_service_server::FlightServiceServer};

mod flight_service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50052".parse()?;
    let service = flight_service::FlightServiceImpl {};

    let svc = FlightServiceServer::new(service);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}