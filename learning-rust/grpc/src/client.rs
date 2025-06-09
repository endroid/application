use crate::sudoku::sudoku_client::SudokuClient;
use crate::factorial::factorial_client::FactorialClient;

pub mod sudoku {
    tonic::include_proto!("sudoku");
}

pub mod factorial {
    tonic::include_proto!("factorial");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to the server
    let channel = tonic::transport::Channel::from_static("http://0.0.0.0:50051")
        .connect()
        .await?;
    
    // Call Sudoku service
    let mut sudoku_client = SudokuClient::new(channel.clone());
    let sudoku_request = tonic::Request::new(sudoku::Board {
        values: "1,2,3,4,5,6,7,8,9".to_string(),
    });
    
    let start = std::time::Instant::now();
    let sudoku_response = sudoku_client.solve(sudoku_request).await?;
    let duration = start.elapsed();
    println!("Sudoku solve took: {:?}", duration);
    println!("Sudoku response: {:?}", sudoku_response.into_inner().values);
    
    // Call Factorial service
    let mut factorial_client = FactorialClient::new(channel);
    let factorial_request = tonic::Request::new(factorial::FactorialRequest {
        number: 8,
    });
    
    let start = std::time::Instant::now();
    let factorial_response = factorial_client.calculate_factorial(factorial_request).await?;
    let duration = start.elapsed();
    println!("\nFactorial calculation took: {:?}", duration);
    println!("Factorial of 8 is: {}", factorial_response.into_inner().result);
    
    Ok(())
}