use std::{thread, time::Duration};

use indoc::indoc;
use rand::rngs::ThreadRng;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Text,
    widgets::{Block, Borders, Paragraph},
};
use tetrust::{
    block::BlockGenerator,
    board::{BOARD_COLS, PLAYABLE_ROWS},
    game::{Config, Game, Gravity, UpdateOutcome},
    input::Stdin,
};

/// The number of ticks that must elapse between applications of gravity.
const INITIAL_GRAVITY_TICKS: u64 = 48;

const MIN_GRAVITY_TICKS: u64 = 12;

const ACCELERATION: u64 = 4;

const ACCELERATE_EVERY_N_POINTS: u32 = 5;

/// The number of ticks that must elapse between reads of user input.
const INPUT_TICKS: u64 = 1;

fn main() -> Result<(), String> {
    ratatui::run(|terminal| -> Result<(), String> {
        let block_generator = BlockGenerator::new(rand::rng());
        let frame_interval = Duration::from_secs_f32(1.0 / 60.0);
        let config = Config {
            gravity: Gravity::new(INITIAL_GRAVITY_TICKS, MIN_GRAVITY_TICKS, ACCELERATION)?,
            frame_interval,
            accelerate_every_n_points: ACCELERATE_EVERY_N_POINTS,
            input_ticks: INPUT_TICKS,
        };
        let mut game = Game::new(
            block_generator,
            Stdin,
            config,
        );

        loop {
            match game.update().map_err(|e| e.to_string())? {
                UpdateOutcome::Updated => {
                    _ = terminal
                        .draw(|frame| render(frame, &game))
                        .map_err(|e| e.to_string())?
                }
                UpdateOutcome::Quit => return Ok(()),
                _ => (),
            }

            thread::sleep(game.time_until_next_tick())
        }
    })
}

const BORDER_THICKNESS: u16 = 1;

const BOARD_HEIGHT: u16 = PLAYABLE_ROWS as u16 + BORDER_THICKNESS * 2;

/// The number of rendered columns is double the columns of the board, since square cells are
/// rendered using two █ characters: ██.
const BOARD_WIDTH: u16 = BOARD_COLS as u16 * 2 + BORDER_THICKNESS * 2;

const BOARD_SIDEBAR_PADDING: u16 = 2;

const SIDEBAR_WIDTH: u16 = 8;

const SCORE_WIDGET_HEIGHT: u16 = 3;

const NEXT_BLOCK_WIDGET_HEIGHT: u16 = 4;

fn render(frame: &mut ratatui::Frame, state: &Game<ThreadRng, Stdin>) {
    let header = Text::from_iter([
        "TETRUST".bold(),
        "<←|↓|→> Move | <z|x> Rotate | <r> Restart | <q> Quit".into(),
    ]);

    let [text_area, _, game_area] = frame.area().layout(&Layout::vertical([
        Constraint::Length(header.height() as u16),
        Constraint::Length(1),
        Constraint::Length(BOARD_HEIGHT),
    ]));
    frame.render_widget(header.centered(), text_area);

    if state.game_over() {
        render_game_over(frame, game_area);
    } else {
        render_game(frame, game_area, state);
    }
}

fn render_game(frame: &mut ratatui::Frame, game_area: Rect, state: &Game<ThreadRng, Stdin>) {
    let [_, board, _, score_col, _] = game_area.layout::<5>(&Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length(BOARD_WIDTH),
        Constraint::Length(BOARD_SIDEBAR_PADDING),
        Constraint::Length(SIDEBAR_WIDTH),
        Constraint::Fill(1),
    ]));
    frame.render_widget(state.canvas(), board);
    render_sidebar(frame, score_col, state)
}

fn render_sidebar(frame: &mut ratatui::Frame, sidebar: Rect, state: &Game<ThreadRng, Stdin>) {
    let [score_rect, _, next_block_rect, _] = sidebar.layout(&Layout::vertical([
        Constraint::Length(SCORE_WIDGET_HEIGHT),
        Constraint::Length(1),
        Constraint::Length(NEXT_BLOCK_WIDGET_HEIGHT),
        Constraint::Fill(1),
    ]));

    render_score(frame, score_rect, state);
    render_next_block(frame, next_block_rect, state);
}

fn render_score(frame: &mut ratatui::Frame, rect: Rect, state: &Game<ThreadRng, Stdin>) {
    let score_text = Paragraph::new(Text::from(state.score().to_string()).bold())
        .right_aligned()
        .block(Block::new().borders(Borders::ALL).title("Score"));
    frame.render_widget(score_text, rect);
}

fn render_next_block(frame: &mut ratatui::Frame, rect: Rect, state: &Game<ThreadRng, Stdin>) {
    let next_block = Paragraph::new(state.queue()[0].schematic())
        .left_aligned()
        .block(Block::new().borders(Borders::ALL).title("Next"));
    frame.render_widget(next_block, rect);
}

fn render_game_over(frame: &mut ratatui::Frame, game_rect: Rect) {
    const TOP_PADDING: u16 = 7;
    const TEXT_HEIGHT: u16 = 2;
    let [_, text_rect, _] = game_rect.layout(&Layout::vertical([
        Constraint::Length(TOP_PADDING),
        Constraint::Length(TEXT_HEIGHT),
        Constraint::Fill(1),
    ]));
    let message = Paragraph::new(Text::from(game_over_text()).bold().red()).centered();
    frame.render_widget(message, text_rect)
}

const fn game_over_text() -> &'static str {
    indoc! {"
        ▄▀▀  ▄▀▀▄ █▄▄█ █▀▀   ▄▀▀▄ █  █ █▀▀ █▀▀▄
        ▀▄▄▀ █▀▀█ █  █ ██▄   ▀▄▄▀ ▀▄▄▀ ██▄ █▀▀▄
    "}
}
