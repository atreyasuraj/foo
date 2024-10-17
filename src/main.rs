use std::future::Future;
use std::net::SocketAddr;
use std::str::FromStr;
use autometrics::autometrics;

use prometheus::Encoder;
use tonic::{Request, Response, Status};
use tonic::codegen::Body;
use tonic::transport::{Server, server};
use autometrics::prometheus_exporter::{self, PrometheusResponse};

use axum::{routing::get, Router};
use axum::routing::Route;
use tokio::net::TcpListener;

use proto::calculator_server::{Calculator, CalculatorServer};

use crate::proto::{CalculationRequest, CalculationResponse};

mod proto {
    tonic::include_proto!("calculator");
}

#[derive(Debug, Default)]
struct CalculatorService {}

#[tonic::async_trait]
impl Calculator for CalculatorService {
    #[autometrics]
    async fn add(&self, request: Request<CalculationRequest>) -> Result<Response<CalculationResponse>, Status> {
        println!("Got a request: {:?}", request);

        let input = request.get_ref();

        let response = CalculationResponse {
            result: input.a + input.b,
        };
        Ok(Response::new(response))
    }
}

pub fn get_metrics() -> PrometheusResponse {
    prometheus_exporter::encode_http_response()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    prometheus_exporter::init();
    eprintln!("Prometheus init");
    let addr: SocketAddr = "127.0.0.1:50051".parse().unwrap();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();    let calc = CalculatorService::default();

    tokio::spawn(async move {
        Server::builder()
            .add_service(CalculatorServer::new(calc))
            .serve(addr.into())
            .await
            .unwrap();
    });

    let app = Router::new()
        .route("/", get(handler))
        .route("/metrics", get(|| async { prometheus_exporter::encode_http_response() }));

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn handler() -> &'static str {
    "Hello, World!"
}
