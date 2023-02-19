// use std::sync::Mutex;

// use actix_web::{web, App, HttpResponse, HttpServer, Responder, Result};
// use prometheus_client::encoding::text::encode;
// use prometheus_client::encoding::{EncodeLabelSet, EncodeLabelValue};
// use prometheus_client::metrics::counter::Counter;
// use prometheus_client::metrics::family::Family;
// use prometheus_client::registry::Registry;

// #[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelValue)]
// pub enum Method {
//     Get,
//     Post,
// }

// #[derive(Clone, Debug, Hash, PartialEq, Eq, EncodeLabelSet)]
// pub struct MethodLabels {
//     pub method: Method,
// }

// pub struct Metrics {
//     requests: Family<MethodLabels, Counter>,
// }

// impl Metrics {
//     pub fn inc_requests(&self, method: Method) {
//         self.requests.get_or_create(&MethodLabels { method }).inc();
//     }
// }

// pub struct AppState {
//     pub registry: Registry,
// }

// pub async fn metrics_handler(state: web::Data<Mutex<AppState>>,metrics: web::Data<Metrics>) -> Result<HttpResponse> {
//     metrics.inc_requests(Method::Get);
//     let state = state.lock().unwrap();
//     let mut body = String::new();
//     encode(&mut body, &state.registry).unwrap();
//     Ok(HttpResponse::Ok()
//         .content_type("application/openmetrics-text; version=1.0.0; charset=utf-8")
//         .body(body))
// }


// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     let metrics = web::Data::new(Metrics {
//         requests: Family::default(),
//     });
//     let mut state = AppState {
//         registry: Registry::default(),
//     };
//     state
//         .registry
//         .register("requests", "Count of requests", metrics.requests.clone());
//     let state = web::Data::new(Mutex::new(state));

//     HttpServer::new(move || {
//         App::new()
//             .app_data(metrics.clone())
//             .app_data(state.clone())
//             .service(web::resource("/metrics").route(web::get().to(metrics_handler)))
//     })
//     .bind(("127.0.0.1", 9050))?
//     .run()
//     .await
// }