use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};

// DO I REALLY NEED A GENERIC VECTOR???

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}

impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl Default for Vector2 {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}
impl Add for Vector2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x + rhs.x, self.y + rhs.y)
    }
}
impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl Sub for Vector2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector2::new(self.x - rhs.x, self.y - rhs.y)
    }
}
impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

pub fn dot(v1: &Vector2, v2: &Vector2) -> f64 {
    v1.x * v2.x + v1.y * v2.y
}

#[cfg(test)]
mod test {
    use crate::modules::vector::{dot, Vector2};

    #[test]
    fn dot_product() {
        let a = Vector2::new(12.0, 5.0);
        let b = Vector2::new(12.0, 5.0);
        assert_eq!(dot(&a, &b), 169.0);
    }
}
