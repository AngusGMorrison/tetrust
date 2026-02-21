use std::{
    io::{self},
    thread,
    time::Duration,
};

use tetrust::{
    block::BlockGenerator,
    game::{Event, GameState},
    timer::GameTimer,
};

/// The number of ticks that must elapse between applications of gravity.
const INITIAL_GRAVITY_TICKS: u64 = 48;

fn main() {
    let block_generator = BlockGenerator::new(rand::rng());
    let mut state = GameState::new(block_generator);
    let frame_interval = Duration::from_secs_f32(1.0 / 60.0);
    let mut timer = GameTimer::new(frame_interval, INITIAL_GRAVITY_TICKS);

    loop {
        let Some(tick) = timer.update() else { continue };
        if tick.gravity {
            println!("{}", state);
            println!("Press any key to continue...");
            let mut buf = String::new();
            io::stdin()
                .read_line(&mut buf)
                .expect("Failed to read from stdin");
            state.update(Event::Gravity);
        }

        if state.game_over() {
            break;
        }

        thread::sleep(timer.time_until_next_tick())
    }
}
