use std::{collections::VecDeque, fmt};

use rand::Rng;

use crate::{
    block::{ActiveBlock, BlockGenerator, BlockType},
    board::{BOARD_COLS, Board},
};

/// The maxiumum number of blocks that may be queued.
const QUEUE_LEN: usize = 3;

// The [GameState] is updated in response to events passed to [GameState::update]. This decouples
// the representation of the game's state from concepts such as the game loop.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Event {
    Move,
    Rotate,
    Gravity,
}

/// State machine representing a game of Tetrust.
#[derive(Debug, Clone)]
pub struct GameState<R: Rng> {
    score: u32,
    board: Board,
    block_generator: BlockGenerator<R>,
    active_block: ActiveBlock,
    queue: VecDeque<BlockType>,
    game_over: bool,
}

impl<R: Rng> GameState<R> {
    /// Instantiate a new game using the given [BlockGenerator] as its source of [Block]s.
    pub fn new(mut block_generator: BlockGenerator<R>) -> Self {
        let first_block = block_generator.block();
        let active_block = ActiveBlock::new(first_block);

        // Populate the queue with random blocks.
        let mut queue: VecDeque<BlockType> = (0..QUEUE_LEN).map(|_| block_generator.block()).collect();
        queue.make_contiguous(); // simplifies returning the queue to the game loop

        GameState {
            score: 0,
            board: Board::new(),
            block_generator,
            active_block,
            queue,
            game_over: false,
        }
    }

    /// Returns the current score.
    pub fn score(&self) -> u32 {
        self.score
    }

    /// Returns true is the game is over, at which point no further events will be handled.
    pub fn game_over(&self) -> bool {
        self.game_over
    }

    /// Returns the current block queue as a contiguous slice.
    pub fn queue(&self) -> &[BlockType] {
        let (front, back) = self.queue.as_slices();
        debug_assert_eq!(
            back.len(),
            0,
            "Back slice of block queue was non-empty. This indicates that the queue wasn't made contiguous after inserting a new block.",
        );
        front
    }

    /// Update the [GameState] in response to an [Event]. Does nothing if called when the game is
    /// over.
    pub fn update(&mut self, event: Event) {
        use Event::*;

        if self.game_over {
            return;
        }

        match event {
            Gravity => self.handle_gravity(),
            _ => unimplemented!(),
        }
    }

    /// Attempts to move the current [ActiveBlock] one row downwards, and handles the resulting
    /// collision if movement is impossible.
    fn handle_gravity(&mut self) {
        self.active_block.move_down();
        if self.board.collides(&self.active_block) {
            self.active_block.move_up();
            self.handle_landing()
        }
    }

    /// Handles the case where a block can no longer move downwards under gravity.
    fn handle_landing(&mut self) {
        // Add the active block to the board.
        self.board.fix_active_block(&self.active_block);

        // Clear lines and update the score.
        let lines_cleared = self.board.clear_lines();
        self.score += lines_cleared as u32;

        // Handle game over or set up the next block.
        if self.board.buffer_zone_occupied() {
            self.game_over = true
        } else {
            self.load_next_active_block();
        }
    }

    /// Pulls the next block off the queue and sets it as the game's active block.
    fn load_next_active_block(&mut self) {
        let next_block = self
            .queue
            .pop_front()
            .expect("Block queue should never be empty");
        self.active_block = ActiveBlock::new(next_block);
        self.queue.push_back(self.block_generator.block());
        self.queue.make_contiguous();
    }
}

impl<R: Rng> fmt::Display for GameState<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "*{}*", "—".repeat(BOARD_COLS))?;
        let mut block_positions = self.active_block.board_positions().peekable();
        for (i, r) in self.board.iter().enumerate() {
            if i == 2 {
                // render the boundary between the buffer zone and the visible board
                writeln!(f, "|{}|", "—".repeat(BOARD_COLS))?;
            }

            write!(f, "|")?;
            for (j, v) in r.iter().enumerate() {
                match block_positions.peek() {
                    Some((m, n)) if *m == i && *n == j => {
                        write!(f, "█")?;
                        block_positions.next();
                    }
                    _ => {
                        let symbol = if *v == 1 { "█" } else { " " };
                        write!(f, "{symbol}")?;
                    }
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "*{}*", "—".repeat(BOARD_COLS))?;
        Ok(())
    }
}
