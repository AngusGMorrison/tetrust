use std::time::{Duration, Instant};

/// A game tick, where each field is a flag representing whether the correponding event
/// should be triggered on that tick.
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct Tick {
    pub gravity: bool,
    pub input: bool,
}

impl Tick {
    pub fn any(&self) -> bool {
        self.gravity || self.input
    }
}

#[cfg(test)]
mod any_tests {
    use super::*;

    #[test]
    fn when_gravity_and_input_are_false_returns_false() {
        assert!(
            !Tick {
                gravity: false,
                input: false
            }
            .any()
        );
    }

    #[test]
    fn when_gravity_is_true_and_input_is_false_returns_true() {
        assert!(
            Tick {
                gravity: true,
                input: false
            }
            .any()
        );
    }

    #[test]
    fn when_gravity_is_false_and_input_is_true_returns_true() {
        assert!(
            Tick {
                gravity: false,
                input: true
            }
            .any()
        );
    }

    #[test]
    fn when_gravity_and_input_are_true_returns_true() {
        assert!(
            Tick {
                gravity: true,
                input: true
            }
            .any()
        );
    }
}

pub trait Clock {
    fn now(&self) -> Instant;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn now(&self) -> Instant {
        Instant::now()
    }
}

/// Ticks at a constant rate, returning the events that should be triggered on each tick. Must be
/// manually updated in a loop in order to accumulate progress towards the next tick.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameTimer<C = SystemClock> {
    interval_timer: IntervalTimer<C>,

    // The number of ticks after which gravity should be applied.
    gravity_ticks: u64,

    // The number of ticks after which user input should be read.
    input_ticks: u64,

    // The total number of ticks elapsed.
    tick_count: u64,
}

impl GameTimer<SystemClock> {
    /// Instantiates a new [GameTimer] that advances when polled, returning a [Tick]
    /// each time a tick interval has elapsed since the last poll.
    #[cfg(test)]
    pub fn new(tick_interval: Duration, gravity_ticks: u64, input_ticks: u64) -> Self {
        Self::new_with_clock(tick_interval, gravity_ticks, input_ticks, SystemClock)
    }
}

impl<C: Clock> GameTimer<C> {
    pub(crate) fn new_with_clock(
        tick_interval: Duration,
        gravity_ticks: u64,
        input_ticks: u64,
        clock: C,
    ) -> Self {
        Self {
            interval_timer: IntervalTimer::new(tick_interval, clock),
            tick_count: 0,
            gravity_ticks,
            input_ticks,
        }
    }

    pub fn gravity_ticks(&self) -> u64 {
        self.gravity_ticks
    }

    /// Sets the number of ticks required to trigger gravity events.
    pub fn set_gravity_ticks(&mut self, ticks: u64) {
        self.gravity_ticks = ticks;
    }

    /// Update triggers the timer to evaluate how much progress has been made towards the next tick
    /// since the last.
    ///
    /// Returns [None] if insufficient time has elapsed for the timer to tick.
    ///
    /// Returns [Some(Tick)] if the timer has ticked, with the fields of [Tick] indicating which
    /// events are scheduled to occur on that tick.
    pub fn update(&mut self) -> Option<Tick> {
        if self.interval_timer.update() {
            // Ticks at the boundary of a u64 will be imperfect... but that's not going to happen.
            self.tick_count = self.tick_count.wrapping_add(1);
            Some(self.last_tick())
        } else {
            None
        }
    }

    /// Returns the remaining duration until the next tick.
    pub fn time_until_next_tick(&self) -> Duration {
        self.interval_timer
            .next_tick_at
            .saturating_duration_since(self.interval_timer.now())
    }

    /// Returns the most recent tick.
    fn last_tick(&self) -> Tick {
        Tick {
            gravity: self.tick_count.is_multiple_of(self.gravity_ticks),
            input: self.tick_count.is_multiple_of(self.input_ticks),
        }
    }
}

#[cfg(test)]
pub(crate) mod test_helpers {
    use std::cell::Cell;
    use std::rc::Rc;

    use super::*;

    #[derive(Debug, Clone)]
    pub(crate) struct MockClock(Rc<Cell<Instant>>);

    impl MockClock {
        pub(crate) fn new(now: Instant) -> Self {
            Self(Rc::new(Cell::new(now)))
        }

        pub(crate) fn advance(&self, d: Duration) {
            self.0.set(self.0.get() + d);
        }
    }

    impl PartialEq for MockClock {
        fn eq(&self, other: &Self) -> bool {
            self.0.get() == other.0.get()
        }
    }

    impl Eq for MockClock {}

    impl Clock for MockClock {
        fn now(&self) -> Instant {
            self.0.get()
        }
    }
}

#[cfg(test)]
mod game_timer_tests {
    use super::test_helpers::MockClock;
    use super::*;

    mod new_with_clock_tests {
        use super::*;

        #[test]
        fn initializes_fields() {
            let now = Instant::now();
            let interval = Duration::from_millis(100);

            let actual = GameTimer::new_with_clock(interval, 5, 1, MockClock::new(now));

            let expected = GameTimer {
                interval_timer: IntervalTimer::new(interval, MockClock::new(now)),
                gravity_ticks: 5,
                input_ticks: 1,
                tick_count: 0,
            };

            assert_eq!(actual, expected);
        }
    }

    mod time_until_next_tick_tests {
        use super::*;

        const INTERVAL: Duration = Duration::from_millis(100);

        #[test]
        fn when_next_tick_is_in_the_future_returns_remaining_duration() {
            let now = Instant::now();
            let clock = MockClock::new(now);
            let timer = GameTimer::new_with_clock(INTERVAL, 1, 1, clock.clone());
            clock.advance(Duration::from_millis(40));
            assert_eq!(timer.time_until_next_tick(), Duration::from_millis(60));
        }

        #[test]
        fn when_next_tick_is_overdue_returns_zero() {
            let now = Instant::now();
            let clock = MockClock::new(now);
            let timer = GameTimer::new_with_clock(INTERVAL, 1, 1, clock.clone());
            clock.advance(INTERVAL + Duration::from_millis(1));
            assert_eq!(timer.time_until_next_tick(), Duration::ZERO);
        }
    }

    mod last_tick_tests {
        use super::*;

        fn timer_with_tick_count(
            gravity_ticks: u64,
            input_ticks: u64,
            tick_count: u64,
        ) -> GameTimer<MockClock> {
            GameTimer {
                interval_timer: IntervalTimer::new(
                    Duration::from_millis(100),
                    MockClock::new(Instant::now()),
                ),
                gravity_ticks,
                input_ticks,
                tick_count,
            }
        }

        #[test]
        fn when_tick_count_is_multiple_of_both_both_flags_are_true() {
            let timer = timer_with_tick_count(2, 3, 6);
            assert_eq!(
                timer.last_tick(),
                Tick {
                    gravity: true,
                    input: true
                }
            );
        }

        #[test]
        fn when_tick_count_is_multiple_of_gravity_ticks_only_gravity_is_true() {
            let timer = timer_with_tick_count(2, 3, 4);
            assert_eq!(
                timer.last_tick(),
                Tick {
                    gravity: true,
                    input: false
                }
            );
        }

        #[test]
        fn when_tick_count_is_multiple_of_input_ticks_only_input_is_true() {
            let timer = timer_with_tick_count(2, 3, 3);
            assert_eq!(
                timer.last_tick(),
                Tick {
                    gravity: false,
                    input: true
                }
            );
        }

        #[test]
        fn when_tick_count_is_multiple_of_neither_both_flags_are_false() {
            let timer = timer_with_tick_count(2, 3, 5);
            assert_eq!(
                timer.last_tick(),
                Tick {
                    gravity: false,
                    input: false
                }
            );
        }
    }

    mod update_tests {
        use super::*;

        const INTERVAL: Duration = Duration::from_millis(100);

        #[test]
        fn when_interval_has_not_elapsed_returns_none() {
            let clock = MockClock::new(Instant::now());
            let mut timer = GameTimer::new_with_clock(INTERVAL, 1, 1, clock.clone());
            clock.advance(INTERVAL - Duration::from_millis(1));
            assert_eq!(timer.update(), None);
        }

        #[test]
        fn when_interval_has_elapsed_increments_tick_count_and_returns_some_tick() {
            let clock = MockClock::new(Instant::now());
            let mut timer = GameTimer::new_with_clock(INTERVAL, 1, 1, clock.clone());
            clock.advance(INTERVAL);
            assert_eq!(
                timer.update(),
                Some(Tick {
                    gravity: true,
                    input: true
                })
            );
            assert_eq!(timer.tick_count, 1);
        }
    }

    mod set_gravity_ticks_tests {
        use super::*;

        #[test]
        fn when_called_updates_gravity_ticks() {
            let mut timer = GameTimer::new(Duration::from_millis(100), 5, 1);
            timer.set_gravity_ticks(10);
            assert_eq!(timer.gravity_ticks(), 10);
        }
    }
}

/// Ticks at a constant rate specified at instantiation. The timer must be manually updated in a
/// loop in order to accumulate progress towards the next tick.
#[derive(Debug, Clone, PartialEq, Eq)]
struct IntervalTimer<C> {
    clock: C,

    // The interval between ticks.
    tick_interval: Duration,

    // The instant at which timer was last updated.
    last_update: Instant,

    // The accumulated time since the last tick.
    time_since_last_tick: Duration,

    next_tick_at: Instant,
}

impl<C: Clock> IntervalTimer<C> {
    /// Returns a new timer, which start ticking immediately.
    fn new(tick_interval: Duration, clock: C) -> Self {
        let now = clock.now();
        Self {
            clock,
            tick_interval,
            last_update: now,
            time_since_last_tick: Duration::default(),
            next_tick_at: now + tick_interval,
        }
    }

    fn now(&self) -> Instant {
        self.clock.now()
    }

    /// Updates the timer's state with the time accumulated since the last tick. Returns true if a
    /// tick is triggered by the update.
    fn update(&mut self) -> bool {
        let now = self.clock.now();
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

#[cfg(test)]
mod interval_timer_tests {
    use super::test_helpers::MockClock;
    use super::*;

    mod new_tests {
        use super::*;

        #[test]
        fn initializes_last_update_to_now() {
            let now = Instant::now();
            let timer = IntervalTimer::new(Duration::from_millis(100), MockClock::new(now));
            assert_eq!(timer.last_update, now);
        }

        #[test]
        fn initializes_time_since_last_tick_to_zero() {
            let timer =
                IntervalTimer::new(Duration::from_millis(100), MockClock::new(Instant::now()));
            assert_eq!(timer.time_since_last_tick, Duration::ZERO);
        }

        #[test]
        fn initializes_next_tick_at_to_now_plus_interval() {
            let now = Instant::now();
            let interval = Duration::from_millis(100);
            let timer = IntervalTimer::new(interval, MockClock::new(now));
            assert_eq!(timer.next_tick_at, now + interval);
        }
    }

    mod update_tests {
        use super::*;

        const INTERVAL: Duration = Duration::from_millis(100);

        fn timer_at(now: Instant) -> IntervalTimer<MockClock> {
            IntervalTimer::new(INTERVAL, MockClock::new(now))
        }

        #[test]
        fn when_elapsed_time_is_less_than_interval_returns_false() {
            let mut timer = timer_at(Instant::now());
            timer.clock.advance(INTERVAL - Duration::from_millis(1));
            assert!(!timer.update());
        }

        #[test]
        fn when_elapsed_time_equals_interval_returns_true() {
            let mut timer = timer_at(Instant::now());
            timer.clock.advance(INTERVAL);
            assert!(timer.update());
        }

        #[test]
        fn when_elapsed_time_exceeds_interval_returns_true() {
            let mut timer = timer_at(Instant::now());
            timer.clock.advance(INTERVAL + Duration::from_millis(1));
            assert!(timer.update());
        }

        #[test]
        fn when_elapsed_time_spans_multiple_intervals_returns_true() {
            let mut timer = timer_at(Instant::now());
            timer
                .clock
                .advance(INTERVAL * 2 + Duration::from_millis(30));
            assert!(timer.update());
        }

        #[test]
        fn always_sets_last_update_to_now() {
            let mut timer = timer_at(Instant::now());
            timer.clock.advance(Duration::from_millis(40));
            let now = timer.clock.now();
            timer.update();
            assert_eq!(timer.last_update, now);
        }

        #[test]
        fn when_no_tick_accumulates_elapsed_time_in_time_since_last_tick() {
            let mut timer = timer_at(Instant::now());
            let elapsed = INTERVAL - Duration::from_millis(1);
            timer.clock.advance(elapsed);
            timer.update();
            assert_eq!(timer.time_since_last_tick, elapsed);
        }

        #[test]
        fn when_tick_sets_time_since_last_tick_to_remainder_after_draining_full_intervals() {
            let mut timer = timer_at(Instant::now());
            let remainder = Duration::from_millis(30);
            timer.clock.advance(INTERVAL + remainder);
            timer.update();
            assert_eq!(timer.time_since_last_tick, remainder);
        }

        #[test]
        fn when_no_tick_sets_next_tick_at_to_now_plus_remaining_interval() {
            let mut timer = timer_at(Instant::now());
            let elapsed = Duration::from_millis(40);
            timer.clock.advance(elapsed);
            let now = timer.clock.now();
            timer.update();
            assert_eq!(timer.next_tick_at, now + INTERVAL - elapsed);
        }

        #[test]
        fn when_elapsed_time_spans_multiple_intervals_sets_time_since_last_tick_to_remainder() {
            let mut timer = timer_at(Instant::now());
            let remainder = Duration::from_millis(30);
            timer.clock.advance(INTERVAL * 2 + remainder);
            timer.update();
            assert_eq!(timer.time_since_last_tick, remainder);
        }
    }
}
