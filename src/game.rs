use std::{cell::RefCell, collections::VecDeque, fmt};

use rand::Rng;

use crate::{
    block::{Block, BlockGenerator, Position, Rotation},
    board::{BOARD_COLS, Board},
};

/// The maxiumum number of blocks that may be queued.
const QUEUE_LEN: usize = 3;

#[derive(Debug, Clone)]
pub struct ActiveBlock {
    // The row-column coordinates of the top-left corner of the block's [BoundingBox].
    top_left: Position,
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
            top_left: (r, c),
            block,
        }
    }

    // Returns the board-space coordinates of the top-left cell of the ActiveBlock's bounding box.
    pub fn top_left(&self) -> Position {
        self.top_left
    }

    // /// Returns the board-space coordinates of the bottom-right cell of the ActiveBlock's bounding
    // /// box.
    // pub fn bottom_right(&self) -> Position {}

    pub fn rotation(&self) -> &Rotation {
        self.block.rotation()
    }
}

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
    board: RefCell<Board>,
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
        println!("spawning block at position {:?}", active_block.top_left());

        // Populate the queue with random blocks.
        let mut queue: VecDeque<Block> = (0..QUEUE_LEN).map(|_| block_generator.block()).collect();
        queue.make_contiguous(); // simplifies returning the queue to the game loop

        GameState {
            score: 0,
            board: RefCell::new(Board::new()),
            block_generator,
            active_block,
            queue,
            game_over: false,
        }
    }

    pub fn score(&self) -> u32 {
        self.score
    }

    pub fn game_over(&self) -> bool {
        self.game_over
    }

    pub fn queue(&self) -> &[Block] {
        let (front, back) = self.queue.as_slices();
        debug_assert_eq!(
            back.len(),
            0,
            "Back slice of block queue was non-empty. This indicates that the queue wasn't made contiguous after inserting a new block.",
        );
        front
    }

    // Update the [GameState] in response to an [Event]. Does nothing if called when the game is
    // over.
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

    fn handle_gravity(&mut self) {
        let mut next_position = self.active_block.top_left; // TODO: This needs to work with the block's lowest point, not its highest
        next_position.0 += 1;
        if self.board.borrow().occupied(next_position) {
            self.handle_landing()
        } else {
            self.active_block.top_left = next_position;
        }
    }

    fn handle_landing(&mut self) {
        // Add the active block to the board.
        self.board.borrow_mut().fix_active_block(&self.active_block);

        // Clear lines and update the score.
        let lines_cleared = self.board.borrow_mut().clear_lines();
        self.score += lines_cleared as u32;

        // Handle game over or set up the next block.
        if self.board.borrow().buffer_zone_occupied() {
            self.game_over = true
        } else {
            self.load_next_active_block();
        }
    }

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
        self.board.borrow_mut().fix_active_block(&self.active_block);
        self.board.borrow().fmt(f)?;
        self.board
            .borrow_mut()
            .remove_active_block(&self.active_block);
        Ok(())
    }
}
