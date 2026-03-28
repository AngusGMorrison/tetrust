use std::{fmt, ops};

use BlockType::*;
use indoc::indoc;
use rand::Rng;
use rand_distr::{Distribution, Uniform};
use ratatui::{
    style::Stylize,
    text::{Line, Span, Text},
};

use crate::board::{BOARD_COLS, BUFFER_ZONE_ROWS};

/// Row-column coordinates for matrix access.
pub type Position = (usize, usize);

// TODO: Update this as new block types are added.
const N_BLOCK_TYPES: u8 = 7;

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

    pub fn colorizer(&self) -> fn(&'static str) -> Span<'static> {
        match self {
            I => Stylize::cyan,
            J => Stylize::blue,
            L => Stylize::light_red,
            O => Stylize::yellow,
            S => Stylize::green,
            T => Stylize::magenta,
            Z => Stylize::red,
        }
    }

    /// Returns a coloured grid cell for rendering.
    pub fn grid_cell(&self) -> Span<'static> {
        self.colorizer()("██")
    }

    pub fn schematic(&self) -> Text<'static> {
        let colorizer = self.colorizer();
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
            .map(|line| Line::from(colorizer(line)))
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

#[derive(Debug, Clone)]
pub struct BlockGenerator<R> {
    rng: R,
    sampler: Uniform<u8>,
}

impl<R> BlockGenerator<R> {
    pub fn new(rng: R) -> Self {
        let sampler = Uniform::new_inclusive(1, N_BLOCK_TYPES).unwrap_or_else(|_| {
            panic!("uniform sampler is always valid for 1..={}", N_BLOCK_TYPES)
        });
        Self { rng, sampler }
    }
}

impl<R: Rng> BlockGenerator<R> {
    pub fn block(&mut self) -> BlockType {
        match self.sampler.sample(&mut self.rng) {
            1 => I,
            2 => J,
            3 => L,
            4 => O,
            5 => S,
            6 => T,
            7 => Z,
            i => unreachable!(
                "Only {N_BLOCK_TYPES} block types are implemented, but sampler returned {i}",
            ),
        }
    }
}

/// A single rotation of a block situated in a local coordinate space. Conceptually, this is a 2D
/// matrix, but the matrix itself isn't required to implement the game.
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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

    /// Returns the schematic representation of the block according to the [BlockType].
    pub fn schematic(&self) -> Text<'static> {
        self.block_type.schematic()
    }
}
