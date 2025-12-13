use std::fmt;

#[derive(Clone, Copy)]
pub struct Orientation(&'static [&'static [u8]]);

impl fmt::Debug for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.0 {
            for (i, val) in row.iter().enumerate() {
                if i == row.len() - 1 {
                    writeln!(f, "{}", val)?
                } else {
                    write!(f, "{} ", val)?
                }
            }
        }

        fmt::Result::Ok(())
    }
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}

#[rustfmt::skip]
const I_ORIENTATIONS: [Orientation; 4] = [
    Orientation(&[
        &[0, 0, 0, 0],
        &[1, 1, 1, 1],
        &[0, 0, 0, 0],
        &[0, 0, 0, 0],
    ]),
    Orientation(&[
        &[0, 0, 1, 0],
        &[0, 0, 1, 0],
        &[0, 0, 1, 0],
        &[0, 0, 1, 0],
    ]),
    Orientation(&[
        &[0, 0, 0, 0],
        &[0, 0, 0, 0],
        &[1, 1, 1, 1],
        &[0, 0, 0, 0],
    ]),
    Orientation(&[
        &[0, 1, 0, 0],
        &[0, 1, 0, 0],
        &[0, 1, 0, 0],
        &[0, 1, 0, 0],
    ]),
];

#[rustfmt::skip]
const O_ORIENTATION: Orientation = Orientation(&[
    &[1, 1],
    &[1, 1],
]);

pub trait Block {
    fn orientation(&self) -> Orientation;
    fn rotate(&mut self);
}

#[derive(Debug)]
pub enum Blocks {
    I { orientation_idx: usize },
    O,
}

use Blocks::*;

impl Block for Blocks {
    fn orientation(&self) -> Orientation {
        match self {
            I { orientation_idx: i } => I_ORIENTATIONS[*i],
            O => O_ORIENTATION,
        }
    }

    fn rotate(&mut self) {
        match self {
            I { orientation_idx: i } => {
                *self = I {
                    orientation_idx: (*i + 1) % I_ORIENTATIONS.len(),
                }
            }
            O => {}
        }
    }
}

impl fmt::Display for Blocks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.orientation())
    }
}
