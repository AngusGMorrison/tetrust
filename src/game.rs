use std::collections::VecDeque;

use rand::Rng;

use crate::{
    block::{Block, BlockGenerator},
    board::{BOARD_COLS, Board},
};

/// The maxiumum number of blocks that may be queued.
const QUEUE_LEN: usize = 3;

#[derive(Debug, Clone)]
struct ActiveBlock {
    // The row-column coordinates of the top-left corner of the block's [BoundingBox].
    position: (usize, usize),
    block: Block,
}

impl ActiveBlock {
    fn new(block: Block) -> Self {
        let height = block.height();
        debug_assert!(
            height <= 2,
            "Block starting height was {}. Any height greater than 2 places it out of bounds.",
            height
        );

        let width = block.width();
        debug_assert!(
            width <= BOARD_COLS,
            "Block width {} exceeds board width {}",
            width,
            BOARD_COLS,
        );

        // The initial row coordinate is the lowest possible row in the two-row buffer zone that
        // places the block fully out of sight of the user.
        //
        // For example, the I block's initial position is lying horizontally on the 1st row. The
        // O block's initial position places its top-left corner on the 0th row.
        let r = 2 - height;

        // The initial column coordinate places the block approximately in the center of the board.
        //
        // For example, on a standard 10-column board, the I block's leftmost cell falls in row[3],
        // while the O and S blocks' fall in row[4]. This gives a one-cell rightwards bias to
        // three-cell-wide blocks.
        let c = BOARD_COLS / 2 - block.width() / 2;

        Self {
            position: (r, c),
            block,
        }
    }
}

// The [GameState] is updated in response to events passed to [GameState::update]. This decouples
// the representation of the game's state from concepts such as the game loop.
pub enum Event {
    Move,
    Gravity,
}

/// State machine representing a game of Tetrust.
#[derive(Debug, Clone)]
pub struct GameState<R: Rng> {
    score: u32,
    board: Board,
    block_generator: BlockGenerator<R>,
    active_block: ActiveBlock,
    queue: VecDeque<Block>,
    game_over: bool,
}

impl<R: Rng> GameState<R> {
    /// Instantiate a new game using the given [BlockGenerator] as its source of [Block]s.
    pub fn new(mut block_generator: BlockGenerator<R>) -> Self {
        let first_block = block_generator.block();
        let active_block = ActiveBlock::new(first_block);

        // Populate the queue with random blocks.
        let queue: VecDeque<Block> = (0..QUEUE_LEN).map(|_| block_generator.block()).collect();

        GameState {
            score: 0,
            board: Board::new(),
            block_generator,
            active_block,
            queue,
            game_over: false,
        }
    }

    fn score(&self) -> u32 {
        self.score
    }

    fn game_over(&self) -> bool {
        self.game_over
    }

    pub fn update(&mut self, event: Event) {
        use Event::*;

        match event {
            Gravity => self.apply_gravity(),
            _ => unimplemented!(),
        }
    }

    fn apply_gravity(&mut self) {}
}

// impl<R> fmt::Display for Game<R> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

//     }
// }
