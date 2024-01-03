use crate::{critical, modules::log::Log};
use core::fmt::Display;
use std::{collections::BTreeSet, ops};

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
    pub fn composite() -> Vec<(String, [f64; 7])> {
        vec![
            (
                "N".to_string(),
                generate_powers_from_unit_str("kgm/s^2".to_string()),
            ),
            (
                "C".to_string(),
                generate_powers_from_unit_str("As".to_string()),
            ),
            (
                "J".to_string(),
                generate_powers_from_unit_str("kgm^2/s^2".to_string()),
            ),
            (
                "L".to_string(),
                generate_powers_from_unit_str("m^3".to_string()),
            ),
            (
                "W".to_string(),
                generate_powers_from_unit_str("kgm^2/s^3".to_string()),
            ),
            (
                "Ω".to_string(),
                generate_powers_from_unit_str("kgm^2/s^3A^2".to_string()),
            ),
            (
                "V".to_string(),
                generate_powers_from_unit_str("kgm^2/s^3A".to_string()),
            ),
        ]
    }
}

pub fn generate_powers_from_unit_str_composite(units_str: String) -> ([f64; 7], BTreeSet<String>) {
    let mut powers = vec![0.0; 7];
    let mut composite_units_used = BTreeSet::new();
    let slash_index = units_str.find("/").unwrap_or(units_str.len());

    for (unit_str, composite_unit) in Units::composite().iter() {
        let possibly_index = units_str.find(unit_str.as_str());
        if let None = possibly_index {
            continue;
        }
        composite_units_used.insert(unit_str.clone());
        let index = possibly_index.unwrap();
        let fraction_coefficient = if slash_index > index { 1.0 } else { -1.0 };
        let carrot_index = index + unit_str.len();
        let mut chars = units_str.chars();
        let possibly_carrot = chars.nth(carrot_index);
        if let None = possibly_carrot {
            for i in 0..powers.len() {
                powers[i] += composite_unit[i] * fraction_coefficient;
            }
            continue;
        }
        let carrot = possibly_carrot.unwrap();
        if carrot != '^' {
            for i in 0..powers.len() {
                powers[i] += composite_unit[i] * fraction_coefficient;
            }
            continue;
        }
        let mut digits = String::new();
        let mut sign = 1.0;
        for digit in chars.filter(|digit| *digit != '.') {
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
            Err(_) => {
                for i in 0..powers.len() {
                    powers[i] += composite_unit[i] * fraction_coefficient;
                }
            }
            Ok(power) => {
                for i in 0..powers.len() {
                    powers[i] += composite_unit[i] * sign * power * fraction_coefficient;
                }
            }
        }
    }

    generate_powers_from_unit_str(units_str)
        .iter()
        .enumerate()
        .for_each(|(i, power)| powers[i] += *power);

    (
        powers.try_into().unwrap_or_else(|_| {
            critical!("Impossible");
        }),
        composite_units_used,
    )
}
fn generate_powers_from_unit_str(units_str: String) -> [f64; 7] {
    let mut powers = vec![0.0; 7];
    let slash_index = units_str.find("/").unwrap_or(units_str.len());

    for (i, unit) in Units::collection().iter().enumerate() {
        let unit_str = unit.to_string();
        let possibly_index = units_str.find(&unit_str);
        if let None = possibly_index {
            continue;
        }
        let index = possibly_index.unwrap();
        // allows stuff like "m/s" and "m/s^2"
        let fraction_coefficient = if slash_index > index { 1.0 } else { -1.0 };
        let carrot_index = index + unit_str.len();
        let mut chars = units_str.chars();
        let possibly_carrot = chars.nth(carrot_index);
        if let None = possibly_carrot {
            powers[i] = fraction_coefficient;
            continue;
        }
        let carrot = possibly_carrot.unwrap();
        if carrot != '^' {
            powers[i] = fraction_coefficient;
            continue;
        }
        let mut digits = String::new();
        let mut sign = 1.0;
        for digit in chars.filter(|digit| *digit != '.') {
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
            Err(_) => powers[i] = fraction_coefficient,
            Ok(power) => powers[i] = sign * power * fraction_coefficient,
        }
    }

    powers.try_into().unwrap_or_else(|_| {
        critical!("Impossible");
    })
}
fn generate_unit_str_from_powers_composite(data: ([f64; 7], BTreeSet<String>)) -> String {
    let powers = data.0;
    let mut numerator_str = String::new();
    let mut denominator_str = String::new();

    let mut powers_by_composite = vec![];
    let composites = Units::composite();
    for (unit_str, composite_unit) in composites
        .iter()
        .filter(|(unit_str, _)| data.1.contains(unit_str))
    {
        let (diff_numerator, diff_denominator) =
            composite_unit
                .iter()
                .enumerate()
                .fold((0.0, 0.0), |acc, (i, composite_unit_power)| {
                    (
                        acc.0 + (powers[i] - composite_unit_power).abs(),
                        acc.1 + (powers[i] + composite_unit_power).abs(),
                    )
                });
        if diff_numerator < diff_denominator {
            powers_by_composite.push((
                0,
                (0..powers.len())
                    .map(|i| powers[i] - composite_unit[i])
                    .collect::<Vec<f64>>(),
                unit_str,
            ));
        } else {
            powers_by_composite.push((
                1,
                (0..powers.len())
                    .map(|i| powers[i] + composite_unit[i])
                    .collect::<Vec<f64>>(),
                unit_str,
            ));
        }
    }

    // find the best usage
    let powers_by_composite_score: Vec<f64> = powers_by_composite
        .iter()
        .map(|powers_by_composite| {
            powers_by_composite
                .1
                .iter()
                .fold(0.0, |acc, x| acc + x.abs())
        })
        .collect();
    let mut best_index = -1;
    let mut best_value = powers.iter().fold(0.0, |acc, x| acc + x.abs());
    for (i, item) in powers_by_composite_score.iter().enumerate() {
        if *item <= best_value {
            best_index = i as isize;
            best_value = item.clone();
        }
    }
    if best_index != -1 {
        let best_unit = &powers_by_composite[best_index as usize];
        let new_powers = best_unit
            .1
            .clone()
            .try_into()
            .unwrap_or_else(|v: Vec<f64>| {
                critical!(
                    "Invalid power length, expected length to by 7, found {}",
                    v.len()
                );
            });
        let mut new_data = data.1.clone();
        if best_unit.0 == 0 {
            // unit goes in numerator
            numerator_str += &best_unit.2;
        } else {
            denominator_str += &best_unit.2;
        }
        new_data.insert(best_unit.2.clone());

        let normal_unit_str_data = generate_unit_str_from_powers_composite((new_powers, new_data));
        let normal_unit_str: Vec<&str> = normal_unit_str_data.split('/').collect();
        if normal_unit_str.len() > 1 {
            return format!(
                "{}{}/{}{}",
                numerator_str, normal_unit_str[0], denominator_str, normal_unit_str[1]
            );
        }
        if denominator_str.len() == 0 {
            return format!("{}{}", numerator_str, normal_unit_str[0]);
        }
        return format!(
            "{}{}/{}",
            numerator_str, normal_unit_str[0], denominator_str
        );
    }
    let normal_unit_str_data = generate_unit_str_from_powers(powers);
    let normal_unit_str: Vec<&str> = normal_unit_str_data.split('/').collect();
    if normal_unit_str.len() > 1 {
        return format!(
            "{}{}/{}{}",
            numerator_str, normal_unit_str[0], denominator_str, normal_unit_str[1]
        );
    }
    if denominator_str.len() == 0 {
        return format!("{}{}", numerator_str, normal_unit_str[0]);
    }
    format!(
        "{}{}/{}",
        numerator_str, normal_unit_str[0], denominator_str
    )
}
fn generate_unit_str_from_powers(powers: [f64; 7]) -> String {
    let mut unit_str = String::new();

    for (i, unit) in Units::collection()
        .iter()
        .enumerate()
        .filter(|(i, _)| powers[*i] > 0.0)
    {
        let power = powers[i];
        unit_str += &unit.to_string();
        if power == 1.0 {
            continue;
        }
        unit_str += &format!("^{}", power.to_string());
    }
    let mut first = true;
    for (i, unit) in Units::collection()
        .iter()
        .enumerate()
        .filter(|(i, _)| powers[*i] < 0.0)
    {
        let power = powers[i];
        if first {
            unit_str += "/";
            first = false;
        }
        unit_str += &unit.to_string();
        if powers[i] == -1.0 {
            continue;
        }
        unit_str += &format!("^{}", (-power).to_string());
    }

    unit_str
}

#[macro_export]
macro_rules! unit {
    ($value: expr, $unit_str: expr) => {
        Unit::new_composite(
            $value,
            generate_powers_from_unit_str_composite($unit_str.to_string()),
        )
    };
}

#[derive(Debug, Clone)]
pub struct Unit {
    pub value: f64,
    pub powers: [f64; 7],
    composites_used: BTreeSet<String>,
}
impl Unit {
    pub fn new(value: f64, powers: [f64; 7]) -> Self {
        let mut unit = Self {
            value,
            powers,
            composites_used: BTreeSet::new(),
        };
        unit.check_for_composite();
        unit
    }
    pub fn new_int(value: i64, powers: [u8; 7]) -> Self {
        Self::new_from_vec(
            value as f64,
            powers.to_vec().into_iter().map(|x| x as f64).collect(),
        )
    }
    pub fn new_from_vec(value: f64, powers: Vec<f64>) -> Self {
        let mut unit = Self {
            value,
            powers: powers.try_into().unwrap_or_else(|v: Vec<f64>| {
                critical!(
                    "Invalid power length, expected length to by 7, found {}",
                    v.len()
                );
            }),
            composites_used: BTreeSet::new(),
        };
        unit.check_for_composite();
        unit
    }
    pub fn new_composite(value: f64, data: ([f64; 7], BTreeSet<String>)) -> Self {
        let mut unit = Self {
            value,
            powers: data.0,
            composites_used: data.1,
        };
        unit.check_for_composite();
        unit
    }
    pub fn new_composite_from_vec(value: f64, data: (Vec<f64>, BTreeSet<String>)) -> Self {
        let mut unit = Self {
            value,
            powers: data.0.try_into().unwrap_or_else(|v: Vec<f64>| {
                critical!(
                    "Invalid power length, expected length to by 7, found {}",
                    v.len()
                );
            }),
            composites_used: data.1,
        };
        unit.check_for_composite();
        unit
    }
    pub fn check_for_composite(&mut self) {
        for (unit_str, composite_unit) in Units::composite().iter() {
            if composite_unit
                .iter()
                .enumerate()
                .all(|(i, power)| self.powers[i] == *power)
            {
                self.composites_used.insert(unit_str.clone());
            }
        }
    }
}
impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:.3} {}",
            self.value,
            generate_unit_str_from_powers_composite((self.powers, self.composites_used.clone()))
        ))
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
        let composites = self
            .composites_used
            .union(&rhs.composites_used)
            .cloned()
            .collect();
        Unit::new_composite(self.value + rhs.value, (self.powers, composites))
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
        let composites = self
            .composites_used
            .union(&rhs.composites_used)
            .cloned()
            .collect();
        Unit::new_composite(self.value - rhs.value, (self.powers, composites))
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
        let composites = self
            .composites_used
            .union(&rhs.composites_used)
            .cloned()
            .collect();
        Unit::new_composite_from_vec(self.value * rhs.value, (power_sum, composites))
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
        let composites = self
            .composites_used
            .union(&rhs.composites_used)
            .cloned()
            .collect();
        Unit::new_composite_from_vec(self.value / rhs.value, (power_diff, composites))
    }
}

impl ops::Mul<f64> for Unit {
    type Output = Unit;
    fn mul(self, rhs: f64) -> Self::Output {
        Unit::new_composite(self.value * rhs, (self.powers, self.composites_used))
    }
}
impl ops::Div<f64> for Unit {
    type Output = Unit;
    fn div(self, rhs: f64) -> Self::Output {
        Unit::new_composite(self.value / rhs, (self.powers, self.composites_used))
    }
}
