use std::time::{Duration, Instant};

/// A game tick, where each field is a flag representing whether the correponding event
/// should be triggered on that tick.
#[derive(Debug, Copy, Clone, Default)]
pub struct Tick {
    pub gravity: bool,
}

/// Ticks at a constant rate, returning the events that should be triggered on each tick. Must be
/// manually updated in a loop in order to accumulate progress towards the next tick.
#[derive(Debug, Clone)]
pub struct GameTimer {
    interval_timer: IntervalTimer,

    // The number of ticks after which gravity should be applied.
    gravity_ticks: u64,

    // The total number of ticks elapsed.
    tick_count: u64,
}

impl GameTimer {
    /// Instantiates a new [GameTimer] that ticks every `tick_interval` and applies gravity every
    /// `gravity_ticks`.
    pub fn new(tick_interval: Duration, gravity_ticks: u64) -> Self {
        Self {
            interval_timer: IntervalTimer::new(tick_interval),
            tick_count: 0,
            gravity_ticks,
        }
    }

    /// Sets the number of ticks required to trigger gravity events.
    pub fn set_gravity_ticks(&mut self, ticks: u64) {
        self.gravity_ticks = ticks
    }

    /// Update triggers the timer to evaluate how much progress has been made towards the next tick
    /// since the last.
    /// 
    /// Returns [None] if insufficient time has elapsed for the timer to tick.
    /// 
    /// Returns [Some(Tick)] if the timer has ticked, with the fields of [Tick] indicating which
    /// events are scheduled to occur on that tick.
    pub fn update(&mut self) -> Option<Tick> {
        let ticked = self.interval_timer.update();
        if !ticked {
            return None;
        }

        let mut tick = Tick::default();

        // Ticks at the boundary of a u64 will be imperfect... but that's not going to happen.
        self.tick_count = self.tick_count.wrapping_add(1);
        if self.tick_count.is_multiple_of(self.gravity_ticks) {
            tick.gravity = true
        }

        Some(tick)
    }

    /// Returns the remaining duration until the next tick.
    pub fn time_until_next_tick(&self) -> Duration {
        self.interval_timer
            .next_tick_at
            .saturating_duration_since(Instant::now())
    }
}

/// Ticks at a constant rate specified at instantiation. The timer must be manually updated in a
/// loop in order to accumulate progress towards the next tick.
#[derive(Debug, Clone)]
struct IntervalTimer {
    // The interval between ticks.
    tick_interval: Duration,

    // The instant at which timer was last updated.
    last_update: Instant,

    // The accumulated time since the last tick.
    time_since_last_tick: Duration,

    next_tick_at: Instant,
}

impl IntervalTimer {
    /// Returns a new timer, which start ticking immediately.
    fn new(tick_interval: Duration) -> Self {
        let now = Instant::now();
        Self {
            tick_interval,
            last_update: now,
            time_since_last_tick: Duration::default(),
            next_tick_at: now + tick_interval,
        }
    }

    /// Updates the timer's state with the time accumulated since the last tick. Returns true if a
    /// tick is triggered by the update.
    fn update(&mut self) -> bool {
        let now = Instant::now();
        let delta = now - self.last_update;
        self.last_update = now;

        self.time_since_last_tick += delta;
        let ticked = self.time_since_last_tick >= self.tick_interval;
        while self.time_since_last_tick >= self.tick_interval {
            self.time_since_last_tick -= self.tick_interval
        }

        self.next_tick_at = now + self.tick_interval - self.time_since_last_tick;

        ticked
    }
}
