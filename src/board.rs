use std::fmt;

use crate::block::{ActiveBlock, BlockType};

/// The play space. A 2D matrix where a square is Some with the occupying [BlockType] if occupied
/// and None otherwise.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Board([[Option<BlockType>; Self::COLUMNS]; Self::ROWS]);

impl Board {
    /// The number of columns on the board.
    pub const COLUMNS: usize = 10;

    /// The height of the invisible buffer zone used for spawning blocks.
    pub const BUFFER_ZONE_ROWS: usize = 2;

    /// The number of rows rendered to the player.
    pub const PLAYABLE_ROWS: usize = 20;

    /// The total number of rows on the board.
    pub const ROWS: usize = Self::BUFFER_ZONE_ROWS + Self::PLAYABLE_ROWS;

    /// Instantiates an empty board.
    pub fn new() -> Self {
        Self::default()
    }

    /// Instatiates a full board.
    #[cfg(test)]
    fn new_filled() -> Self {
        Self([[Some(BlockType::I); Self::COLUMNS]; Self::ROWS])
    }

    /// Clear continguous rows of occupied squares and consolidate the board, returning the number
    /// of lines cleared.
    pub fn clear_lines(&mut self) -> u8 {
        let mut cleared_row_count = 0;

        // First, work down the board to find the highest currently occupied row. This tells us
        // when to stop swapping cleared lines upwards.
        let mut highest_occupied_row = 0isize; // isize is simpler to compare in the loop condition below
        for (i, row) in self.0.iter().enumerate() {
            if row.iter().any(|v| v.is_some()) {
                highest_occupied_row = i as isize;
                break;
            }
        }

        // Next, work up the board looking for completed rows.
        let mut i = (Self::ROWS - 1) as isize; // isize avoids a wrapping sub when highest_occupied_row is 0
        while i >= highest_occupied_row {
            // Skip incomplete rows.
            if self.0[i as usize].iter().any(|v| v.is_none()) {
                i -= 1;
                continue;
            }

            // Clear completed rows.
            self.0[i as usize].fill(None);
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
            // Collisions with the left boundary are detectable by underflow of `pos.1`.
            .any(|pos| {
                pos.0 >= Self::ROWS || pos.1 >= Self::COLUMNS || self.0[pos.0][pos.1].is_some()
            })
    }

    /// Fills the board cells corresponding to the final position of the active block, fixing the
    /// the block to the board.
    pub fn fix_active_block(&mut self, active_block: &ActiveBlock) {
        active_block
            .board_positions()
            .for_each(|(r, c)| self.0[r][c] = Some(active_block.block_type()));
    }

    /// Returns true if the two-row buffer zone at the top of the board is occupied, which can be
    /// used to detect the game over state.
    pub fn buffer_zone_occupied(&self) -> bool {
        let occupied = self.0[1].iter().any(|v| v.is_some());

        #[cfg(debug_assertions)]
        if !occupied {
            debug_assert!(
                self.0[0].iter().all(|v| v.is_none()),
                "Lower row of buffer zone was empty, but upper row was populated",
            )
        }

        occupied
    }

    /// Returns an iterator over the board's rows.
    pub fn iter(&self) -> impl Iterator<Item = &[Option<BlockType>; Self::COLUMNS]> {
        self.0.iter()
    }
}

impl From<[[Option<BlockType>; Board::COLUMNS]; Board::ROWS]> for Board {
    fn from(value: [[Option<BlockType>; Board::COLUMNS]; Board::ROWS]) -> Self {
        Board(value)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "*{}*", "—".repeat(Board::COLUMNS))?;

        let print_row = |f: &mut fmt::Formatter<'_>, row: &[Option<BlockType>; 10]| {
            let row = row.map(|o| o.map_or(" ".into(), |bt| bt.to_string()));
            writeln!(
                f,
                "|{}{}{}{}{}{}{}{}{}{}|",
                row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7], row[8], row[9]
            )
        };

        self.0[..Board::BUFFER_ZONE_ROWS]
            .iter()
            .try_for_each(|row| print_row(f, row))?;

        writeln!(f, "|{}|", "—".repeat(Board::COLUMNS))?;

        self.0[Board::BUFFER_ZONE_ROWS..]
            .iter()
            .try_for_each(|row| print_row(f, row))?;

        writeln!(f, "*{}*", "—".repeat(Board::COLUMNS))
    }
}

#[cfg(test)]
mod board_tests {
    use super::*;
    mod clear_lines_tests {
        use super::*;

        #[test]
        fn when_board_is_empty_clears_no_lines() {
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
        fn when_board_is_full_clears_all_lines() {
            let mut board = Board::new_filled();
            let expected_lines_cleared = Board::ROWS as u8;
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
        fn when_one_complete_line_clears_one_line() {
            let mut board = Board::new();
            board.0[Board::ROWS - 1] = [Some(BlockType::I); Board::COLUMNS];

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
        fn when_multiple_complete_lines_clears_all_complete_lines() {
            let mut board = Board::new();
            board.0[Board::ROWS - 2] = [Some(BlockType::I); Board::COLUMNS];
            board.0[Board::ROWS - 1] = [Some(BlockType::I); Board::COLUMNS];

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
        fn when_complete_line_has_rows_above_it_consolidates_board() {
            let mut board = Board::new();
            board.0[Board::ROWS - 3] = [
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
            ];
            board.0[Board::ROWS - 2] = [
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
            ];
            board.0[Board::ROWS - 1] = [Some(BlockType::I); Board::COLUMNS];

            let expected_lines_cleared = 1;
            let mut expected_board = Board::new();
            expected_board.0[Board::ROWS - 2] = [
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
            ];
            expected_board.0[Board::ROWS - 1] = [
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
            ];

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
        fn when_multiple_complete_lines_have_rows_above_them_consolidates_board() {
            let mut board = Board::new();
            board.0[Board::ROWS - 4] = [
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
            ];
            board.0[Board::ROWS - 3] = [Some(BlockType::I); Board::COLUMNS];
            board.0[Board::ROWS - 2] = [
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
            ];
            board.0[Board::ROWS - 1] = [Some(BlockType::I); Board::COLUMNS];

            let expected_lines_cleared = 2;
            let mut expected_board = Board::new();
            expected_board.0[Board::ROWS - 2] = [
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
            ];
            expected_board.0[Board::ROWS - 1] = [
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
                Some(BlockType::I),
                None,
            ];

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

    mod collides_tests {
        use super::*;

        #[test]
        fn when_block_is_within_bounds_and_board_is_empty_returns_false() {
            let board = Board::new();
            let block = ActiveBlock::new(BlockType::I);
            assert!(!board.collides(&block));
        }

        #[test]
        fn when_block_row_exceeds_board_rows_returns_true() {
            let board = Board::new();
            let mut block = ActiveBlock::new(BlockType::I);
            for _ in 0..Board::ROWS - 1 {
                block.move_down();
            }
            assert!(board.collides(&block));
        }

        #[test]
        fn when_block_column_exceeds_board_columns_returns_true() {
            let board = Board::new();
            let mut block = ActiveBlock::new(BlockType::I);
            // I starts at left=3; move right 5 times to left=8 so local col 2 maps to board col 10.
            for _ in 0..5 {
                block.move_right();
            }
            assert!(board.collides(&block));
        }

        #[test]
        fn when_block_is_past_left_boundary_returns_true() {
            let board = Board::new();
            let mut block = ActiveBlock::new(BlockType::I);
            // I starts at left=3; move left 4 times to left=-1 so local col 0 overflows to usize::MAX.
            for _ in 0..4 {
                block.move_left();
            }
            assert!(board.collides(&block));
        }

        #[test]
        fn when_block_overlaps_occupied_cell_returns_true() {
            let mut board = Board::new();
            // I at its initial position occupies board cell (1, 3).
            board.0[1][3] = Some(BlockType::I);
            let block = ActiveBlock::new(BlockType::I);
            assert!(board.collides(&block));
        }
    }

    mod fix_active_block_tests {
        use super::*;

        #[test]
        fn sets_block_positions_to_block_type() {
            let mut board = Board::new();
            let block = ActiveBlock::new(BlockType::I);

            board.fix_active_block(&block);

            // I at its initial position occupies (1, 3..=6).
            let mut expected = Board::new();
            expected.0[1][3] = Some(BlockType::I);
            expected.0[1][4] = Some(BlockType::I);
            expected.0[1][5] = Some(BlockType::I);
            expected.0[1][6] = Some(BlockType::I);
            assert_eq!(board, expected);
        }
    }

    mod buffer_zone_occupied_tests {
        use super::*;

        #[test]
        fn when_buffer_zone_row_1_is_occupied_returns_true() {
            let mut board = Board::new();
            board.0[1][0] = Some(BlockType::I);
            assert!(board.buffer_zone_occupied());
        }

        #[test]
        fn when_buffer_zone_is_empty_returns_false() {
            let board = Board::new();
            assert!(!board.buffer_zone_occupied());
        }
    }
}
