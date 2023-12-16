#[derive(Clone)]
pub struct Vector2<T>
where
    T: Default,
{
    pub x: T,
    pub y: T,
}

impl<T: Default> Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn default() -> Self {
        Self {
            x: T::default(),
            y: T::default(),
        }
    }
}
