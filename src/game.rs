use std::{collections::VecDeque, fmt};

use rand::Rng;
use ratatui::style::Stylize;
use ratatui::symbols::Marker;
use ratatui::text::{Span};
use ratatui::widgets::canvas::Canvas;
use ratatui::widgets::{Block, Widget};

use crate::block::Position;
use crate::board::{BOARD_ROWS, BUFFER_ZONE_ROWS, PLAYABLE_ROWS};
use crate::{
    block::{ActiveBlock, BlockGenerator, BlockType},
    board::{BOARD_COLS, Board},
};

/// The maxiumum number of blocks that may be queued.
const QUEUE_LEN: usize = 3;

/// A direction of movement or rotation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

// The [GameState] is updated in response to events passed to [GameState::update]. This decouples
// the representation of the game's state from concepts such as the game loop.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Event {
    Quit,
    Move(Direction),
    Rotate(Direction),
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
        let mut queue: VecDeque<BlockType> =
            (0..QUEUE_LEN).map(|_| block_generator.block()).collect();
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
            Move(direction) => self.handle_move(direction),
            Rotate(direction) => self.handle_rotate(direction),
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

    pub fn canvas(&self) -> impl Widget {
        Canvas::default()
            // Bordering the canvas adds 2 to its vertical and horizontal dimensions. The layout
            // it's rendered to must provide exactly enough room for the board and its borders to
            // avoid artifacts from the resolution mismatch.
            .block(Block::bordered().title("Tetrust"))
            // x_bounds and y_bounds define the canvas' viewport - inside its borders.
            //
            // Due to ratatui's internal rendering logic, stepping by two columns on each loop
            // iteration to render double-width blocks (██), requires a negative x-offset to avoid
            // blocks slipping behind the left border of the canvas.
            .x_bounds([-1.0, (BOARD_COLS * 2) as f64 - 1.0])
            // The y-bounds don't require an offset, since we're stepping by one row each time.
            .y_bounds([0.0, (BOARD_ROWS - BUFFER_ZONE_ROWS - 1) as f64])
            .marker(Marker::HalfBlock)
            .paint(|ctx| {
                let mut block_positions = self.active_block.board_positions().peekable();
                let stylize = stylizer(self.active_block.block_type());
                for (ir, r) in self.board.iter().skip(2).enumerate() {
                    for (ic, c) in r.iter().enumerate() {
                        let (x, y) = to_terminal_coords((ir, ic));
                        match block_positions.peek() {
                            Some((m, n)) if *m == ir + BUFFER_ZONE_ROWS && *n == ic => {
                                ctx.print(x, y, stylize("██"));
                                block_positions.next();
                            }
                            _ => {
                                if *c == 1 {
                                    ctx.print(x, y, "██".black());
                                }
                            }
                        }
                    }
                }
            })
    }
}

/// Converts a (row, col) board position to (x, y) terminal coordinates, where y = 0 at the bottom
/// of the terminal area.
fn to_terminal_coords((row, col): Position) -> (f64, f64) {
    (
        // Widths are doubled, since square tiles are achieved using two █ characters: ██.
        (col * 2) as f64,
        // Rows are counted from the bottom of the area instead of the top.
        (PLAYABLE_ROWS - row - 1) as f64,
    )
}

fn stylizer<'a>(block_type: BlockType) -> fn(&'a str) -> Span<'a> {
    use BlockType::*;
    match block_type {
        I => Stylize::yellow,
        J => Stylize::green,
        O => Stylize::red,
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
                        write!(f, "▀")?;
                        block_positions.next();
                    }
                    _ => {
                        let symbol = if *v == 1 { "▀" } else { " " };
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
