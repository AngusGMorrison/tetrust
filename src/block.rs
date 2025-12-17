use std::fmt;

use BlockType::*;
use rand::Rng;
use rand_distr::{Distribution, Uniform};

/// A single orientation of a [Block], expressed as a square matrix where zeroes are empty space
/// and ones are part of the Block.
#[derive(Clone, Copy)]
pub struct Orientation(&'static [&'static [u8]]);

impl fmt::Debug for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0 {
            for (i, val) in row.iter().enumerate() {
                if i == row.len() - 1 {
                    writeln!(f, "{}", val)?
                } else {
                    write!(f, "{} ", val)?
                }
            }
        }

        fmt::Result::Ok(())
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

/// Row-column coordinates for matrix access.
type Position = (usize, usize);

/// The coordinates describing a [Block]'s bounding box relative to the upper-left corner of its
/// orientation matrix.
#[derive(Debug, Clone, Copy)]
pub struct BoundingBox {
    // The upper-leftmost rc-coordinate of the Block.
    min: Position,
    // The lower-rightmost rc-coordinate of the Block.
    max: Position,
}

impl BoundingBox {
    pub fn min(&self) -> Position {
        self.min
    }

    pub fn max(&self) -> Position {
        self.max
    }
}

/// Rotating a [Block] results in a new [Orientation] and [BoundingBox].
#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    orientation: Orientation,
    bounding_box: BoundingBox,
}

impl Rotation {
    pub fn orientation(&self) -> Orientation {
        self.orientation
    }

    pub fn bounding_box(&self) -> BoundingBox {
        self.bounding_box
    }
}

/// The complete set of rotations for a given [BlockType].
type Rotations = [Rotation; 4];

#[rustfmt::skip]
const I_ROTATIONS: &Rotations = &[
    Rotation {
        orientation: Orientation(&[
            &[0, 0, 0, 0],
            &[1, 1, 1, 1],
            &[0, 0, 0, 0],
            &[0, 0, 0, 0],
        ]),
        bounding_box: BoundingBox {
            min: (1, 0),
            max: (1, 3),
        },
    },
    Rotation {
        orientation: Orientation(&[
            &[0, 0, 1, 0],
            &[0, 0, 1, 0],
            &[0, 0, 1, 0],
            &[0, 0, 1, 0],
        ]),
        bounding_box: BoundingBox {
            min: (0, 2),
            max: (3, 2),
        },
    },
    Rotation {
        orientation: Orientation(&[
            &[0, 0, 0, 0],
            &[0, 0, 0, 0],
            &[1, 1, 1, 1],
            &[0, 0, 0, 0],
        ]),
        bounding_box: BoundingBox {
            min: (2, 0),
            max: (2, 3),
        },
    },
    Rotation {
        orientation: Orientation(&[
            &[0, 1, 0, 0],
            &[0, 1, 0, 0],
            &[0, 1, 0, 0],
            &[0, 1, 0, 0],
        ]),
        bounding_box: BoundingBox {
            min: (0, 1),
            max: (3, 1),
        },
    },
];

#[rustfmt::skip]
const J_ROTATIONS: &Rotations = &[
    Rotation {
        orientation: Orientation(&[
            &[1, 0, 0],
            &[1, 1, 1],
            &[0, 0, 0],
        ]),
        bounding_box: BoundingBox{
            min: (0, 0),
            max: (1, 2),
        },
    },
    Rotation {
        orientation: Orientation(&[
            &[0, 1, 1],
            &[0, 1, 0],
            &[0, 1, 0],
        ]),
        bounding_box: BoundingBox{
            min: (0, 1),
            max: (2, 2),
        },
    },
    Rotation {
        orientation: Orientation(&[
            &[0, 0, 0],
            &[1, 1, 1],
            &[0, 0, 1],
        ]),
        bounding_box: BoundingBox{
            min: (1, 0),
            max:(2, 2),
        },
    },
    Rotation {
        orientation: Orientation(&[
            &[0, 1, 0],
            &[0, 1, 0],
            &[1, 1, 0],
        ]),
        bounding_box: BoundingBox{
            min: (0, 0),
            max: (2, 1),
        },
    }
];

// Repeating the single orientation for the unique O block means we don't need any special case code
// to handle it.
#[rustfmt::skip]
const O_ROTATIONS: &Rotations = &[
    Rotation {
        orientation: Orientation(&[
            &[1, 1],
            &[1, 1],
        ]),
        bounding_box: BoundingBox {
            min: (0, 0),
            max:(1, 1),
        },
    },
    Rotation {
        orientation: Orientation(&[
            &[1, 1],
            &[1, 1],
        ]),
        bounding_box: BoundingBox {
            min: (0, 0),
            max:(1, 1),
        },
    },
    Rotation {
        orientation: Orientation(&[
            &[1, 1],
            &[1, 1],
        ]),
        bounding_box: BoundingBox {
            min: (0, 0),
            max:(1, 1),
        },
    },
    Rotation {
        orientation: Orientation(&[
            &[1, 1],
            &[1, 1],
        ]),
        bounding_box: BoundingBox {
            min: (0, 0),
            max:(1, 1),
        },
    },
];

// TODO: Update this as new block types are added.
const N_BLOCK_TYPES: u8 = 3;

/// The varieties of block that may be seen in a game.
#[derive(Copy, Clone, Debug)]
pub enum BlockType {
    I,
    J,
    O,
}

impl BlockType {
    /// Returns all the orientations a block may be rotated into.
    fn rotations(&self) -> &'static Rotations {
        match self {
            I => I_ROTATIONS,
            J => J_ROTATIONS,
            O => O_ROTATIONS,
        }
    }
}

impl fmt::Display for BlockType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            I => writeln!(f, "I"),
            J => writeln!(f, "J"),
            O => writeln!(f, "O"),
        }
    }
}

/// The state of a block in play.
#[derive(Copy, Clone, Debug)]
pub struct Block {
    block_type: BlockType,
    rotation_counter: usize,
}

impl Block {
    pub fn new(block_type: BlockType) -> Self {
        Self {
            block_type,
            rotation_counter: 0,
        }
    }

    pub fn block_type(&self) -> BlockType {
        self.block_type
    }

    pub fn width(&self) -> usize {
        let bounding_box = self.rotation().bounding_box;
        bounding_box.max.1 - bounding_box.min.1
    }

    pub fn height(&self) -> usize {
        let bounding_box = self.rotation().bounding_box;
        bounding_box.max.0 - bounding_box.max.1
    }

    /// Returns the [Block]'s current [Rotation].
    pub fn rotation(&self) -> &'static Rotation {
        &self.block_type.rotations()[self.rotation_counter]
    }

    /// Rotates the [Block] clockwise, returning its new [Rotation].
    pub fn rotate_clockwise(&mut self) -> &'static Rotation {
        self.rotation_counter = (self.rotation_counter + 1) % 4;
        self.rotation()
    }

    /// Rotates the [Block] counter-clockwise, returning its new [Rotation].
    pub fn rotate_counter_clockwise(&mut self) -> &'static Rotation {
        // usize::MAX gives the correct index % 4 even when underflow occurs.
        self.rotation_counter = (self.rotation_counter - 1) % 4;
        self.rotation()
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.rotation().orientation)
    }
}

impl From<BlockType> for Block {
    fn from(block_type: BlockType) -> Self {
        Self::new(block_type)
    }
}

#[derive(Debug, Clone)]
pub struct BlockGenerator<R: Rng> {
    rng: R,
    sampler: Uniform<u8>,
}

impl<R: Rng> BlockGenerator<R> {
    pub fn new(rng: R) -> Self {
        let sampler = Uniform::new_inclusive(1, N_BLOCK_TYPES)
            .expect("uniform sampler is always valid for 1..=7");
        Self { rng, sampler }
    }

    pub fn block(&mut self) -> Block {
        match self.sampler.sample(&mut self.rng) {
            1 => I.into(),
            2 => J.into(),
            3 => O.into(),
            i => unreachable!(
                "Only {} block types are implemented, but sampler returned {}",
                N_BLOCK_TYPES, i
            ),
        }
    }
}
