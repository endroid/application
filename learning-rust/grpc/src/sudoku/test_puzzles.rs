/// Collection of test Sudoku puzzles of various difficulty levels
/// 
/// These puzzles are sourced from well-known puzzle creators and include
/// the famous AI Escargot by Arto Inkala, considered one of the hardest puzzles ever created.

use super::{SudokuBoard, SudokuComplexity};

pub struct TestPuzzles;

impl TestPuzzles {
    /// Easy Sudoku puzzle - good for testing basic functionality
    pub fn easy() -> SudokuBoard {
        let puzzle = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79";
        SudokuBoard::from_string(puzzle).unwrap()
    }

    /// Medium difficulty puzzle
    pub fn medium() -> SudokuBoard {
        let puzzle = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..7.";
        SudokuBoard::from_string(puzzle).unwrap()
    }

    /// Hard puzzle that requires advanced techniques
    pub fn hard() -> SudokuBoard {
        let puzzle = ".......1.4.........6.2......................................5.8.........3..6.....";
        SudokuBoard::from_string(puzzle).unwrap()
    }

    /// The famous AI Escargot by Arto Inkala - one of the hardest puzzles ever created
    /// 
    /// This puzzle was designed by Finnish mathematician Arto Inkala and is known
    /// for requiring solvers to consider multiple logical relationships simultaneously.
    /// It has been featured in various media as an example of an extremely difficult Sudoku.
    pub fn ai_escargot() -> SudokuBoard {
        let puzzle = "1....7.9..3..2...8..96..5....53..9...1..8...26....4...3......1..41....7...7...3..";
        SudokuBoard::from_string(puzzle).unwrap()
    }

    /// Another very hard puzzle - World's Hardest Sudoku 2012 by Arto Inkala  
    pub fn worlds_hardest_2012() -> SudokuBoard {
        let puzzle = "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..7.";
        SudokuBoard::from_string(puzzle).unwrap()
    }

    /// A minimal 17-clue Sudoku (the minimum number of clues for a unique solution)
    pub fn minimal_17_clue() -> SudokuBoard {
        let puzzle = "..........3.85...1.2......5.7.....4...1...9.......5.......73..2.1.......4...9....";
        SudokuBoard::from_string(puzzle).unwrap()
    }

    /// Get all test puzzles with their difficulty labels
    pub fn all_puzzles() -> Vec<(String, SudokuBoard, SudokuComplexity)> {
        vec![
            ("Easy Test".to_string(), Self::easy(), SudokuComplexity::Easy),
            ("Medium Test".to_string(), Self::medium(), SudokuComplexity::Medium),
            ("Hard Test".to_string(), Self::hard(), SudokuComplexity::Hard),
            ("AI Escargot (Arto Inkala)".to_string(), Self::ai_escargot(), SudokuComplexity::Expert),
            ("World's Hardest 2012 (Arto Inkala)".to_string(), Self::worlds_hardest_2012(), SudokuComplexity::Expert),
            ("Minimal 17-Clue".to_string(), Self::minimal_17_clue(), SudokuComplexity::Expert),
        ]
    }

    /// Get a puzzle by difficulty level
    pub fn by_difficulty(difficulty: SudokuComplexity) -> SudokuBoard {
        match difficulty {
            SudokuComplexity::Easy => Self::easy(),
            SudokuComplexity::Medium => Self::medium(),
            SudokuComplexity::Hard => Self::hard(),
            SudokuComplexity::Expert => Self::ai_escargot(),
        }
    }

    /// Get a random puzzle (cycles through difficulties)
    pub fn random(seed: usize) -> (String, SudokuBoard) {
        let puzzles = Self::all_puzzles();
        let index = seed % puzzles.len();
        let (name, board, _) = &puzzles[index];
        (name.clone(), board.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sudoku::SudokuSolver;

    #[test]
    fn test_all_puzzles_are_valid() {
        for (name, board, _) in TestPuzzles::all_puzzles() {
            assert!(board.is_valid(), "Puzzle '{}' is not valid", name);
        }
    }

    #[test]
    fn test_easy_puzzle_solvable() {
        let solver = SudokuSolver::new();
        let easy_puzzle = TestPuzzles::easy();
        let solution = solver.solve(&easy_puzzle);
        assert!(solution.is_some(), "Easy puzzle should be solvable");
        assert!(solver.validate_solution(&solution.unwrap()));
    }

    #[test]
    fn test_medium_puzzle_solvable() {
        let solver = SudokuSolver::new();
        let medium_puzzle = TestPuzzles::medium();
        let solution = solver.solve(&medium_puzzle);
        assert!(solution.is_some(), "Medium puzzle should be solvable");
        assert!(solver.validate_solution(&solution.unwrap()));
    }

    #[test]
    #[ignore] // This test takes a long time - run with: cargo test -- --ignored
    fn test_ai_escargot_solvable() {
        let solver = SudokuSolver::with_max_iterations(10_000_000); // Increase limit for hard puzzle
        let ai_escargot = TestPuzzles::ai_escargot();
        let solution = solver.solve(&ai_escargot);
        assert!(solution.is_some(), "AI Escargot should be solvable");
        assert!(solver.validate_solution(&solution.unwrap()));
    }
}
