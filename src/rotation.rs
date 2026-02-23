use crate::block::Position;

/// A single rotation of a block situated in a local coordinate space. Conceptually, this is a 2D
/// matrix, but the matrix itself isn't required to implement the game.
pub struct Rotation {
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
    pub fn vertical_offset(&self) -> usize {
        self.vertical_offset
    }

    pub fn horizontal_offset(&self) -> usize {
        self.horizontal_offset
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns an iterator over the positions occupied by the block in its local coordinate space.
    pub fn positions(&self) -> impl Iterator<Item = &Position> {
        self.positions.iter()
    }
}

type Rotations = [Rotation; 4];

#[rustfmt::skip]
pub const I_ROTATIONS: &Rotations = &[
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
];

#[rustfmt::skip]
pub const J_ROTATIONS: &Rotations = &[
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
];

#[rustfmt::skip]
pub const O_ROTATIONS: &Rotations = &[
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
];
