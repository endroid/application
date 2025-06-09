use tonic::{Request, Response, Status};
use crate::proto::factorial::factorial_server::Factorial;
use crate::proto::factorial::{FactorialRequest, FactorialResponse};

/// Service that calculates factorial of a given number
/// 
/// Provides mathematical factorial computation (n!) where n! = n × (n-1) × ... × 2 × 1
/// Uses saturating multiplication to prevent overflow for very large numbers.
#[derive(Debug, Default)]
pub struct FactorialCalculatorService {}

#[tonic::async_trait]
impl Factorial for FactorialCalculatorService {
    /// Calculate the factorial of a number
    /// 
    /// # Arguments
    /// * `request` - Contains the number to calculate factorial for
    /// 
    /// # Returns
    /// * `Response<FactorialResponse>` - Contains the calculated factorial result
    /// * `Status::invalid_argument` - If the input number is negative
    /// 
    /// # Note
    /// Uses saturating multiplication which caps at u64::MAX to prevent overflow
    async fn calculate_factorial(
        &self,
        request: Request<FactorialRequest>,
    ) -> Result<Response<FactorialResponse>, Status> {
        let input_number = request.into_inner().number;
        
        println!("🔢 Received factorial calculation request");
        println!("   Input: {}", input_number);
        
        // Validate input: factorial is only defined for non-negative integers
        if input_number < 0 {
            println!("❌ Invalid input: negative number ({})", input_number);
            return Err(Status::invalid_argument(
                "Factorial is not defined for negative numbers"
            ));
        }

        // Measure calculation time
        let start_time = std::time::Instant::now();
        
        // Calculate factorial using fold with saturating multiplication
        // This prevents overflow by capping at the maximum value
        let factorial_result = (1..=input_number as u64)
            .fold(1u64, |accumulator, current_number| {
                accumulator.saturating_mul(current_number)
            });
        
        let calc_time = start_time.elapsed();
        
        let response = FactorialResponse {
            result: factorial_result as i64,
        };

        println!("✅ Factorial calculated successfully!");
        println!("   ⏱️  Calculation time: {:?}", calc_time);
        println!("   🎯 Result: {}! = {}", input_number, factorial_result);

        Ok(Response::new(response))
    }
}
