use tonic::transport::Server;

pub mod sudoku {
    tonic::include_proto!("sudoku");
}

pub mod factorial {
    tonic::include_proto!("factorial");
}

pub mod proto {
    pub(crate) const FILE_DESCRIPTOR_SET: &[u8] =
        tonic::include_file_descriptor_set!("descriptor");
}

mod sudoku_service;
mod factorial_service;

use sudoku_service::SudokuService;
use factorial_service::FactorialService;
use crate::{
    sudoku::sudoku_server::SudokuServer,
    factorial::factorial_server::FactorialServer
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "0.0.0.0:50051".parse()?;
    
    // Initialize services
    let sudoku_service = SudokuService::default();
    let factorial_service = FactorialService::default();

    // Set up reflection service
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build_v1alpha()
        .unwrap();

    println!("Server listening on {}", address);
    
    // Start the server with all services
    Server::builder()
        .add_service(SudokuServer::new(sudoku_service))
        .add_service(FactorialServer::new(factorial_service))
        .add_service(reflection_service)
        .serve(address)
        .await?;
    
    Ok(())
}