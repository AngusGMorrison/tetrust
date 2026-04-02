use std::{fmt, ops};

use BlockType::*;
use indoc::indoc;
use ratatui::{
    style::Stylize,
    text::{Line, Span, Text},
};

use crate::board::{BOARD_COLS, BUFFER_ZONE_ROWS};

/// Row-column coordinates for matrix access.
pub type Position = (usize, usize);

/// The varieties of block that may be seen in a game.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BlockType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl BlockType {
    /// The number of block types in the game.
    pub const COUNT: u8 = 7;

    /// Returns all possible rotations of the block type.
    fn rotations(&self) -> &'static Rotations {
        match self {
            I => I_ROTATIONS,
            J => J_ROTATIONS,
            L => L_ROTATIONS,
            O => O_ROTATIONS,
            S => S_ROTATIONS,
            T => T_ROTATIONS,
            Z => Z_ROTATIONS,
        }
    }

    fn colorize(&self, s: &'static str) -> Span<'static> {
        match self {
            I => s.cyan(),
            J => s.blue(),
            L => s.light_red(),
            O => s.yellow(),
            S => s.green(),
            T => s.magenta(),
            Z => s.red(),
        }
    }

    /// Returns a coloured grid cell for rendering.
    pub fn grid_cell(&self) -> Span<'static> {
        self.colorize("██")
    }

    /// Returns the schematic representation of the block type for rendering.
    pub fn schematic(&self) -> Text<'static> {
        let raw: &'static str = match self {
            I => indoc! {"
                \n████████
            "},
            J => indoc! {"
                ██
                ██████
            "},
            L => indoc! {"
                    ██
                ██████
            "},
            O => indoc! {"
                ████
                ████
            "},
            S => indoc! {"
                  ████
                ████
            "},
            T => indoc! {"
                  ██
                ██████
            "},
            Z => indoc! {"
                ████
                  ████
            "},
        };
        raw.lines()
            .map(|line| Line::from(self.colorize(line)))
            .collect::<Vec<_>>()
            .into()
    }
}

impl ops::Index<RotationIndex> for BlockType {
    type Output = Rotation;

    fn index(&self, index: RotationIndex) -> &Self::Output {
        &self.rotations()[index]
    }
}

impl fmt::Display for BlockType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.schematic())
    }
}

/// A single rotation of a block situated in a local coordinate space. Conceptually, this is a 2D
/// matrix, but the matrix itself isn't necessary to implement the game.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Rotation {
    /// The positive vertical offset of the top of the block from the local coordinate space's
    /// origin.
    vertical_offset: usize,

    /// The positive horizontal offset of the left of the block from the local coordinate space's
    /// origin.
    horizontal_offset: usize,

    /// The width of the block.
    width: usize,

    /// The height of the block.
    height: usize,

    /// The positions occupied by the block in its local coordinate space.
    positions: [Position; 4],
}

impl Rotation {
    fn vertical_offset(&self) -> usize {
        self.vertical_offset
    }

    fn horizontal_offset(&self) -> usize {
        self.horizontal_offset
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }

    /// Returns an iterator over the positions occupied by the block in its local coordinate space.
    fn positions(&self) -> impl Iterator<Item = &Position> {
        self.positions.iter()
    }
}

/// A complete set of four rotations for a [BlockType].
#[derive(Debug, Clone, PartialEq, Eq)]
struct Rotations([Rotation; 4]);

/// Type-safe wrapping type for indexing [Rotations], constrained to the range 0..4.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
struct RotationIndex(usize);

impl RotationIndex {
    fn new() -> Self {
        Self::default()
    }

    fn inc(&mut self) {
        self.0 = self.0.wrapping_add(1) % 4
    }

    fn dec(&mut self) {
        self.0 = self.0.wrapping_sub(1) % 4
    }
}

impl ops::Index<RotationIndex> for Rotations {
    type Output = Rotation;

    fn index(&self, index: RotationIndex) -> &Self::Output {
        &self.0[index.0]
    }
}

// I rotations. All states use a 4×4 bounding box.
//
// Rot 0:  . . . .    Rot 1:  . . X .    Rot 2:  . . . .    Rot 3:  . X . .
//         X X X X            . . X .            . . . .            . X . .
//         . . . .            . . X .            X X X X            . X . .
//         . . . .            . . X .            . . . .            . X . .
#[rustfmt::skip]
const I_ROTATIONS: &Rotations = &Rotations([
    Rotation {
        vertical_offset: 1,
        horizontal_offset: 0,
        width: 4,
        height: 1,
        positions: [(1, 0), (1, 1), (1, 2), (1, 3)],
    },
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 2,
        width: 1,
        height: 4,
        positions: [(0, 2), (1, 2), (2, 2), (3, 2)],
    },
    Rotation {
        vertical_offset: 2,
        horizontal_offset: 0,
        width: 4,
        height: 1,
        positions: [(2, 0), (2, 1), (2, 2), (2, 3)],
    },
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 1,
        width: 1,
        height: 4,
        positions: [(0, 1), (1, 1), (2, 1), (3, 1)],
    },
]);

// J rotations. All states use a 3×3 bounding box.
//
// Rot 0:  X . .    Rot 1:  . X X    Rot 2:  . . .    Rot 3:  . X .
//         X X X            . X .            X X X            . X .
//         . . .            . X .            . . X            X X .
#[rustfmt::skip]
const J_ROTATIONS: &Rotations = &Rotations([
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 0,
        width: 3,
        height: 2,
        positions: [(0, 0), (1, 0), (1, 1), (1, 2)],
    },
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 1,
        width: 2,
        height: 3,
        positions: [(0, 1), (0, 2), (1, 1), (2, 1)],
    },
    Rotation {
        vertical_offset: 1,
        horizontal_offset: 0,
        width: 3,
        height: 2,
        positions: [(1, 0), (1, 1), (1, 2), (2, 2)],
    },
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 0,
        width: 2,
        height: 3,
        positions: [(0, 1), (1, 1), (2, 0), (2, 1)],
    },
]);

// O rotations. All states use a 2×2 bounding box.
//
// All rotations:  X X
//                 X X
#[rustfmt::skip]
const O_ROTATIONS: &Rotations = &Rotations([
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 0,
        width: 2,
        height: 2,
        positions: [(0, 0), (0, 1), (1, 0), (1, 1)],
    },
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 0,
        width: 2,
        height: 2,
        positions: [(0, 0), (0, 1), (1, 0), (1, 1)],
    },
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 0,
        width: 2,
        height: 2,
        positions: [(0, 0), (0, 1), (1, 0), (1, 1)],
    },
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 0,
        width: 2,
        height: 2,
        positions: [(0, 0), (0, 1), (1, 0), (1, 1)],
    },
]);

// L rotations (mirror of J). All states use a 3×3 bounding box.
//
// Rot 0:  . . X    Rot 1:  . X .    Rot 2:  . . .    Rot 3:  X X .
//         X X X            . X .            X X X            . X .
//         . . .            . X X            X . .            . X .
#[rustfmt::skip]
const L_ROTATIONS: &Rotations = &Rotations([
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 0,
        width: 3,
        height: 2,
        positions: [(0, 2), (1, 0), (1, 1), (1, 2)],
    },
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 1,
        width: 2,
        height: 3,
        positions: [(0, 1), (1, 1), (2, 1), (2, 2)],
    },
    Rotation {
        vertical_offset: 1,
        horizontal_offset: 0,
        width: 3,
        height: 2,
        positions: [(1, 0), (1, 1), (1, 2), (2, 0)],
    },
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 0,
        width: 2,
        height: 3,
        positions: [(0, 0), (0, 1), (1, 1), (2, 1)],
    },
]);

// S rotations. All states use a 3×3 bounding box.
//
// Rot 0:  . X X    Rot 1:  . X .    Rot 2:  . . .    Rot 3:  X . .
//         X X .            . X X            . X X            X X .
//         . . .            . . X            X X .            . X .
#[rustfmt::skip]
const S_ROTATIONS: &Rotations = &Rotations([
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 0,
        width: 3,
        height: 2,
        positions: [(0, 1), (0, 2), (1, 0), (1, 1)],
    },
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 1,
        width: 2,
        height: 3,
        positions: [(0, 1), (1, 1), (1, 2), (2, 2)],
    },
    Rotation {
        vertical_offset: 1,
        horizontal_offset: 0,
        width: 3,
        height: 2,
        positions: [(1, 1), (1, 2), (2, 0), (2, 1)],
    },
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 0,
        width: 2,
        height: 3,
        positions: [(0, 0), (1, 0), (1, 1), (2, 1)],
    },
]);

// T rotations. All states use a 3×3 bounding box.
//
// Rot 0:  . X .    Rot 1:  . X .    Rot 2:  . . .    Rot 3:  . X .
//         X X X            . X X            X X X            X X .
//         . . .            . X .            . X .            . X .
#[rustfmt::skip]
const T_ROTATIONS: &Rotations = &Rotations([
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 0,
        width: 3,
        height: 2,
        positions: [(0, 1), (1, 0), (1, 1), (1, 2)],
    },
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 1,
        width: 2,
        height: 3,
        positions: [(0, 1), (1, 1), (1, 2), (2, 1)],
    },
    Rotation {
        vertical_offset: 1,
        horizontal_offset: 0,
        width: 3,
        height: 2,
        positions: [(1, 0), (1, 1), (1, 2), (2, 1)],
    },
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 0,
        width: 2,
        height: 3,
        positions: [(0, 1), (1, 0), (1, 1), (2, 1)],
    },
]);

// Z rotations. All states use a 3×3 bounding box.
//
// Rot 0:  X X .    Rot 1:  . . X    Rot 2:  . . .    Rot 3:  . X .
//         . X X            . X X            X X .            X X .
//         . . .            . X .            . X X            X . .
#[rustfmt::skip]
const Z_ROTATIONS: &Rotations = &Rotations([
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 0,
        width: 3,
        height: 2,
        positions: [(0, 0), (0, 1), (1, 1), (1, 2)],
    },
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 1,
        width: 2,
        height: 3,
        positions: [(0, 2), (1, 1), (1, 2), (2, 1)],
    },
    Rotation {
        vertical_offset: 1,
        horizontal_offset: 0,
        width: 3,
        height: 2,
        positions: [(1, 0), (1, 1), (2, 1), (2, 2)],
    },
    Rotation {
        vertical_offset: 0,
        horizontal_offset: 0,
        width: 2,
        height: 3,
        positions: [(0, 1), (1, 0), (1, 1), (2, 0)],
    },
]);

/// The block currently under the player's control.
#[derive(Debug, Clone)]
pub struct ActiveBlock {
    // The row-column coordinates of the top-left corner of the block's virtual bounding box on the
    // board.
    //
    // The column coordinate of the box is allowed to be negative since it may leave the left bounds
    // of the board while all of the block's cells remain inbounds (either vertical alignment of an
    // I block, for example).
    top_left: (usize, isize),
    block_type: BlockType,
    rotation_idx: RotationIndex,
}

impl ActiveBlock {
    pub fn new(block_type: BlockType) -> Self {
        let rotation_idx = RotationIndex::new();
        let rotation = &block_type[rotation_idx];

        let height = rotation.height();
        debug_assert!(
            height <= 2,
            "Block starting height was {}. Any height greater than 2 places it out of bounds.",
            height
        );

        let width = rotation.width();
        debug_assert!(
            width <= BOARD_COLS,
            "Block width {} exceeds board width {}",
            width,
            BOARD_COLS,
        );

        // Place the bounding box so that the block lands at the bottom of the buffer zone.
        let r = BUFFER_ZONE_ROWS - rotation.vertical_offset() - height;

        // The initial column coordinate places the block approximately in the center of the board.
        //
        // For example, on a standard 10-column board, the I block's leftmost cell falls in row[3],
        // while the O and S blocks' fall in row[4]. This gives a one-cell rightwards bias to
        // three-cell-wide blocks.
        let c = BOARD_COLS / 2 - rotation.horizontal_offset() - width / 2;

        Self {
            top_left: (r, c as isize),
            block_type,
            rotation_idx,
        }
    }

    pub(crate) fn block_type(&self) -> BlockType {
        self.block_type
    }

    // Returns the board-space coordinates of the top-left cell of the ActiveBlock.
    fn top_left(&self) -> (usize, isize) {
        self.top_left
    }

    fn rotation(&self) -> &Rotation {
        &self.block_type[self.rotation_idx]
    }

    /// Returns an iterator of the positions of the block's cells in board space in order of
    /// increasing row then column.
    pub fn board_positions(&self) -> impl Iterator<Item = Position> {
        let (top, left) = self.top_left();
        self.rotation().positions().map(move |(block_r, block_c)| {
            let r = top + block_r;

            // Unsigned addition will result in impossibly large values of c when a cell is outside
            // the left bounds of the board. This is used to detect collisions.
            let c = (left as usize).wrapping_add(*block_c);
            (r, c)
        })
    }

    pub fn move_down(&mut self) {
        self.top_left.0 = self.top_left.0.saturating_add(1)
    }

    pub fn move_up(&mut self) {
        self.top_left.0 = self.top_left.0.saturating_sub(1)
    }

    pub fn move_left(&mut self) {
        self.top_left.1 = self.top_left.1.saturating_sub(1)
    }

    pub fn move_right(&mut self) {
        self.top_left.1 = self.top_left.1.saturating_add(1)
    }

    pub fn rotate_clockwise(&mut self) {
        self.rotation_idx.inc();
    }

    pub fn rotate_counter_clockwise(&mut self) {
        self.rotation_idx.dec();
    }

    /// Returns a grid cell coloured according to the [BlockType].
    pub fn grid_cell(&self) -> Span<'static> {
        self.block_type.grid_cell()
    }
}

#[cfg(test)]
mod block_type_tests {
    use super::*;

    mod index_tests {
        use super::*;

        #[test]
        fn when_rotation_index_is_0_returns_rotation_0() {
            let idx = RotationIndex::new();
            assert_eq!(I[idx], I_ROTATIONS.0[0]);
        }

        #[test]
        fn when_rotation_index_is_1_returns_rotation_1() {
            let mut idx = RotationIndex::new();
            idx.inc();
            assert_eq!(I[idx], I_ROTATIONS.0[1]);
        }

        #[test]
        fn when_rotation_index_is_2_returns_rotation_2() {
            let mut idx = RotationIndex::new();
            idx.inc();
            idx.inc();
            assert_eq!(I[idx], I_ROTATIONS.0[2]);
        }

        #[test]
        fn when_rotation_index_is_3_returns_rotation_3() {
            let mut idx = RotationIndex::new();
            idx.inc();
            idx.inc();
            idx.inc();
            assert_eq!(I[idx], I_ROTATIONS.0[3]);
        }
    }

    mod rotations_tests {
        use super::*;

        #[test]
        fn i_returns_i_rotations() {
            assert_eq!(I.rotations(), I_ROTATIONS);
        }

        #[test]
        fn j_returns_j_rotations() {
            assert_eq!(J.rotations(), J_ROTATIONS);
        }

        #[test]
        fn l_returns_l_rotations() {
            assert_eq!(L.rotations(), L_ROTATIONS);
        }

        #[test]
        fn o_returns_o_rotations() {
            assert_eq!(O.rotations(), O_ROTATIONS);
        }

        #[test]
        fn s_returns_s_rotations() {
            assert_eq!(S.rotations(), S_ROTATIONS);
        }

        #[test]
        fn t_returns_t_rotations() {
            assert_eq!(T.rotations(), T_ROTATIONS);
        }

        #[test]
        fn z_returns_z_rotations() {
            assert_eq!(Z.rotations(), Z_ROTATIONS);
        }
    }
}

#[cfg(test)]
mod rotation_index_tests {
    use super::*;

    mod inc_tests {
        use super::*;

        #[test]
        fn when_index_is_less_than_3_increments() {
            let mut idx = RotationIndex::new();
            idx.inc();
            assert_eq!(idx, RotationIndex(1));
        }

        #[test]
        fn when_index_is_3_wraps_to_0() {
            let mut idx = RotationIndex(3);
            idx.inc();
            assert_eq!(idx, RotationIndex(0));
        }
    }

    mod dec_tests {
        use super::*;

        #[test]
        fn when_index_is_greater_than_0_decrements() {
            let mut idx = RotationIndex(3);
            idx.dec();
            assert_eq!(idx, RotationIndex(2));
        }

        #[test]
        fn when_index_is_0_wraps_to_3() {
            let mut idx = RotationIndex::new();
            idx.dec();
            assert_eq!(idx, RotationIndex(3));
        }
    }
}
