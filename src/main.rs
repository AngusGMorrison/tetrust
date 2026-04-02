use std::{thread, time::Duration};

use tetrust::{
    block_generator::BlockGenerator, config::{Config, Gravity}, game::{Game, UpdateOutcome}, input::Stdin
};

/// The number of ticks that must elapse between applications of gravity.
const INITIAL_GRAVITY_TICKS: u64 = 48;

const MIN_GRAVITY_TICKS: u64 = 12;

const ACCELERATION: u64 = 4;

const ACCELERATE_EVERY_N_POINTS: u32 = 5;

/// The number of ticks that must elapse between reads of user input.
const INPUT_TICKS: u64 = 1;

fn main() -> Result<(), String> {
    let block_generator = BlockGenerator::new(rand::rng());
    let frame_interval = Duration::from_secs_f32(1.0 / 60.0);
    let config = Config {
        gravity: Gravity::new(INITIAL_GRAVITY_TICKS, MIN_GRAVITY_TICKS, ACCELERATION)?,
        frame_interval,
        accelerate_every_n_points: ACCELERATE_EVERY_N_POINTS,
        input_ticks: INPUT_TICKS,
    };
    let mut game = Game::new(block_generator, Stdin, config);

    ratatui::run(|terminal| -> Result<(), String> {
        loop {
            match game.update().map_err(|e| e.to_string())? {
                UpdateOutcome::Updated => {
                    _ = terminal
                        .draw(|frame| frame.render_widget(&game, frame.area()))
                        .map_err(|e| e.to_string())?
                }
                UpdateOutcome::Quit => return Ok(()),
                _ => (),
            }

            thread::sleep(game.time_until_next_tick())
        }
    })
}
