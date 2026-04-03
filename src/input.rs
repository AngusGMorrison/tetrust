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
        if termevent::poll(duration)? {
            Ok(translate(termevent::read()?))
        } else {
            Ok(Input::None)
        }
    }
}

fn translate(event: TermEvent) -> Input {
    use Input::*;
    match event {
        TermEvent::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            match key_event.code {
                KeyCode::Left => Left,
                KeyCode::Right => Right,
                KeyCode::Down => Down,
                KeyCode::Char('q') | KeyCode::Char('Q') => Quit,
                KeyCode::Char('z') | KeyCode::Char('Z') => RotateLeft,
                KeyCode::Char('x') | KeyCode::Char('X') => RotateRight,
                KeyCode::Char('r') | KeyCode::Char('R') => Restart,
                _ => None,
            }
        }
        _ => None,
    }
}

#[cfg(test)]
mod translate_tests {
    use super::*;
    use crossterm::event::{KeyEvent, KeyEventKind, KeyModifiers};

    fn press(code: KeyCode) -> TermEvent {
        TermEvent::Key(KeyEvent::new(code, KeyModifiers::empty()))
    }

    fn release(code: KeyCode) -> TermEvent {
        TermEvent::Key(KeyEvent::new_with_kind(
            code,
            KeyModifiers::empty(),
            KeyEventKind::Release,
        ))
    }

    #[test]
    fn when_left_key_pressed_returns_left() {
        assert_eq!(translate(press(KeyCode::Left)), Input::Left);
    }

    #[test]
    fn when_right_key_pressed_returns_right() {
        assert_eq!(translate(press(KeyCode::Right)), Input::Right);
    }

    #[test]
    fn when_down_key_pressed_returns_down() {
        assert_eq!(translate(press(KeyCode::Down)), Input::Down);
    }

    #[test]
    fn when_q_pressed_returns_quit() {
        assert_eq!(translate(press(KeyCode::Char('q'))), Input::Quit);
    }

    #[test]
    fn when_uppercase_q_pressed_returns_quit() {
        assert_eq!(translate(press(KeyCode::Char('Q'))), Input::Quit);
    }

    #[test]
    fn when_z_pressed_returns_rotate_left() {
        assert_eq!(translate(press(KeyCode::Char('z'))), Input::RotateLeft);
    }

    #[test]
    fn when_uppercase_z_pressed_returns_rotate_left() {
        assert_eq!(translate(press(KeyCode::Char('Z'))), Input::RotateLeft);
    }

    #[test]
    fn when_x_pressed_returns_rotate_right() {
        assert_eq!(translate(press(KeyCode::Char('x'))), Input::RotateRight);
    }

    #[test]
    fn when_uppercase_x_pressed_returns_rotate_right() {
        assert_eq!(translate(press(KeyCode::Char('X'))), Input::RotateRight);
    }

    #[test]
    fn when_r_pressed_returns_restart() {
        assert_eq!(translate(press(KeyCode::Char('r'))), Input::Restart);
    }

    #[test]
    fn when_uppercase_r_pressed_returns_restart() {
        assert_eq!(translate(press(KeyCode::Char('R'))), Input::Restart);
    }

    #[test]
    fn when_unmapped_key_pressed_returns_none() {
        assert_eq!(translate(press(KeyCode::F(1))), Input::None);
    }

    #[test]
    fn when_key_is_released_returns_none() {
        assert_eq!(translate(release(KeyCode::Left)), Input::None);
    }

    #[test]
    fn when_event_is_not_a_key_event_returns_none() {
        assert_eq!(translate(TermEvent::FocusGained), Input::None);
    }
}
