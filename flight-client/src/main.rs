use arrow_flight::utils::flight_data_to_arrow_batch;
use arrow_flight::flight_service_client::FlightServiceClient;
use arrow_flight::{Ticket,FlightData};
use std::sync::Arc;
use arrow_schema::{ArrowError, Schema};
use arrow_ipc::convert::fb_to_schema;
use arrow_ipc::{ root_as_message};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FlightServiceClient::connect("http://localhost:50052").await?;

    let request = tonic::Request::new(Ticket {
        ticket: "get_metrics".into(),
    });

    let mut stream = client.do_get(request).await?.into_inner();

    let flight_data = stream.message().await?.unwrap();
    // the schema should be the first message returned, else client should error
    let schema = Arc::new(get_schema(&[flight_data])?);
    println!("Schema: {:?}", schema);

    let dictionaries_by_id = HashMap::new();
    // all the remaining stream messages should be dictionary and record batches
    while let Some(flight_data) = stream.message().await? {
        // the unwrap is infallible and thus safe
        let record_batch = flight_data_to_arrow_batch(&flight_data, schema.clone(), &dictionaries_by_id)?;

        println!(
            "record_batch has {} columns and {} rows",
            record_batch.num_columns(),
            record_batch.num_rows()
        );
        println!("BATCH={:?}", record_batch);
    }
    Ok(())
}



fn get_schema(
    flight_data: &[FlightData],
) -> Result<Schema, ArrowError> {
    let schema = flight_data.get(0).ok_or_else(|| {
        ArrowError::CastError("Need at least one FlightData for schema".to_string())
    })?;
    let message = root_as_message(&schema.data_header[..])
        .map_err(|_| ArrowError::CastError("Cannot get root as message".to_string()))?;

    let ipc_schema: arrow_ipc::Schema = message.header_as_schema().ok_or_else(|| {
        ArrowError::CastError("Cannot get header as Schema".to_string())
    })?;
    let schema = fb_to_schema(ipc_schema);
    Ok(schema)
}