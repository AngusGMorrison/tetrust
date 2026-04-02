use std::collections::VecDeque;
use std::io;
use std::time::Duration;

use rand::Rng;

use crate::block_generator::BlockGenerator;
use crate::config::Config;
use crate::input::{Input, PollInput};
use crate::timer::{GameTimer, Tick};
use crate::{
    block::{ActiveBlock, BlockType},
    board::Board,
};

/// The maxiumum number of blocks that may be queued.
const QUEUE_LEN: usize = 3;

/// A direction of movement or rotation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

/// A game of Tetrust.
#[derive(Debug)]
pub struct Game<R, I> {
    config: Config,
    score: u32,
    board: Board,
    block_generator: BlockGenerator<R>,
    active_block: ActiveBlock,
    queue: VecDeque<BlockType>,
    game_over: bool,
    timer: GameTimer,
    input: I,
}

pub enum UpdateOutcome {
    Unchanged,
    Updated,
    Quit,
}

impl<R, I> Game<R, I> {
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

    pub fn time_until_next_tick(&self) -> Duration {
        self.timer.time_until_next_tick()
    }

    pub(crate) fn active_block(&self) -> &ActiveBlock {
        &self.active_block
    }

    pub(crate) fn board(&self) -> &Board {
        &self.board
    }
}

impl<R: Rng, I: PollInput> Game<R, I> {
    /// Instantiate a new game using the given [BlockGenerator] as its source of [Block]s.
    pub fn new(mut block_generator: BlockGenerator<R>, input: I, config: Config) -> Self {
        let first_block = block_generator.block();
        let active_block = ActiveBlock::new(first_block);

        // Populate the queue with random blocks.
        let mut queue: VecDeque<BlockType> =
            (0..QUEUE_LEN).map(|_| block_generator.block()).collect();
        queue.make_contiguous(); // simplifies returning the queue to the game loop

        let timer = GameTimer::new(
            config.frame_interval,
            config.gravity.initial_ticks(),
            config.input_ticks,
        );

        Game {
            config,
            timer,
            score: 0,
            board: Board::new(),
            block_generator,
            active_block,
            queue,
            game_over: false,
            input,
        }
    }

    /// Begins a new game.
    fn restart(&mut self) {
        self.timer = GameTimer::new(
            self.config.frame_interval,
            self.config.gravity.initial_ticks(),
            self.config.input_ticks,
        );
        self.score = 0;
        self.board = Board::new();

        let first_block = self.block_generator.block();
        self.active_block = ActiveBlock::new(first_block);

        self.queue.clear();
        (0..QUEUE_LEN).for_each(|_| self.queue.push_back(self.block_generator.block()));
        self.queue.make_contiguous();

        self.game_over = false
    }

    /// Drives the game loop at a maxmimum rate determined by the [GameTimer]'s tick interval.
    pub fn update(&mut self) -> io::Result<UpdateOutcome> {
        if let Some(tick) = self.timer.update() {
            if self.game_over() {
                return self.update_game_over(&tick);
            } else {
                return self.update_game_in_progress(&tick);
            }
        }

        Ok(UpdateOutcome::Unchanged)
    }

    /// Manages updates that are valid in the game over state.
    fn update_game_over(&mut self, tick: &Tick) -> io::Result<UpdateOutcome> {
        if tick.input {
            match self.input.poll_input(self.timer.time_until_next_tick())? {
                Input::Quit => return Ok(UpdateOutcome::Quit),
                Input::Restart => {
                    self.restart();
                    return Ok(UpdateOutcome::Updated);
                }
                _ => (),
            }
        }
        Ok(UpdateOutcome::Unchanged)
    }

    /// Manages updates that are valid while the game is in progress.
    fn update_game_in_progress(&mut self, tick: &Tick) -> io::Result<UpdateOutcome> {
        if tick.gravity {
            self.handle_gravity();
        }

        if tick.input {
            use crate::input::Input::*;
            match self.input.poll_input(self.timer.time_until_next_tick())? {
                Down => self.handle_gravity(),
                Left => self.handle_move(Direction::Left),
                Right => self.handle_move(Direction::Right),
                RotateLeft => self.handle_rotate(Direction::Left),
                RotateRight => self.handle_rotate(Direction::Right),
                Restart => {
                    self.restart();
                    return Ok(UpdateOutcome::Updated);
                }
                Quit => return Ok(UpdateOutcome::Quit),
                _ => (),
            }
        }

        if tick.any() {
            Ok(UpdateOutcome::Updated)
        } else {
            Ok(UpdateOutcome::Unchanged)
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
        if lines_cleared > 0
            && self
                .score
                .is_multiple_of(self.config.accelerate_every_n_points)
        {
            self.accelerate();
        }

        // Handle game over or set up the next block.
        if self.board.buffer_zone_occupied() {
            self.game_over = true
        } else {
            self.load_next_active_block();
        }
    }

    /// Increase the rate at which blocks fall under gravity by decreasing the number of game ticks
    /// between gravity applications.
    fn accelerate(&mut self) {
        let current_gravity_ticks = self.timer.gravity_ticks();
        // Gravity ticks cannot be decreased below the minimum ticks specified at the start of
        // the game.
        let next_gravity_ticks = current_gravity_ticks
            .saturating_sub(self.config.gravity.acceleration())
            .max(self.config.gravity.min_ticks());
        self.timer.set_gravity_ticks(next_gravity_ticks);
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

    fn handle_move(&mut self, direction: Direction) {
        let undo = if direction == Direction::Left {
            self.active_block.move_left();
            ActiveBlock::move_right
        } else {
            self.active_block.move_right();
            ActiveBlock::move_left
        };

        if self.board.collides(&self.active_block) {
            undo(&mut self.active_block)
        }
    }

    fn handle_rotate(&mut self, direction: Direction) {
        let undo = if direction == Direction::Left {
            self.active_block.rotate_counter_clockwise();
            ActiveBlock::rotate_clockwise
        } else {
            self.active_block.rotate_clockwise();
            ActiveBlock::rotate_counter_clockwise
        };

        if self.board.collides(&self.active_block) {
            undo(&mut self.active_block)
        }
    }
}
