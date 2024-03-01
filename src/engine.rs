use crate::{registry::Registry, world::World};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ESEngine {
    pub world: World,
    pub registry: Registry,
}
impl ESEngine {
    pub fn new() -> Self {
        Self {
            world: World::new(0),
            registry: Registry::new(),
        }
    }
}
