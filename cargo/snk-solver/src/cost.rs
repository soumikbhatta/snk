use std::ops::{Add, AddAssign, Mul};

use snk_grid::color::Color;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cost(pub u64);

impl Add for Cost {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}
impl AddAssign for Cost {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}
impl Mul<u64> for Cost {
    type Output = Self;

    fn mul(self, rhs: u64) -> Self {
        Self(self.0 * rhs)
    }
}

impl From<Color> for Cost {
    fn from(color: Color) -> Self {
        match color {
            Color::Empty => Cost(1),
            Color::Color1 => Cost(256),
            Color::Color2 => Cost(256 * 200),
            Color::Color3 => Cost(256 * 200 * 200),
            Color::Color4 => Cost(256 * 200 * 200 * 200),
        }
    }
}

impl Cost {
    pub fn zero() -> Self {
        Self(0)
    }
    pub fn max() -> Self {
        Self(u64::MAX)
    }
    pub fn is_free(&self) -> bool {
        self.0 < 256
    }
}

#[test]
fn it_should_not_overflow() {
    // it should not panic
    let very_large_cost = Cost::from(Color::Color4) * 256;
    assert!(very_large_cost < Cost::max())
}

#[test]
fn it_should_sum_cost() {
    let mut c = Cost::zero();
    c = c + Color::Color1.into();
    assert!(Cost::zero() < c);
}
