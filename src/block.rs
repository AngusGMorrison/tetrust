use std::{fmt, ops};

use BlockType::*;
use rand::Rng;
use rand_distr::{Distribution, Uniform};

use crate::rotation::{I_ROTATIONS, J_ROTATIONS, O_ROTATIONS, Rotation, RotationIndex, Rotations};

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
    /// Returns all possible rotations of the block type.
    pub fn rotations(&self) -> &'static Rotations {
        match self {
            I => I_ROTATIONS,
            J => J_ROTATIONS,
            O => O_ROTATIONS,
        }
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
        match self {
            I => writeln!(f, "I"),
            J => writeln!(f, "J"),
            O => writeln!(f, "O"),
        }
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

    pub fn block(&mut self) -> BlockType {
        match self.sampler.sample(&mut self.rng) {
            1 => I,
            2 => J,
            3 => O,
            i => unreachable!(
                "Only {N_BLOCK_TYPES} block types are implemented, but sampler returned {i}",
            ),
        }
    }
}
