use crate::{critical, modules::log::Log};
use std::ops;

pub enum Units {
    Meter,
    Kilogram,
    Second,
    Ampere,
    Kelvin,
    Mole,
    Candela,
}
impl Units {
    pub fn collection() -> [Units; 7] {
        [
            Units::Meter,
            Units::Kilogram,
            Units::Second,
            Units::Ampere,
            Units::Kelvin,
            Units::Mole,
            Units::Candela,
        ]
    }
    pub fn to_string(&self) -> String {
        match &self {
            Units::Meter => "m",
            Units::Kilogram => "kg",
            Units::Second => "s",
            Units::Ampere => "A",
            Units::Kelvin => "K",
            Units::Mole => "mol",
            Units::Candela => "cd",
        }
        .to_string()
    }
}

pub fn generate_powers_from_unit_str(unit_str: String) -> [f64; 7] {
    let mut powers = vec![0.0; 7];

    for (i, item) in Units::collection().iter().enumerate() {
        let item_str = item.to_string();
        let possibly_index = unit_str.find(&item_str);
        if let None = possibly_index {
            continue;
        }
        let index = possibly_index.unwrap();
        let carrot_index = index + item_str.len();
        let mut chars = unit_str.chars();
        let possibly_carrot = chars.nth(carrot_index);
        if let None = possibly_carrot {
            powers[i] = 1.0;
            continue;
        }
        let carrot = possibly_carrot.unwrap();
        if carrot != '^' {
            powers[i] = 1.0;
            continue;
        }
        let mut digits = String::new();
        let mut sign = 1.0;
        for digit in chars {
            if digit == '-' {
                sign = -1.0;
                continue;
            }
            if !digit.is_digit(10) {
                break;
            }
            digits += digit.to_string().as_str();
        }
        let possibly_power = digits.parse::<f64>();
        match possibly_power {
            Err(_) => powers[i] = 1.0,
            Ok(power) => powers[i] = sign * power,
        }
    }

    powers.try_into().unwrap_or_else(|_| {
        critical!("Impossible");
    })
}

#[derive(Debug, Clone, Copy)]
pub struct Unit {
    pub value: f64,
    pub powers: [f64; 7],
}
impl Unit {
    pub fn new(value: f64, powers: [f64; 7]) -> Self {
        Self { value, powers }
    }
    pub fn new_int(value: i64, powers: [u8; 7]) -> Self {
        Self::new_from_vec(
            value as f64,
            powers.to_vec().into_iter().map(|x| x as f64).collect(),
        )
    }
    pub fn new_from_vec(value: f64, powers: Vec<f64>) -> Self {
        Self {
            value,
            powers: powers.try_into().unwrap_or_else(|v: Vec<f64>| {
                critical!(
                    "Invalid power length, expected length to by 7, found {}",
                    v.len()
                );
            }),
        }
    }
}
impl ops::Add<Unit> for Unit {
    type Output = Unit;
    fn add(self, rhs: Unit) -> Self::Output {
        if self.powers != rhs.powers {
            critical!(
                "Cannot add 2 values with different units\nLHS units: {:?}\nRHS units: {:?}",
                self.powers,
                rhs.powers
            );
        }
        Unit::new(self.value + rhs.value, self.powers)
    }
}
impl ops::Sub<Unit> for Unit {
    type Output = Unit;
    fn sub(self, rhs: Unit) -> Self::Output {
        if self.powers != rhs.powers {
            critical!(
                "Cannot subtract 2 values with different units\nLHS units: {:?}\nRHS units: {:?}",
                self.powers,
                rhs.powers
            );
        }
        Unit::new(self.value - rhs.value, self.powers)
    }
}
impl ops::Mul<Unit> for Unit {
    type Output = Unit;
    fn mul(self, rhs: Unit) -> Self::Output {
        let mut i = 0;
        let power_sum: Vec<f64> = self
            .powers
            .into_iter()
            .map(|power| {
                i += 1;
                power + rhs.powers[i - 1]
            })
            .collect();
        Unit::new_from_vec(self.value * rhs.value, power_sum)
    }
}
impl ops::Div<Unit> for Unit {
    type Output = Unit;
    fn div(self, rhs: Unit) -> Self::Output {
        let mut i = 0;
        let power_diff: Vec<f64> = self
            .powers
            .into_iter()
            .map(|power| {
                i += 1;
                power - rhs.powers[i - 1]
            })
            .collect();
        Unit::new_from_vec(self.value * rhs.value, power_diff)
    }
}
