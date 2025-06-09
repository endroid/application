/// gRPC server that hosts Sudoku and Factorial calculation services
/// 
/// This server provides:
/// - Sudoku solving service (currently a mock implementation)
/// - Factorial calculation service  
/// - gRPC reflection for service discovery and debugging

use tonic::transport::Server;
use grpc::{SudokuSolverService, FactorialCalculatorService};
use grpc::proto::{
    sudoku::sudoku_server::SudokuServer,
    factorial::factorial_server::FactorialServer,
    proto::FILE_DESCRIPTOR_SET
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse and validate the server address
    let server_address = "0.0.0.0:50051".parse()
        .map_err(|e| format!("Invalid server address: {}", e))?;
    
    println!("🚀 Initializing gRPC server...");
    
    // Create service instances
    // These implement the actual business logic for each service
    let sudoku_solver = SudokuSolverService::default();
    let factorial_calculator = FactorialCalculatorService::default();
    println!("✅ Service instances created");

    // Configure gRPC reflection service for debugging and client tooling
    // This allows clients to discover available services and their schemas
    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build_v1alpha()
        .expect("Failed to build reflection service");
    println!("✅ gRPC reflection service configured");

    println!("🌐 Server starting on {}", server_address);
    println!("📋 Available services:");
    println!("   - Sudoku solver service (with timing)");
    println!("   - Factorial calculator service (with timing)");
    println!("   - gRPC reflection service");
    println!("\n⚡ Server ready to accept connections...");
    println!("📝 Server will log all requests and timing information\n");
    
    // Build and start the server with all registered services
    Server::builder()
        .add_service(SudokuServer::new(sudoku_solver))
        .add_service(FactorialServer::new(factorial_calculator))
        .add_service(reflection_service)
        .serve(server_address)
        .await
        .map_err(|e| format!("Server failed to start: {}", e))?;
    
    Ok(())
}
