use core::str::FromStr;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Vector2<T>
where
    T: Default + ToString + FromStr,
{
    pub x: T,
    pub y: T,
}

impl<T: Default + std::fmt::Display + FromStr> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Default + std::fmt::Display + FromStr> Default for Vector2<T> {
    fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}

pub fn dot(v1: &Vector2<f64>, v2: &Vector2<f64>) -> f64 {
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
