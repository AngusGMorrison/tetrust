use indoc::indoc;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::Marker,
    text::Text,
    widgets::{Block, Borders, Paragraph, Widget, canvas::Canvas},
};

use crate::{
    block::Position,
    board::{BOARD_COLS, BOARD_ROWS, BUFFER_ZONE_ROWS, PLAYABLE_ROWS},
    game::Game,
};

const BORDER_THICKNESS: u16 = 1;

const BOARD_HEIGHT: u16 = PLAYABLE_ROWS as u16 + BORDER_THICKNESS * 2;

/// The number of rendered columns is double the columns of the board, since square cells are
/// rendered using two █ characters: ██.
const BOARD_WIDTH: u16 = BOARD_COLS as u16 * 2 + BORDER_THICKNESS * 2;

const BOARD_SIDEBAR_PADDING: u16 = 2;

const SIDEBAR_WIDTH: u16 = 8;

const SCORE_WIDGET_HEIGHT: u16 = 3;

const NEXT_BLOCK_WIDGET_HEIGHT: u16 = 4;

impl<R, I> Widget for &Game<R, I> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let header = Text::from_iter([
            "TETRUST".bold(),
            "<←|↓|→> Move | <z|x> Rotate | <r> Restart | <q> Quit".into(),
        ]);

        let [text_area, _, game_area] = area.layout(&Layout::vertical([
            Constraint::Length(header.height() as u16),
            Constraint::Length(1),
            Constraint::Length(BOARD_HEIGHT),
        ]));

        header.centered().render(text_area, buf);

        if self.game_over() {
            render_game_over(game_area, buf);
        } else {
            self.render_game_in_progress(game_area, buf);
        }
    }
}

impl<R, I> Game<R, I> {
    fn render_game_in_progress(&self, game_area: Rect, buf: &mut Buffer) {
        let [_, board_area, _, sidebar_area, _] = game_area.layout::<5>(&Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Length(BOARD_WIDTH),
            Constraint::Length(BOARD_SIDEBAR_PADDING),
            Constraint::Length(SIDEBAR_WIDTH),
            Constraint::Fill(1),
        ]));
        self.render_board(board_area, buf);
        self.render_sidebar(sidebar_area, buf);
    }

    fn render_board(&self, board_area: Rect, buf: &mut Buffer) {
        Canvas::default()
            // Bordering the canvas adds 2 to its vertical and horizontal dimensions. The layout
            // it's rendered to must provide exactly enough room for the board and its borders to
            // avoid artifacts from the resolution mismatch.
            .block(Block::bordered())
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
                // Iterate over all cells of the board and active block.
                let mut active_block_positions = self.active_block().board_positions().peekable();
                for (i_row, row) in self.board().iter().skip(BUFFER_ZONE_ROWS).enumerate() {
                    for (i_col, cell) in row.iter().enumerate() {
                        let (x, y) = to_terminal_coords((i_row, i_col));
                        match active_block_positions.peek() {
                            // If the current position is an active block position inside the
                            // buffer zone, skip the cell.
                            Some((i_ab_row, _)) if *i_ab_row < BUFFER_ZONE_ROWS => {
                                active_block_positions.next();
                            }
                            // If the current position is an active block position which is on the
                            // visible board, render the current active block cell and advance the
                            // iterator to the next.
                            Some((i_ab_row, i_ab_col))
                                if *i_ab_row == i_row + BUFFER_ZONE_ROWS && *i_ab_col == i_col =>
                            {
                                ctx.print(x, y, self.active_block().grid_cell());
                                active_block_positions.next();
                            }
                            // Otherwise, render the fixed cell from the board.
                            _ => {
                                if let Some(block_type) = cell {
                                    ctx.print(x, y, block_type.grid_cell());
                                }
                            }
                        }
                    }
                }
            })
            .render(board_area, buf)
    }

    fn render_sidebar(&self, sidebar_area: Rect, buf: &mut Buffer) {
        let [score_area, _, next_block_area, _] = sidebar_area.layout(&Layout::vertical([
            Constraint::Length(SCORE_WIDGET_HEIGHT),
            Constraint::Length(1),
            Constraint::Length(NEXT_BLOCK_WIDGET_HEIGHT),
            Constraint::Fill(1),
        ]));

        self.render_score(score_area, buf);
        self.render_next_block(next_block_area, buf);
    }

    fn render_score(&self, score_area: Rect, buf: &mut Buffer) {
        let score_text = Paragraph::new(Text::from(self.score().to_string()).bold())
            .right_aligned()
            .block(Block::new().borders(Borders::ALL).title("Score"));
        score_text.render(score_area, buf)
    }

    fn render_next_block(&self, next_block_area: Rect, buf: &mut Buffer) {
        let next_block = Paragraph::new(self.queue()[0].schematic())
            .left_aligned()
            .block(Block::new().borders(Borders::ALL).title("Next"));
        next_block.render(next_block_area, buf)
    }
}

fn render_game_over(game_rect: Rect, buf: &mut Buffer) {
    const TOP_PADDING: u16 = 7;
    const TEXT_HEIGHT: u16 = 2;
    let [_, text_rect, _] = game_rect.layout(&Layout::vertical([
        Constraint::Length(TOP_PADDING),
        Constraint::Length(TEXT_HEIGHT),
        Constraint::Fill(1),
    ]));
    let message = Paragraph::new(Text::from(game_over_text()).bold().red()).centered();
    message.render(text_rect, buf);
}

const fn game_over_text() -> &'static str {
    indoc! {"
        ▄▀▀  ▄▀▀▄ █▄▄█ █▀▀   ▄▀▀▄ █  █ █▀▀ █▀▀▄
        ▀▄▄▀ █▀▀█ █  █ ██▄   ▀▄▄▀ ▀▄▄▀ ██▄ █▀▀▄
    "}
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
