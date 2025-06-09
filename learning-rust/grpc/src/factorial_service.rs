use tonic::{Request, Response, Status};
use crate::factorial::factorial_server::Factorial;
use crate::factorial::{FactorialRequest, FactorialResponse};

#[derive(Debug, Default)]
pub struct FactorialService {}

#[tonic::async_trait]
impl Factorial for FactorialService {
    async fn calculate_factorial(
        &self,
        request: Request<FactorialRequest>,
    ) -> Result<Response<FactorialResponse>, Status> {
        let number = request.into_inner().number;
        
        if number < 0 {
            return Err(Status::invalid_argument("Number must be non-negative"));
        }

        let result = (1..=number as u64)
            .fold(1u64, |acc, x| acc.saturating_mul(x));
        
        let response = FactorialResponse {
            result: result as i64,
        };

        Ok(Response::new(response))
    }
}
