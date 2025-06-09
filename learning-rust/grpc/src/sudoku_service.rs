use tonic::{Request, Response, Status};
use crate::sudoku::sudoku_server::Sudoku;
use crate::sudoku::Board;

#[derive(Debug, Default)]
pub struct SudokuService {}

#[tonic::async_trait]
impl Sudoku for SudokuService {
    async fn solve(&self, request: Request<Board>) -> Result<Response<Board>, Status> {
        let board = Board {
            values: request.into_inner().values + " SOLVED!"
        };
        Ok(Response::new(board))
    }
}
