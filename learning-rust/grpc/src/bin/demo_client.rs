/// Demo client that showcases both Sudoku and Factorial gRPC services
/// 
/// This client connects to the gRPC server and demonstrates calling both services
/// with sample data while measuring response times.

use grpc::proto::{sudoku, factorial};
use grpc::proto::sudoku::sudoku_client::SudokuClient;
use grpc::proto::factorial::factorial_client::FactorialClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Starting gRPC demo client...\n");
    
    // Establish connection to the gRPC server
    let server_address = "http://0.0.0.0:50051";
    println!("📡 Connecting to server at {}", server_address);
    
    let channel = tonic::transport::Channel::from_static(server_address)
        .connect()
        .await
        .map_err(|e| format!("Failed to connect to server: {}", e))?;
    
    // Demo 1: Test Sudoku solving service with real puzzles
    println!("\n🧩 Testing Sudoku solver service...");
    let mut sudoku_client = SudokuClient::new(channel.clone());
    
    // Test with an easy puzzle first in single-line format
    let easy_puzzle = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
    
    println!("   🟢 Testing Easy puzzle...");
    println!("      Input puzzle: {}", easy_puzzle);
    // Display as 9x9 grid
    if let Ok(input_board) = grpc::SudokuBoard::from_string(easy_puzzle) {
        for line in input_board.to_matrix_string().lines() {
            println!("         {}", line);
        }
    }
    
    let sudoku_request = tonic::Request::new(sudoku::Board {
        values: easy_puzzle.to_string(),
    });
    
    let start_time = std::time::Instant::now();
    match sudoku_client.solve(sudoku_request).await {
        Ok(sudoku_response) => {
            let elapsed_time = start_time.elapsed();
            let solution = sudoku_response.into_inner().values;
            println!("   ✅ Easy puzzle solved!");
            println!("   📊 Empty cells filled: {} → 81", easy_puzzle.matches('.').count());
            println!("   ⏱️  Solve time: {:?}", elapsed_time);
            
            // Parse solution and display nicely
            println!("      Solution: {}", solution);
            if let Ok(solved_board) = grpc::SudokuBoard::from_string(&solution) {
                for line in solved_board.to_matrix_string().lines() {
                    println!("         {}", line);
                }
            }
        }
        Err(e) => {
            println!("   ❌ Failed to solve easy puzzle: {}", e.message());
        }
    }
    
    // Test with a harder puzzle
    println!("\n   🟡 Testing Medium puzzle...");
    let medium_puzzle = "...6..4..7....36......91.8..........5.18...3...3.6.45.4.2...6.9.3.......2...4...";
    
    println!("      Input puzzle: {}", medium_puzzle);
    if let Ok(input_board) = grpc::SudokuBoard::from_string(medium_puzzle) {
        for line in input_board.to_matrix_string().lines() {
            println!("         {}", line);
        }
    }
    
    let sudoku_request = tonic::Request::new(sudoku::Board {
        values: medium_puzzle.to_string(),
    });
    
    let start_time = std::time::Instant::now();
    match sudoku_client.solve(sudoku_request).await {
        Ok(sudoku_response) => {
            let elapsed_time = start_time.elapsed();
            let solution = sudoku_response.into_inner().values;
            println!("   ✅ Medium puzzle solved!");
            println!("   📊 Empty cells filled: {} → 81", medium_puzzle.matches('.').count());
            println!("   ⏱️  Solve time: {:?}", elapsed_time);
            
            // Parse solution and display nicely  
            println!("      Solution: {}", solution);
            if let Ok(solved_board) = grpc::SudokuBoard::from_string(&solution) {
                for line in solved_board.to_matrix_string().lines() {
                    println!("         {}", line);
                }
            }
        }
        Err(e) => {
            println!("   ⚠️  Medium puzzle: {}", e.message());
        }
    }
    
    // Demo 2: Test Factorial calculation service
    println!("\n🔢 Testing factorial calculator service...");
    let mut factorial_client = FactorialClient::new(channel);
    
    let test_number = 8;
    let factorial_request = tonic::Request::new(factorial::FactorialRequest {
        number: test_number,
    });
    
    let start_time = std::time::Instant::now();
    let factorial_response = factorial_client.calculate_factorial(factorial_request).await?;
    let elapsed_time = start_time.elapsed();
    
    println!("   Input number: {}", test_number);
    println!("   Factorial result: {}", factorial_response.into_inner().result);
    println!("   ⏱️  Calculation time: {:?}", elapsed_time);
    
    println!("\n✅ Demo completed successfully!");
    Ok(())
}
