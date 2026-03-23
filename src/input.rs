use crossterm::event::{self as termevent, Event as TermEvent, KeyCode, KeyEventKind};
use std::{io, time::Duration};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Input {
    None,
    Down,
    Left,
    Right,
    RotateLeft,
    RotateRight,
    Quit,
    Restart,
    Help,
}

pub trait PollInput {
    fn poll_input(&mut self, duration: Duration) -> io::Result<Input>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Stdin;

impl PollInput for Stdin {
    fn poll_input(&mut self, duration: Duration) -> io::Result<Input> {
        use Input::*;

        if termevent::poll(duration)? {
            match termevent::read()? {
                TermEvent::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    let input = match key_event.code {
                        KeyCode::Left => Left,
                        KeyCode::Right => Right,
                        KeyCode::Down => Down,
                        KeyCode::Char('q') | KeyCode::Char('Q') => Quit,
                        KeyCode::Char('z') | KeyCode::Char('Z') => RotateLeft,
                        KeyCode::Char('x') | KeyCode::Char('X') => RotateRight,
                        KeyCode::Char('r') | KeyCode::Char('R') => Restart,
                        _ => None,
                    };
                    Ok(input)
                }
                _ => Ok(None),
            }
        } else {
            Ok(None)
        }
    }
}
