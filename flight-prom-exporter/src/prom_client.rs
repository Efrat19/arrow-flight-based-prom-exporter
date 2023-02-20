
use prometheus_client::registry::Registry;
use arrow_array::{StringArray,RecordBatch,ArrayRef};
use prometheus_client::encoding::text::encode;
use std::sync::Arc;
use arrow_schema::DataType;

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
    // pub async fn serialize_registry_to_arrow_batch(&self) -> RecordBatch {
    //   Field::new("street", DataType::Utf8, false)
    //     let serializedRegistry = DataType::Struct(vec![

    //     ])
    // }
}

// fn serde_struct_type() {
//   use std::collections::HashMap;

//   let kv_array = [("k".to_string(), "v".to_string())];
//   let field_metadata: HashMap<String, String> = kv_array.iter().cloned().collect();

//   // Non-empty map: should be converted as JSON obj { ... }
//   let first_name =
//       Field::new("first_name", DataType::Utf8, false).with_metadata(field_metadata);

//   // Empty map: should be omitted.
//   let last_name = Field::new("last_name", DataType::Utf8, false)
//       .with_metadata(HashMap::default());

//   let person = DataType::Struct(vec![
//       first_name,
//       last_name,
//       Field::new(
//           "address",
//           DataType::Struct(vec![
//               Field::new("street", DataType::Utf8, false),
//               Field::new("zip", DataType::UInt16, false),
//           ]),
//           false,
//       ),
//   ]);

//   let serialized = serde_json::to_string(&person).unwrap();

//   // NOTE that this is testing the default (derived) serialization format, not the
//   // JSON format specified in metadata.md

//   assert_eq!(
//       "{\"Struct\":[\
//        {\"name\":\"first_name\",\"data_type\":\"Utf8\",\"nullable\":false,\"dict_id\":0,\"dict_is_ordered\":false,\"metadata\":{\"k\":\"v\"}},\
//        {\"name\":\"last_name\",\"data_type\":\"Utf8\",\"nullable\":false,\"dict_id\":0,\"dict_is_ordered\":false,\"metadata\":{}},\
//        {\"name\":\"address\",\"data_type\":{\"Struct\":\
//        [{\"name\":\"street\",\"data_type\":\"Utf8\",\"nullable\":false,\"dict_id\":0,\"dict_is_ordered\":false,\"metadata\":{}},\
//        {\"name\":\"zip\",\"data_type\":\"UInt16\",\"nullable\":false,\"dict_id\":0,\"dict_is_ordered\":false,\"metadata\":{}}\
//        ]},\"nullable\":false,\"dict_id\":0,\"dict_is_ordered\":false,\"metadata\":{}}]}",
//       serialized
//   );

//   let deserialized = serde_json::from_str(&serialized).unwrap();

//   assert_eq!(person, deserialized);
// }


// pub fn encode<W>(writer: &mut W, registry: &Registry) -> Result<(), std::fmt::Error>
// where
//     W: Write,
// {
//     for (desc, metric) in registry.iter() {
//         writer.write_str("# HELP ")?;
//         writer.write_str(desc.name())?;
//         if let Some(unit) = desc.unit() {
//             writer.write_str("_")?;
//             writer.write_str(unit.as_str())?;
//         }
//         writer.write_str(" ")?;
//         writer.write_str(desc.help())?;
//         writer.write_str("\n")?;

//         writer.write_str("# TYPE ")?;
//         writer.write_str(desc.name())?;
//         if let Some(unit) = desc.unit() {
//             writer.write_str("_")?;
//             writer.write_str(unit.as_str())?;
//         }
//         writer.write_str(" ")?;
//         writer.write_str(EncodeMetric::metric_type(metric.as_ref()).as_str())?;
//         writer.write_str("\n")?;

//         if let Some(unit) = desc.unit() {
//             writer.write_str("# UNIT ")?;
//             writer.write_str(desc.name())?;
//             writer.write_str("_")?;
//             writer.write_str(unit.as_str())?;
//             writer.write_str(" ")?;
//             writer.write_str(unit.as_str())?;
//             writer.write_str("\n")?;
//         }

//         let encoder = MetricEncoder {
//             writer,
//             name: desc.name(),
//             unit: desc.unit(),
//             const_labels: desc.labels(),
//             family_labels: None,
//         }
//         .into();

//         EncodeMetric::encode(metric.as_ref(), encoder)?;
//     }

//     writer.write_str("# EOF\n")?;

//     Ok(())
// }
