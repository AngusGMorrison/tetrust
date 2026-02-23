use std::fmt;

use BlockType::*;
use rand::Rng;
use rand_distr::{Distribution, Uniform};

use crate::rotation::{I_ROTATIONS, J_ROTATIONS, O_ROTATIONS, Rotation};

/// Row-column coordinates for matrix access.
pub type Position = (usize, usize);

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
    fn rotations(&self) -> &'static [Rotation] {
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
        self.rotation().width()
    }

    pub fn height(&self) -> usize {
        self.rotation().height()
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
        let sampler = Uniform::new_inclusive(1, N_BLOCK_TYPES).unwrap_or_else(|_| {
            panic!("uniform sampler is always valid for 1..={}", N_BLOCK_TYPES)
        });
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
