use rand::Rng;
use rand_distr::{Distribution, Uniform};

use crate::block::BlockType;

/// Randomly generates new blocks based on the supplied RNG.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockGenerator<R, S> {
    rng: R,
    sampler: S,
}

impl<R> BlockGenerator<R, Uniform<u8>> {
    pub fn new(rng: R) -> Self {
        let sampler = Uniform::new_inclusive(1, BlockType::COUNT)
            .unwrap_or_else(|_| panic!("uniform sampler was invalid for 1..={}", BlockType::COUNT));
        Self { rng, sampler }
    }
}

impl<R: Rng, S: Distribution<u8>> BlockGenerator<R, S> {
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

#[cfg(test)]
mod block_generator_tests {
    use super::*;
    use rand_distr::Distribution;

    struct MockSampler(u8);

    impl Distribution<u8> for MockSampler {
        fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> u8 {
            self.0
        }
    }

    mod block_tests {
        use super::*;

        #[test]
        #[should_panic]
        fn when_sampler_produces_out_of_range_value_panics() {
            BlockGenerator {
                rng: rand::rng(),
                sampler: MockSampler(0),
            }
            .block();
        }

        #[test]
        fn when_sampler_produces_1_returns_i() {
            let mut generator = BlockGenerator {
                rng: rand::rng(),
                sampler: MockSampler(1),
            };
            assert_eq!(generator.block(), BlockType::I);
        }

        #[test]
        fn when_sampler_produces_2_returns_j() {
            let mut generator = BlockGenerator {
                rng: rand::rng(),
                sampler: MockSampler(2),
            };
            assert_eq!(generator.block(), BlockType::J);
        }

        #[test]
        fn when_sampler_produces_3_returns_l() {
            let mut generator = BlockGenerator {
                rng: rand::rng(),
                sampler: MockSampler(3),
            };
            assert_eq!(generator.block(), BlockType::L);
        }

        #[test]
        fn when_sampler_produces_4_returns_o() {
            let mut generator = BlockGenerator {
                rng: rand::rng(),
                sampler: MockSampler(4),
            };
            assert_eq!(generator.block(), BlockType::O);
        }

        #[test]
        fn when_sampler_produces_5_returns_s() {
            let mut generator = BlockGenerator {
                rng: rand::rng(),
                sampler: MockSampler(5),
            };
            assert_eq!(generator.block(), BlockType::S);
        }

        #[test]
        fn when_sampler_produces_6_returns_t() {
            let mut generator = BlockGenerator {
                rng: rand::rng(),
                sampler: MockSampler(6),
            };
            assert_eq!(generator.block(), BlockType::T);
        }

        #[test]
        fn when_sampler_produces_7_returns_z() {
            let mut generator = BlockGenerator {
                rng: rand::rng(),
                sampler: MockSampler(7),
            };
            assert_eq!(generator.block(), BlockType::Z);
        }
    }
}
