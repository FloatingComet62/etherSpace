use std::ops;

use crate::{critical, modules::log::Log};

pub enum Units {
    Meter,
    Kilogram,
    Second,
    Ampere,
    Kelvin,
    Mole,
    Candela,
}

pub struct Unit {
    pub value: f64,
    pub powers: [f64; 7],
}
impl Unit {
    pub fn new(value: f64, powers: [f64; 7]) -> Self {
        Self { value, powers }
    }
}

impl ops::Add<Unit> for Unit {
    type Output = Unit;
    fn add(self, rhs: Unit) -> Self::Output {
        if self.powers != rhs.powers {
            critical!("Cannot add 2 values with different units");
        }
        // TODO: deci + kilo should work
        let mut net_power_diff = 0.0;
        for i in 0..7 {
            net_power_diff = self.powers[i] - rhs.powers[i];
        }
        Unit::new(
            self.value + rhs.value * f64::powf(10.0, net_power_diff),
            self.powers,
        )
    }
}

impl ops::Sub<Unit> for Unit {
    type Output = Unit;
    fn sub(self, rhs: Unit) -> Self::Output {
        if self.powers != rhs.powers {
            critical!("Cannot add 2 values with different units");
        }
        // TODO: deci + kilo should work
        let mut net_power_diff = 0.0;
        for i in 0..7 {
            net_power_diff = self.powers[i] - rhs.powers[i];
        }
        Unit::new(
            self.value - rhs.value * f64::powf(10.0, net_power_diff),
            self.powers,
        )
    }
}

impl ops::Mul<Unit> for Unit {
    type Output = Unit;
    fn mul(self, rhs: Unit) -> Self::Output {
        if self.powers != rhs.powers {
            critical!("Cannot add 2 values with different units");
        }
        // TODO: deci + kilo should work
        let mut power_sum: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        for i in 0..7 {
            power_sum[i] = self.powers[i] + rhs.powers[i];
        }
        Unit::new(self.value * rhs.value, power_sum)
    }
}

impl ops::Div<Unit> for Unit {
    type Output = Unit;
    fn div(self, rhs: Unit) -> Self::Output {
        if self.powers != rhs.powers {
            critical!("Cannot add 2 values with different units");
        }
        // TODO: deci + kilo should work
        let mut power_diff: [f64; 7] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        for i in 0..7 {
            power_diff[i] = self.powers[i] - rhs.powers[i];
        }
        Unit::new(self.value / rhs.value, power_diff)
    }
}
