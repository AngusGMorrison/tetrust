use std::{
    io::{self},
    thread,
    time::Duration,
};

use crossterm::event::{self as termevent, KeyCode};
use crossterm::event::{Event as TermEvent, KeyEventKind};
use rand::rngs::ThreadRng;
use ratatui::{
    layout::{Constraint, Layout},
    style::Stylize,
    text::Text,
};
use tetrust::{
    block::BlockGenerator,
    board::{BOARD_COLS, BOARD_ROWS, PLAYABLE_ROWS},
    game::{Direction, Event, GameState},
    timer::GameTimer,
};

/// The number of ticks that must elapse between applications of gravity.
const INITIAL_GRAVITY_TICKS: u64 = 12;

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

const GAME_AREA_HEIGHT: u16 = PLAYABLE_ROWS as u16 + BORDER_THICKNESS * 2;

/// The number of rendered columns is double the columns of the board, since square cells are
/// rendered using two █ characters: ██.
const GAME_AREA_WIDTH: u16 = BOARD_COLS as u16 * 2 + BORDER_THICKNESS * 2;

fn render(frame: &mut ratatui::Frame, state: &GameState<ThreadRng>) {
    let header = Text::from_iter([
        "TETRUST".bold(),
        "<q> Quit | <←|→> Move block | <↓> Drop block | <z|x> Rotate block".into(),
    ]);

    let layout = Layout::vertical([
        Constraint::Length(header.height() as u16),
        Constraint::Length(GAME_AREA_HEIGHT),
    ]);
    let [text_area, mut game_area] = frame.area().layout(&layout);
    frame.render_widget(header.centered(), text_area);

    let game_area_layout = Layout::horizontal([GAME_AREA_WIDTH]);
    [game_area] = game_area.layout::<1>(&game_area_layout);
    frame.render_widget(state.canvas(), game_area)
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
