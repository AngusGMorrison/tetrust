use std::{
    io::{self},
    thread,
    time::Duration,
};

use crossterm::event::{self as termevent, KeyCode};
use crossterm::event::{Event as TermEvent, KeyEventKind};
use rand::rngs::ThreadRng;
use ratatui::widgets::Paragraph;
use tetrust::{
    block::BlockGenerator,
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

fn render(frame: &mut ratatui::Frame, state: &GameState<ThreadRng>) {
    let text = if state.game_over() {
        "GAME_OVER".to_string()
    } else {
        state.to_string()
    };
    frame.render_widget(Paragraph::new(text), frame.area())
}

fn poll_input(poll_duration: Duration) -> io::Result<Option<Event>> {
    if termevent::poll(poll_duration)? {
        match termevent::read()? {
            TermEvent::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Char('q') => Ok(Some(Event::Quit)),
                    KeyCode::Left => Ok(Some(Event::Move(Direction::Left))),
                    KeyCode::Right => Ok(Some(Event::Move(Direction::Right))),
                    KeyCode::Down => Ok(Some(Event::Gravity)),
                    _ => Ok(None),
                }
            }
            _ => Ok(None),
        }
    } else {
        Ok(None)
    }
}
