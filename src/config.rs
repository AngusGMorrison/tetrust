use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
/// Gravity configuration.
pub struct Gravity {
    /// Initial ticks between applications of gravity.
    initial_ticks: u64,
    /// The minimum allowable ticks between applications of gravity.
    min_ticks: u64,
    /// The amount by which gravity is reduced when the associated score threshold is passed.
    acceleration: u64,
}

impl Gravity {
    pub fn new(initial_ticks: u64, min_ticks: u64, acceleration: u64) -> Result<Self, String> {
        if initial_ticks < min_ticks {
            return Err(format!(
                "initial_ticks cannot be less than min_ticks: initial_ticks={initial_ticks}, min_ticks={min_ticks}"
            ));
        }

        if acceleration > initial_ticks {
            return Err(format!(
                "acceleration cannot be greater than initial_ticks: acceleration={acceleration}, initial_ticks={initial_ticks}"
            ));
        }

        Ok(Self {
            initial_ticks,
            min_ticks,
            acceleration,
        })
    }

    pub fn initial_ticks(&self) -> u64 {
        self.initial_ticks
    }

    pub fn min_ticks(&self) -> u64 {
        self.min_ticks
    }

    pub fn acceleration(&self) -> u64 {
        self.acceleration
    }
}

/// Game configuration.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Config {
    /// The interval between game updates.
    pub frame_interval: Duration,

    /// Gravity config.
    pub gravity: Gravity,

    /// The number of points that must accumulate before gravity is increased.
    pub accelerate_every_n_points: u32,

    /// The number of game ticks that must elapse between input reads.
    pub input_ticks: u64,
}

#[cfg(test)]
mod gravity_tests {
    use super::*;

    mod new_tests {
        use super::*;

        #[test]
        fn when_initial_ticks_lt_min_ticks_returns_err() {
            let res = Gravity::new(0, 1, 1);
            assert!(res.is_err())
        }

        #[test]
        fn when_acceleration_gt_initial_ticks_returns_err() {
            let res = Gravity::new(0, 0, 1);
            assert!(res.is_err())
        }

        #[test]
        fn when_params_are_in_bounds_returns_ok() {
            let res = Gravity::new(1, 1, 1);
            let expected = Ok(Gravity {
                initial_ticks: 1,
                min_ticks: 1,
                acceleration: 1,
            });

            assert_eq!(res, expected)
        }
    }
}
