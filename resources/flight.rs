// // from: https://github.com/apache/arrow/pull/6308/files
// use std::pin::Pin;

// use futures::Stream;
// use tonic::transport::Server;
// use tonic::{Request, Response, Status, Streaming};
// use arrow_flight::{
//     flight_service_server::FlightService, flight_service_server::FlightServiceServer,
//     Action, ActionType, Criteria, Empty, FlightData, FlightDescriptor, FlightInfo,
//     HandshakeRequest, HandshakeResponse, PutResult, SchemaResult, Ticket,
// };
// use prometheus_client::encoding::text::encode;
// use prometheus_client::encoding::{EncodeLabelSet, EncodeLabelValue};
// use prometheus_client::metrics::counter::Counter;
// use prometheus_client::metrics::family::Family;
// use prometheus_client::registry::Registry;
// use serde_arrow::Schema;
// use serde::Serialize;

// #[derive(Clone)]
// pub struct FlightServiceImpl {}

// #[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelValue)]
// pub enum Method {
//     Get,
//     Post,
// }
// pub struct MethodLabels {
//     pub method: Method,
// }

// #[derive(Serialize)]
// pub struct Metrics {
//     requests: Family<MethodLabels, Counter>,
// }

// pub struct AppState {
//     pub registry: Registry,
// }

// impl Metrics {
//     pub fn inc_requests(&self, method: Method) {
//         self.requests.get_or_create(&MethodLabels { method }).inc();
//     }
// }

// #[tonic::async_trait]
// impl FlightService for FlightServiceImpl {
//     type HandshakeStream = Pin<
//         Box<dyn Stream<Item = Result<HandshakeResponse, Status>> + Send + Sync + 'static>,
//     >;
//     type ListFlightsStream =
//         Pin<Box<dyn Stream<Item = Result<FlightInfo, Status>> + Send + Sync + 'static>>;
//     type DoGetStream =
//         Pin<Box<dyn Stream<Item = Result<FlightData, Status>> + Send + Sync + 'static>>;
//     type DoPutStream =
//         Pin<Box<dyn Stream<Item = Result<PutResult, Status>> + Send + Sync + 'static>>;
//     type DoActionStream = Pin<
//         Box<dyn Stream<Item = Result<arrow_flight::Result, Status>> + Send + Sync + 'static>,
//     >;
//     type ListActionsStream =
//         Pin<Box<dyn Stream<Item = Result<ActionType, Status>> + Send + Sync + 'static>>;

//     async fn do_get(
//         &self,
//         request: Request<Ticket>,
//     ) -> Result<Response<Self::DoGetStream>, Status> {
//         let ticket = request.into_inner();
//         match String::from_utf8(ticket.ticket.to_vec()) {
//             Ok(sql) => {
//                 println!("do_get: {}", sql);

//                 // // create local execution context
//                 // let mut ctx = ExecutionContext::new();

//                 // let testdata = std::env::var("PARQUET_TEST_DATA")
//                 //     .expect("PARQUET_TEST_DATA not defined");

//                 // // register parquet file with the execution context
//                 // ctx.register_parquet(
//                 //     "alltypes_plain",
//                 //     &format!("{}/alltypes_plain.parquet", testdata),
//                 // )
//                 // .map_err(|e| to_tonic_err(&e))?;

//                 // // create the query plan
//                 // let plan = ctx
//                 //     .create_logical_plan(&sql)
//                 //     .and_then(|plan| ctx.optimize(&plan))
//                 //     .and_then(|plan| ctx.create_physical_plan(&plan, 1024 * 1024))
//                 //     .map_err(|e| to_tonic_err(&e))?;

//                 // // execute the query
//                 // let results = ctx.collect(plan.as_ref()).map_err(|e| to_tonic_err(&e))?;
//                 // if results.is_empty() {
//                 //     return Err(Status::internal("There were no results from ticket"));
//                 // }

//                 // // add an initial FlightData message that sends schema\

//                 let metrics = web::Data::new(Metrics {
//                     requests: Family::default(),
//                 });
//                 let registry = Registry,
//                 registry.register("requests", "Count of requests", metrics.requests.clone());
//                 metrics.inc_requests(Method::Get);


//                 let schema = Schema::from_records(&metrics)?;
//                 let mut flights: Vec<Result<FlightData, Status>> =
//                     vec![Ok(FlightData::from(schema.as_ref()))];

//                 let mut batches: Vec<Result<FlightData, Status>> = metrics
//                     .iter()
//                     .map(|batch| Ok(FlightData::from(batch)))
//                     .collect();

//                 // // append batch vector to schema vector, so that the first message sent is the schema
//                 flights.append(&mut batches);

//                 let output = futures::stream::iter(flights);

//                 Ok(Response::new(Box::pin(output) as Self::DoGetStream))
//             }
//             Err(e) => Err(Status::invalid_argument(format!("Invalid ticket: {:?}", e))),
//         }
//     }

//     async fn handshake(
//         &self,
//         _request: Request<Streaming<HandshakeRequest>>,
//     ) -> Result<Response<Self::HandshakeStream>, Status> {
//         Err(Status::unimplemented("Not yet implemented"))
//     }

//     async fn list_flights(
//         &self,
//         _request: Request<Criteria>,
//     ) -> Result<Response<Self::ListFlightsStream>, Status> {
//         Err(Status::unimplemented("Not yet implemented"))
//     }

//     async fn get_flight_info(
//         &self,
//         _request: Request<FlightDescriptor>,
//     ) -> Result<Response<FlightInfo>, Status> {
//         Err(Status::unimplemented("Not yet implemented"))
//     }

//     async fn get_schema(
//         &self,
//         _request: Request<FlightDescriptor>,
//     ) -> Result<Response<SchemaResult>, Status> {
//         Err(Status::unimplemented("Not yet implemented"))
//     }

//     async fn do_put(
//         &self,
//         _request: Request<Streaming<FlightData>>,
//     ) -> Result<Response<Self::DoPutStream>, Status> {
//         Err(Status::unimplemented("Not yet implemented"))
//     }

//     async fn do_action(
//         &self,
//         _request: Request<Action>,
//     ) -> Result<Response<Self::DoActionStream>, Status> {
//         Err(Status::unimplemented("Not yet implemented"))
//     }

//     async fn list_actions(
//         &self,
//         _request: Request<Empty>,
//     ) -> Result<Response<Self::ListActionsStream>, Status> {
//         Err(Status::unimplemented("Not yet implemented"))
//     }
// }

// // fn to_tonic_err(e: &datafusion::error::ExecutionError) -> Status {
// //     Status::internal(format!("{:?}", e))
// // }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "0.0.0.0:50051".parse()?;
//     let service = FlightServiceImpl {};

//     let svc = FlightServiceServer::new(service);

//     println!("Listening on {:?}", addr);

//     Server::builder().add_service(svc).serve(addr).await?;

//     Ok(())
// }


// // implement registry schema
// // 