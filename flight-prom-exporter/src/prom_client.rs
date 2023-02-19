
use prometheus_client::registry::Registry;
use arrow_array::{StringArray,RecordBatch,ArrayRef};
use prometheus_client::encoding::text::encode;
use std::sync::Arc;

// #[derive(Clone)]
pub struct PromClient{
    pub registry: Registry
}

impl PromClient {
    // pub fn get_registry(&self) -> &Registry {
    //     &(self.registry)
    // }
    pub async fn get_registry_as_arrow_batch(&self) -> RecordBatch {
        let mut string_ourput = String::new();
        encode(&mut string_ourput, &self.registry).unwrap();
        let metrics: Vec<&str> = string_ourput.split("\n").collect();
        let string_array = StringArray::from(metrics);
        let metrics_ref: ArrayRef = Arc::new(string_array);
        let record_batch = RecordBatch::try_from_iter(vec![
          ("metrics_ref", metrics_ref),
        ]).unwrap();
        record_batch
    }
}
