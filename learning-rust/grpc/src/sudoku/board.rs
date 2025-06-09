/// Represents a 9x9 Sudoku board and provides basic operations
/// 
/// The board uses 0 to represent empty cells and 1-9 for filled cells.
/// Provides methods to parse from string format and convert back to string.

use std::fmt;

pub type Cell = u8;
pub type Grid = [[Cell; 9]; 9];

#[derive(Debug, Clone, PartialEq)]
pub struct SudokuBoard {
    grid: Grid,
}

impl SudokuBoard {
    /// Create a new empty Sudoku board
    pub fn new() -> Self {
        Self {
            grid: [[0; 9]; 9],
        }
    }

    /// Create a board from various string formats
    /// 
    /// Supports multiple input formats:
    /// 1. Comma-separated: "1,2,0,4,0,6,7,8,9,2,3,4,..." where 0 represents empty cells
    /// 2. 9x9 matrix format with line breaks:
    ///    ```text
    ///    530070000
    ///    600195000
    ///    098000060
    ///    800060003
    ///    400803001
    ///    700020006
    ///    060000280
    ///    000419005
    ///    000080079
    ///    ```
    /// 3. Matrix with spaces/dots: "5 3 . . 7 . . . ."
    /// 
    /// Returns error if the input is invalid (wrong length, invalid numbers)
    pub fn from_string(input: &str) -> Result<Self, String> {
        // Try different parsing strategies
        if input.contains(',') {
            Self::parse_comma_separated(input)
        } else {
            Self::parse_matrix_format(input)
        }
    }

    /// Parse comma-separated format: "1,2,0,4,0,6,7,8,9,..."
    fn parse_comma_separated(input: &str) -> Result<Self, String> {
        let values: Result<Vec<Cell>, _> = input
            .split(',')
            .map(|s| s.trim().parse::<Cell>())
            .collect();
        
        let values = values.map_err(|_| "Invalid number format in comma-separated input")?;
        
        if values.len() != 81 {
            return Err(format!("Expected 81 values, got {}", values.len()));
        }
        
        Self::create_grid_from_values(&values)
    }

    /// Parse matrix format with line breaks and various separators
    fn parse_matrix_format(input: &str) -> Result<Self, String> {
        let mut values = Vec::new();
        
        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue; // Skip empty lines
            }
            
            // Split by spaces, tabs, or treat each character as a cell
            let chars: Vec<char> = if line.contains(' ') || line.contains('\t') {
                // Space or tab separated
                line.split_whitespace()
                    .map(|s| s.chars().next().unwrap_or('0'))
                    .collect()
            } else {
                // Each character is a cell
                line.chars().collect()
            };
            
            for ch in chars {
                let value = match ch {
                    '0'..='9' => ch.to_digit(10).unwrap() as Cell,
                    '.' | '_' | ' ' => 0, // Treat dots, underscores, spaces as empty
                    _ => return Err(format!("Invalid character '{}' in puzzle", ch)),
                };
                values.push(value);
            }
        }
        
        if values.len() != 81 {
            return Err(format!("Expected 81 values, got {} (matrix should be 9x9)", values.len()));
        }
        
        Self::create_grid_from_values(&values)
    }

    /// Create grid from a vector of 81 values
    fn create_grid_from_values(values: &[Cell]) -> Result<Self, String> {
        // Validate that all values are 0-9
        for &value in values {
            if value > 9 {
                return Err(format!("Invalid value: {}. Values must be 0-9", value));
            }
        }
        
        let mut grid = [[0; 9]; 9];
        for (i, &value) in values.iter().enumerate() {
            let row = i / 9;
            let col = i % 9;
            grid[row][col] = value;
        }
        
        Ok(Self { grid })
    }

    /// Convert the board to a single-line string without commas
    /// 
    /// Uses dots (.) for empty cells and numbers 1-9 for filled cells.
    /// Returns 81 characters representing the 9x9 grid row by row.
    pub fn to_string(&self) -> String {
        let mut result = String::with_capacity(81);
        for row in 0..9 {
            for col in 0..9 {
                let cell = if self.grid[row][col] == 0 {
                    '.'
                } else {
                    char::from_digit(self.grid[row][col] as u32, 10).unwrap()
                };
                result.push(cell);
            }
        }
        result
    }

    /// Convert the board to a comma-separated string (for legacy compatibility)
    pub fn to_comma_string(&self) -> String {
        let mut values = Vec::with_capacity(81);
        for row in 0..9 {
            for col in 0..9 {
                values.push(self.grid[row][col].to_string());
            }
        }
        values.join(",")
    }

    /// Convert the board to a human-readable 9x9 matrix format with line breaks
    pub fn to_matrix_string(&self) -> String {
        let mut result = String::new();
        for row in 0..9 {
            for col in 0..9 {
                let cell = if self.grid[row][col] == 0 {
                    ".".to_string()
                } else {
                    self.grid[row][col].to_string()
                };
                result.push_str(&cell);
            }
            if row < 8 {
                result.push('\n');
            }
        }
        result
    }

    /// Convert the board to a nicely formatted display with grid lines
    pub fn to_pretty_string(&self) -> String {
        let mut result = String::new();
        
        for row in 0..9 {
            if row % 3 == 0 {
                result.push_str("┌─────┬─────┬─────┐\n");
            } else {
                result.push_str("├─────┼─────┼─────┤\n");
            }
            
            result.push('│');
            for col in 0..9 {
                let cell = if self.grid[row][col] == 0 {
                    " ".to_string()
                } else {
                    self.grid[row][col].to_string()
                };
                
                result.push(' ');
                result.push_str(&cell);
                
                if col % 3 == 2 {
                    result.push_str(" │");
                } else {
                    result.push(' ');
                }
            }
            result.push('\n');
        }
        result.push_str("└─────┴─────┴─────┘");
        result
    }

    /// Get the value at a specific position
    pub fn get(&self, row: usize, col: usize) -> Cell {
        self.grid[row][col]
    }

    /// Set the value at a specific position
    pub fn set(&mut self, row: usize, col: usize, value: Cell) {
        self.grid[row][col] = value;
    }

    /// Check if a value can be placed at a specific position
    pub fn is_valid_move(&self, row: usize, col: usize, value: Cell) -> bool {
        if value == 0 || value > 9 {
            return false;
        }
        
        // Check if cell is already occupied
        if self.grid[row][col] != 0 {
            return false;
        }
        
        // Check row constraint
        for c in 0..9 {
            if self.grid[row][c] == value {
                return false;
            }
        }
        
        // Check column constraint
        for r in 0..9 {
            if self.grid[r][col] == value {
                return false;
            }
        }
        
        // Check 3x3 box constraint
        let box_row = (row / 3) * 3;
        let box_col = (col / 3) * 3;
        for r in box_row..box_row + 3 {
            for c in box_col..box_col + 3 {
                if self.grid[r][c] == value {
                    return false;
                }
            }
        }
        
        true
    }

    /// Check if the board is completely filled
    pub fn is_complete(&self) -> bool {
        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col] == 0 {
                    return false;
                }
            }
        }
        true
    }

    /// Check if the current board state is valid (no conflicts)
    pub fn is_valid(&self) -> bool {
        // Check all rows
        for row in 0..9 {
            let mut seen = [false; 10]; // Index 0 unused, 1-9 for values
            for col in 0..9 {
                let value = self.grid[row][col];
                if value != 0 {
                    if seen[value as usize] {
                        return false; // Duplicate in row
                    }
                    seen[value as usize] = true;
                }
            }
        }
        
        // Check all columns
        for col in 0..9 {
            let mut seen = [false; 10];
            for row in 0..9 {
                let value = self.grid[row][col];
                if value != 0 {
                    if seen[value as usize] {
                        return false; // Duplicate in column
                    }
                    seen[value as usize] = true;
                }
            }
        }
        
        // Check all 3x3 boxes
        for box_row in 0..3 {
            for box_col in 0..3 {
                let mut seen = [false; 10];
                for r in box_row * 3..(box_row + 1) * 3 {
                    for c in box_col * 3..(box_col + 1) * 3 {
                        let value = self.grid[r][c];
                        if value != 0 {
                            if seen[value as usize] {
                                return false; // Duplicate in box
                            }
                            seen[value as usize] = true;
                        }
                    }
                }
            }
        }
        
        true
    }

    /// Find the next empty cell (returns None if board is complete)
    pub fn find_empty_cell(&self) -> Option<(usize, usize)> {
        for row in 0..9 {
            for col in 0..9 {
                if self.grid[row][col] == 0 {
                    return Some((row, col));
                }
            }
        }
        None
    }
}

impl fmt::Display for SudokuBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "┌───────┬───────┬───────┐")?;
        for (i, row) in self.grid.iter().enumerate() {
            write!(f, "│ ")?;
            for (j, &cell) in row.iter().enumerate() {
                let display = if cell == 0 { " ".to_string() } else { cell.to_string() };
                write!(f, "{}", display)?;
                if j % 3 == 2 {
                    write!(f, " │ ")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f)?;
            if i % 3 == 2 && i < 8 {
                writeln!(f, "├───────┼───────┼───────┤")?;
            }
        }
        write!(f, "└───────┴───────┴───────┘")
    }
}

impl Default for SudokuBoard {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_creation() {
        let board = SudokuBoard::new();
        assert_eq!(board.get(0, 0), 0);
        assert_eq!(board.get(8, 8), 0);
    }

    #[test]
    fn test_from_comma_separated_string() {
        let input = "5,3,0,0,7,0,0,0,0,6,0,0,1,9,5,0,0,0,0,9,8,0,0,0,0,6,0,8,0,0,0,6,0,0,0,3,4,0,0,8,0,3,0,0,1,7,0,0,0,2,0,0,0,6,0,6,0,0,0,0,2,8,0,0,0,0,4,1,9,0,0,5,0,0,0,0,8,0,0,7,9";
        let board = SudokuBoard::from_string(input).unwrap();
        assert_eq!(board.get(0, 0), 5);
        assert_eq!(board.get(0, 1), 3);
        assert_eq!(board.get(0, 2), 0);
    }

    #[test]
    fn test_from_matrix_string() {
        let input = "530070000\n600195000\n098000060\n800060003\n400803001\n700020006\n060000280\n000419005\n000080079";
        let board = SudokuBoard::from_string(input).unwrap();
        assert_eq!(board.get(0, 0), 5);
        assert_eq!(board.get(0, 1), 3);
        assert_eq!(board.get(0, 2), 0);
        assert_eq!(board.get(1, 0), 6);
        assert_eq!(board.get(1, 3), 1);
    }

    #[test]
    fn test_from_matrix_with_dots() {
        let input = "5 3 . . 7 . . . .\n6 . . 1 9 5 . . .\n. 9 8 . . . . 6 .\n. . . . . . . . .\n. . . . . . . . .\n. . . . . . . . .\n. . . . . . . . .\n. . . . . . . . .\n. . . . . . . . .";
        let board = SudokuBoard::from_string(input).unwrap();
        assert_eq!(board.get(0, 0), 5);
        assert_eq!(board.get(0, 1), 3);
        assert_eq!(board.get(0, 2), 0); // dot becomes 0
        assert_eq!(board.get(1, 0), 6);
        assert_eq!(board.get(2, 1), 9);
    }

    #[test]
    fn test_to_string() {
        let input = "530070000\n600195000\n098000060\n800060003\n400803001\n700020006\n060000280\n000419005\n000080079";
        let board = SudokuBoard::from_string(input).unwrap();
        let output = board.to_string();
        assert_eq!(output, "53..7....6..195....98....6.8...6...34..8.3..17...2...6.6....28....419..5....8..79");
    }

    #[test]
    fn test_to_matrix_string() {
        let input = "530070000\n600195000\n098000060\n800060003\n400803001\n700020006\n060000280\n000419005\n000080079";
        let board = SudokuBoard::from_string(input).unwrap();
        let matrix_output = board.to_matrix_string();
        assert!(matrix_output.contains("53..7...."));
        assert!(matrix_output.contains("6..195..."));
    }

    #[test]
    fn test_invalid_input() {
        // Too few values
        let result = SudokuBoard::from_string("1,2,3");
        assert!(result.is_err());
        
        // Invalid value
        let invalid_input = std::iter::repeat("10").take(81).collect::<Vec<_>>().join(",");
        let result = SudokuBoard::from_string(&invalid_input);
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_move() {
        let board = SudokuBoard::new();
        assert!(board.is_valid_move(0, 0, 5));
        
        let mut board = board;
        board.set(0, 0, 5);
        assert!(!board.is_valid_move(0, 1, 5)); // Same row
        assert!(!board.is_valid_move(1, 0, 5)); // Same column
        assert!(!board.is_valid_move(1, 1, 5)); // Same 3x3 box
    }
}
