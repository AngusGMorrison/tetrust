use std::{
    io::{self},
    thread,
    time::Duration,
};

use crossterm::event::{self as termevent, KeyCode};
use crossterm::event::{Event as TermEvent, KeyEventKind};
use indoc::indoc;
use rand::rngs::ThreadRng;
use ratatui::{
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    symbols::Marker,
    text::Text,
    widgets::{Block, Borders, Paragraph, canvas::Canvas},
};
use tetrust::{
    block::BlockGenerator,
    board::{BOARD_COLS, PLAYABLE_ROWS},
    game::{Direction, Event, GameState},
    timer::GameTimer,
};

/// The number of ticks that must elapse between applications of gravity.
const INITIAL_GRAVITY_TICKS: u64 = 48;

/// The number of ticks that must elapse between reads of user input.
const INPUT_TICKS: u64 = 1;

fn main() -> io::Result<()> {
    ratatui::run(|terminal| -> io::Result<()> {
        let block_generator = BlockGenerator::new(rand::rng());
        let mut state = GameState::new(block_generator);
        let frame_interval = Duration::from_secs_f32(1.0 / 60.0);
        let mut timer = GameTimer::new(frame_interval, INITIAL_GRAVITY_TICKS, INPUT_TICKS);

        loop {
            if let Some(tick) = timer.update() {
                if tick.gravity {
                    state.update(Event::Gravity);
                }

                if tick.input
                    && let Some(event) = poll_input(timer.time_until_next_tick())?
                {
                    if event == Event::Quit {
                        return Ok(());
                    } else {
                        state.update(event)
                    }
                }

                terminal.draw(|frame| render(frame, &state))?;
            }

            thread::sleep(timer.time_until_next_tick())
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

fn render(frame: &mut ratatui::Frame, state: &GameState<ThreadRng>) {
    let header = Text::from_iter([
        "TETRUST".bold(),
        "<q> Quit | <←|→> Move block | <↓> Drop block | <z|x> Rotate block".into(),
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

fn render_game(frame: &mut ratatui::Frame, game_area: Rect, state: &GameState<ThreadRng>) {
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

fn render_sidebar(frame: &mut ratatui::Frame, sidebar: Rect, state: &GameState<ThreadRng>) {
    let [score_rect, _, next_block_rect, _] = sidebar.layout(&Layout::vertical([
        Constraint::Length(SCORE_WIDGET_HEIGHT),
        Constraint::Length(1),
        Constraint::Length(NEXT_BLOCK_WIDGET_HEIGHT),
        Constraint::Fill(1),
    ]));

    render_score(frame, score_rect, state);
    render_next_block(frame, next_block_rect, state);
}

fn render_score(frame: &mut ratatui::Frame, rect: Rect, state: &GameState<ThreadRng>) {
    let score_text = Paragraph::new(Text::from(state.score().to_string()).bold())
        .right_aligned()
        .block(Block::new().borders(Borders::ALL).title("Score"));
    frame.render_widget(score_text, rect);
}

fn render_next_block(frame: &mut ratatui::Frame, rect: Rect, state: &GameState<ThreadRng>) {
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

fn poll_input(poll_duration: Duration) -> io::Result<Option<Event>> {
    if termevent::poll(poll_duration)? {
        match termevent::read()? {
            TermEvent::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Left => Ok(Some(Event::Move(Direction::Left))),
                    KeyCode::Right => Ok(Some(Event::Move(Direction::Right))),
                    KeyCode::Down => Ok(Some(Event::Gravity)),
                    KeyCode::Char('q') => Ok(Some(Event::Quit)),
                    KeyCode::Char('z') => Ok(Some(Event::Rotate(Direction::Left))),
                    KeyCode::Char('x') => Ok(Some(Event::Rotate(Direction::Right))),
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}
