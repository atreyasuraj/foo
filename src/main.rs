use tonic::{Request, Response, Status};
use tonic::transport::Server;

use proto::calculator_server::{Calculator, CalculatorServer};

use crate::proto::{CalculationRequest, CalculationResponse, FILE_DESCRIPTOR_SET};

mod proto{
    tonic::include_proto!("calculator");

    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("calculator_descriptor");
}

#[derive(Debug, Default)]
struct CalculatorService{}

#[tonic::async_trait]
impl Calculator for CalculatorService{
    async fn add(&self, request: Request<CalculationRequest>) -> Result<Response<CalculationResponse>, Status> {
        println!("Got a request: {:?}", request);

        let input = request.get_ref();

        let response = CalculationResponse{
            result: input.a + input.b,
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:50051".parse()?;
    let calc = CalculatorService::default();

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    Server::builder()
        .add_service(service)
        .add_service(CalculatorServer::new(calc))
        .serve(addr)
        .await?;

    Ok(())
}