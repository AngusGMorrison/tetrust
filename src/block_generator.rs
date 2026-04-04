use rand::rngs::ThreadRng;
use rand_distr::{Distribution, Uniform};

use crate::block::BlockType;

/// Randomly generates new blocks based on the supplied RNG.
#[derive(Debug, Clone)]
pub struct BlockGenerator<S> {
    rng: ThreadRng,
    sampler: S,
}

impl BlockGenerator<Uniform<u8>> {
    pub fn new() -> Self {
        let sampler = Uniform::new_inclusive(1, BlockType::COUNT)
            .unwrap_or_else(|_| panic!("uniform sampler was invalid for 1..={}", BlockType::COUNT));
        Self { rng: rand::rng(), sampler }
    }
}

impl<S: Distribution<u8>> BlockGenerator<S> {
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
pub(crate) mod test_helpers {
    use rand::Rng;
    use rand_distr::Distribution;

    use super::*;

    pub(crate) struct MockSampler(pub(crate) u8);

    impl Distribution<u8> for MockSampler {
        fn sample<R: Rng + ?Sized>(&self, _rng: &mut R) -> u8 {
            self.0
        }
    }

    impl BlockGenerator<MockSampler> {
        pub(crate) fn with_mock_sampler(value: u8) -> Self {
            Self { rng: rand::rng(), sampler: MockSampler(value) }
        }
    }
}

#[cfg(test)]
mod block_generator_tests {
    use super::*;

    mod block_tests {
        use super::*;

        #[test]
        #[should_panic]
        fn when_sampler_produces_out_of_range_value_panics() {
            let mut generator = BlockGenerator::with_mock_sampler(0);
            generator.block();
        }

        #[test]
        fn when_sampler_produces_1_returns_i() {
            let mut generator = BlockGenerator::with_mock_sampler(1);
            assert_eq!(generator.block(), BlockType::I);
        }

        #[test]
        fn when_sampler_produces_2_returns_j() {
            let mut generator = BlockGenerator::with_mock_sampler(2);
            assert_eq!(generator.block(), BlockType::J);
        }

        #[test]
        fn when_sampler_produces_3_returns_l() {
            let mut generator = BlockGenerator::with_mock_sampler(3);
            assert_eq!(generator.block(), BlockType::L);
        }

        #[test]
        fn when_sampler_produces_4_returns_o() {
            let mut generator = BlockGenerator::with_mock_sampler(4);
            assert_eq!(generator.block(), BlockType::O);
        }

        #[test]
        fn when_sampler_produces_5_returns_s() {
            let mut generator = BlockGenerator::with_mock_sampler(5);
            assert_eq!(generator.block(), BlockType::S);
        }

        #[test]
        fn when_sampler_produces_6_returns_t() {
            let mut generator = BlockGenerator::with_mock_sampler(6);
            assert_eq!(generator.block(), BlockType::T);
        }

        #[test]
        fn when_sampler_produces_7_returns_z() {
            let mut generator = BlockGenerator::with_mock_sampler(7);
            assert_eq!(generator.block(), BlockType::Z);
        }
    }
}
