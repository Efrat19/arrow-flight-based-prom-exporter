
use arrow_flight::{
    flight_service_server::FlightService, HandshakeResponse, FlightInfo,
    FlightData, PutResult, HandshakeRequest, FlightDescriptor, SchemaResult,
    Ticket, Action,Empty, Criteria, ActionType};
use futures::{stream, Stream};
use std::pin::Pin;
use tonic::{Request, Response, Status, Streaming};
use arrow_array::{RecordBatch};
use arrow_flight::utils::batches_to_flight_data;

use crate::prom_client::PromClient;

macro_rules! status {
    ($desc:expr, $err:expr) => {
        Status::internal(format!("{}: {} at {}:{}", $desc, $err, file!(), line!()))
    };
}

// #[derive(Clone)]
pub struct PromClientFlightService {
    pub prom_client: PromClient
}

#[tonic::async_trait]
impl FlightService for PromClientFlightService {
    type HandshakeStream = Pin<
        Box<dyn Stream<Item = Result<HandshakeResponse, Status>> + Send + Sync + 'static>,
    >;
    type ListFlightsStream =
        Pin<Box<dyn Stream<Item = Result<FlightInfo, Status>> + Send + Sync + 'static>>;
    type DoGetStream =
        Pin<Box<dyn Stream<Item = Result<FlightData, Status>> + Send + Sync + 'static>>;
    type DoPutStream =
        Pin<Box<dyn Stream<Item = Result<PutResult, Status>> + Send + Sync + 'static>>;
    type DoActionStream = Pin<
        Box<
            dyn Stream<Item = Result<arrow_flight::Result, Status>>
                + Send
                + Sync
                + 'static,
        >,
    >;
    type ListActionsStream =
        Pin<Box<dyn Stream<Item = Result<ActionType, Status>> + Send + Sync + 'static>>;
    type DoExchangeStream =
        Pin<Box<dyn Stream<Item = Result<FlightData, Status>> + Send + Sync + 'static>>;

    async fn handshake(
        &self,
        _request: Request<Streaming<HandshakeRequest>>,
    ) -> Result<Response<Self::HandshakeStream>, Status> {
        Err(Status::unimplemented("Implement handshake"))
    }

    async fn list_flights(
        &self,
        _request: Request<Criteria>,
    ) -> Result<Response<Self::ListFlightsStream>, Status> {
        Err(Status::unimplemented("Implement list_flights"))
    }

    async fn get_flight_info(
        &self,
        _request: Request<FlightDescriptor>,
    ) -> Result<Response<FlightInfo>, Status> {
        Err(Status::unimplemented("Implement get_flight_info"))
    }

    async fn get_schema(
        &self,
        _request: Request<FlightDescriptor>,
    ) -> Result<Response<SchemaResult>, Status> {
        Err(Status::unimplemented("Implement get_schema"))
    }

    async fn do_get(
        &self,
        _request: Request<Ticket>,
    ) -> Result<Response<Self::DoGetStream>, Status> {
        let batch = self.prom_client.get_registry_as_arrow_batch().await;
        let stream = batch_to_stream(batch).await.unwrap();
        let resp = Response::new(stream);
        Ok(resp)
    }

    async fn do_put(
        &self,
        _request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoPutStream>, Status> {
        Err(Status::unimplemented("Implement do_put"))
    }

    async fn do_action(
        &self,
        _request: Request<Action>,
    ) -> Result<Response<Self::DoActionStream>, Status> {
        Err(Status::unimplemented("Implement do_action"))
    }

    async fn list_actions(
        &self,
        _request: Request<Empty>,
    ) -> Result<Response<Self::ListActionsStream>, Status> {
        Err(Status::unimplemented("Implement list_actions"))
    }

    async fn do_exchange(
        &self,
        _request: Request<Streaming<FlightData>>,
    ) -> Result<Response<Self::DoExchangeStream>, Status> {
        Err(Status::unimplemented("Implement do_exchange"))
    }
}

pub async fn batch_to_stream(batch: RecordBatch) -> Result<Pin<Box<dyn Stream<Item = Result<FlightData, Status>> + Send + Sync>>, Status> {
    let schema = (*batch.schema()).clone();
    let batches = vec![batch];
    let flight_data = batches_to_flight_data(schema, batches)
        .map_err(|e| status!("Could not convert batches", e))?
        .into_iter()
        .map(Ok);
    let stream: Pin<Box<dyn Stream<Item = Result<FlightData, Status>> + Send + Sync>> =
        Box::pin(stream::iter(flight_data));
        Ok(stream)
}
