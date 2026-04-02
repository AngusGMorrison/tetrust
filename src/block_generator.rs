use rand::Rng;
use rand_distr::{Distribution, Uniform};

use crate::block::BlockType;

/// Randomly generates new blocks based on the supplied RNG.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockGenerator<R> {
    rng: R,
    sampler: Uniform<u8>,
}

impl<R> BlockGenerator<R> {
    pub fn new(rng: R) -> Self {
        let sampler = Uniform::new_inclusive(1, BlockType::COUNT)
            .unwrap_or_else(|_| panic!("uniform sampler was invalid for 1..={}", BlockType::COUNT));
        Self { rng, sampler }
    }
}

impl<R: Rng> BlockGenerator<R> {
    /// Generate a new block.
    pub fn block(&mut self) -> BlockType {
        match self.sampler.sample(&mut self.rng) {
            1 => BlockType::I,
            2 => BlockType::J,
            3 => BlockType::L,
            4 => BlockType::O,
            5 => BlockType::S,
            6 => BlockType::T,
            7 => BlockType::Z,
            i => unreachable!(
                "Only {} block types are implemented, but sampler returned {}",
                BlockType::COUNT,
                i
            ),
        }
    }
}
