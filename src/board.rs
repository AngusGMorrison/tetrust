use std::fmt;

use crate::block::ActiveBlock;

/// The number of rows on the board. The first two rows are a buffer for spawning blocks and aren't
/// rendered to the user.
pub const BOARD_ROWS: usize = 22;

/// The number of columns on the board.
pub const BOARD_COLS: usize = 10;

/// The play space. A 2D matrix where a square is one if occupied and zero otherwise.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Board([[u8; BOARD_COLS]; BOARD_ROWS]);

impl Board {
    /// Instantiates an empty board.
    pub fn new() -> Self {
        Self::default()
    }

    /// Instatiates a full board.
    #[cfg(test)]
    fn new_filled() -> Self {
        Self([[1; BOARD_COLS]; BOARD_ROWS])
    }

    /// Clear continguous rows of occupied squares and consolidate the board, returning the number
    /// of lines cleared.
    pub fn clear_lines(&mut self) -> u8 {
        let mut cleared_row_count = 0;

        // First, work down the board to find the highest currently occupied row. This tells us
        // when to stop swapping cleared lines upwards.
        let mut highest_occupied_row = 0isize; // isize is simpler to compare in the loop condition below
        for (i, row) in self.0.iter().enumerate() {
            if row.contains(&1) {
                highest_occupied_row = i as isize;
                break;
            }
        }

        // Next, work up the board looking for completed rows.
        let mut i = (BOARD_ROWS - 1) as isize; // isize avoids a wrapping sub when highest_occupied_row is 0
        while i >= highest_occupied_row {
            // Skip incomplete rows.
            if self.0[i as usize].contains(&0) {
                i -= 1;
                continue;
            }

            // Clear completed rows.
            self.0[i as usize].fill(0);
            cleared_row_count += 1;

            // Consolidate the board by bubbling cleared rows upwards.
            let rows_to_swap = (highest_occupied_row + 1) as usize..=i as usize;
            for j in rows_to_swap.rev() {
                self.0.swap(j, j - 1)
            }
            highest_occupied_row += 1;
        }

        cleared_row_count
    }

    /// Returns true if the active block overlaps a non-empty cell of the board.
    pub fn collides(&self, active_block: &ActiveBlock) -> bool {
        active_block
            .board_positions()
            .any(|pos| pos.0 >= BOARD_ROWS || pos.1 >= BOARD_COLS || self.0[pos.0][pos.1] == 1)
    }

    /// Fills the board cells corresponding to the final position of the active block, fixing the
    /// the block to the board.
    pub fn fix_active_block(&mut self, active_block: &ActiveBlock) {
        active_block
            .board_positions()
            .for_each(|(r, c)| self.0[r][c] = 1);
    }

    /// Returns true if the two-row buffer zone at the top of the board is occupied, which can be
    /// used to detect the game over state.
    pub fn buffer_zone_occupied(&self) -> bool {
        let occupied = self.0[1].contains(&1);

        #[cfg(debug_assertions)]
        if !occupied {
            debug_assert!(
                self.0[0].iter().all(|&v| v == 0),
                "Lower row of buffer zone was empty, but upper row was populated",
            )
        }

        occupied
    }

    /// Returns an iterator over the board's rows.
    pub fn iter(&self) -> impl Iterator<Item = &[u8; BOARD_COLS]> {
        self.0.iter()
    }
}

impl From<[[u8; BOARD_COLS]; BOARD_ROWS]> for Board {
    fn from(value: [[u8; BOARD_COLS]; BOARD_ROWS]) -> Self {
        Board(value)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "*{}*", "—".repeat(BOARD_COLS))?;
        let row_printer = |f: &mut fmt::Formatter<'_>, row: &[u8; 10]| {
            writeln!(
                f,
                "|{}{}{}{}{}{}{}{}{}{}|",
                row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7], row[8], row[9]
            )
        };

        let iter = self.0.iter();
        iter.clone()
            .take(2)
            .try_for_each(|row| row_printer(f, row))?;
        writeln!(f, "|{}|", "—".repeat(BOARD_COLS))?;
        iter.skip(2).try_for_each(|row| row_printer(f, row))?;
        writeln!(f, "*{}*", "—".repeat(BOARD_COLS))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod clear_lines_tests {
        use super::*;

        #[test]
        fn empty_board() {
            let mut board = Board::new();
            let expected_lines_cleared = 0;
            let expected_board = Board::new();

            let lines_cleared = board.clear_lines();

            assert_eq!(
                expected_lines_cleared, lines_cleared,
                "Expected {} lines cleared, but got {}",
                expected_lines_cleared, lines_cleared
            );

            assert_eq!(
                expected_board, board,
                "Cleared board did not match expected board:\nExpected:\n{}\nActual:\n{}",
                expected_board, board
            )
        }

        #[test]
        fn full_board() {
            let mut board = Board::new_filled();
            let expected_lines_cleared = BOARD_ROWS as u8;
            let expected_board = Board::new();

            let lines_cleared = board.clear_lines();

            assert_eq!(
                expected_lines_cleared, lines_cleared,
                "Expected {} lines cleared, but got {}",
                expected_lines_cleared, lines_cleared
            );

            assert_eq!(
                expected_board, board,
                "Cleared board did not match expected board:\nExpected:\n{}\nActual:\n{}",
                expected_board, board
            )
        }

        #[test]
        fn single_line_no_consolidation() {
            let mut board = Board::new();
            board.0[BOARD_ROWS - 1] = [1; BOARD_COLS];

            let expected_lines_cleared = 1;
            let expected_board = Board::new();

            let lines_cleared = board.clear_lines();

            assert_eq!(
                expected_lines_cleared, lines_cleared,
                "Expected {} lines cleared, but got {}",
                expected_lines_cleared, lines_cleared
            );

            assert_eq!(
                expected_board, board,
                "Cleared board did not match expected board:\nExpected:\n{}\nActual:\n{}",
                expected_board, board
            )
        }

        #[test]
        fn multiple_lines_no_consolidation() {
            let mut board = Board::new();
            board.0[BOARD_ROWS - 2] = [1; BOARD_COLS];
            board.0[BOARD_ROWS - 1] = [1; BOARD_COLS];

            let expected_lines_cleared = 2;
            let expected_board = Board::new();

            let lines_cleared = board.clear_lines();

            assert_eq!(
                expected_lines_cleared, lines_cleared,
                "Expected {} lines cleared, but got {}",
                expected_lines_cleared, lines_cleared
            );

            assert_eq!(
                expected_board, board,
                "Cleared board did not match expected board:\nExpected:\n{}\nActual:\n{}",
                expected_board, board
            )
        }

        #[test]
        fn single_line_with_consolidation() {
            let mut board = Board::new();
            board.0[BOARD_ROWS - 3] = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
            board.0[BOARD_ROWS - 2] = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
            board.0[BOARD_ROWS - 1] = [1; BOARD_COLS];

            let expected_lines_cleared = 1;
            let mut expected_board = Board::new();
            expected_board.0[BOARD_ROWS - 2] = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
            expected_board.0[BOARD_ROWS - 1] = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0];

            let lines_cleared = board.clear_lines();

            assert_eq!(
                expected_lines_cleared, lines_cleared,
                "Expected {} lines cleared, but got {}",
                expected_lines_cleared, lines_cleared
            );

            assert_eq!(
                expected_board, board,
                "Cleared board did not match expected board:\nExpected:\n{}\nActual:\n{}",
                expected_board, board
            )
        }

        #[test]
        fn multiple_lines_with_consolidation() {
            let mut board = Board::new();
            board.0[BOARD_ROWS - 4] = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
            board.0[BOARD_ROWS - 3] = [1; BOARD_COLS];
            board.0[BOARD_ROWS - 2] = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0];
            board.0[BOARD_ROWS - 1] = [1; BOARD_COLS];

            let expected_lines_cleared = 2;
            let mut expected_board = Board::new();
            expected_board.0[BOARD_ROWS - 2] = [0, 1, 0, 1, 0, 1, 0, 1, 0, 1];
            expected_board.0[BOARD_ROWS - 1] = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0];

            let lines_cleared = board.clear_lines();

            assert_eq!(
                expected_lines_cleared, lines_cleared,
                "Expected {} lines cleared, but got {}",
                expected_lines_cleared, lines_cleared
            );

            assert_eq!(
                expected_board, board,
                "Cleared board did not match expected board:\nExpected:\n{}\nActual:\n{}",
                expected_board, board
            )
        }
    }
}
