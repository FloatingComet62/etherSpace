use crate::{critical, modules::log::Log};
use core::fmt::Display;
use std::{collections::BTreeSet, ops};

const NUM_OF_UNITS: usize = 6;
pub enum Units {
    Meter,
    Kilogram,
    Second,
    Ampere,
    Kelvin,
    Mole,
    // Candela,
}
impl Units {
    pub fn collection() -> [Units; NUM_OF_UNITS] {
        [
            Units::Meter,
            Units::Kilogram,
            Units::Second,
            Units::Ampere,
            Units::Kelvin,
            Units::Mole,
            // Units::Candela,
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
            // Units::Candela => "cd", // if i ever come to using luminocity in my PHYSICS engine, i have already goofed up
        }
        .to_string()
    }
    pub fn prefix_data() -> [(char, f64); 12] {
        [
            ('f', 1e-15), // femto
            ('p', 1e-12), // pico
            ('n', 1e-9),  // nano
            ('u', 1e-6),  // micro
            ('d', 1e-3),  // deci
            ('c', 1e-2),  // centi
            ('h', 1e2),   // hecto
            ('k', 1e3),   // Kilo
            ('M', 1e6),   // Mega
            ('G', 1e9),   // Giga
            ('T', 1e12),  // Tera
            ('P', 1e15),  // Peta
        ]
    }
    pub fn prefix(possibly_prefix: Option<char>) -> f64 {
        let prefix = if let Some(c) = possibly_prefix {
            c
        } else {
            return 1.0;
        };
        for (c, f) in Units::prefix_data().into_iter() {
            if prefix == c {
                return f;
            }
        }
        1.0
    }
    pub fn to_prefix(scale: f64) -> (bool, char) {
        for (c, f) in Units::prefix_data().into_iter() {
            if scale == f {
                return (true, c);
            }
        }
        (false, '\0')
    }
    pub fn composite() -> Vec<(String, ([f64; NUM_OF_UNITS], f64))> {
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
                generate_powers_from_unit_str("dm^3".to_string()),
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

pub fn generate_powers_from_unit_str_composite(
    units_str: String,
) -> ([f64; NUM_OF_UNITS], BTreeSet<String>, f64) {
    let mut powers = vec![0.0; NUM_OF_UNITS];
    let mut composite_units_used = BTreeSet::new();
    let slash_index = units_str.find("/").unwrap_or(units_str.len());
    let mut scale = 1.0;
    for (unit_str, composite_unit) in Units::composite().iter() {
        let index = if let Some(i) = units_str.find(unit_str.as_str()) {
            i
        } else {
            continue;
        };
        composite_units_used.insert(unit_str.clone());
        let fraction_coefficient = if slash_index > index { 1.0 } else { -1.0 };

        let prefix_coefficient = if index == 0 {
            1.0
        } else {
            Units::prefix(units_str.chars().nth(index - 1))
        };
        scale *= prefix_coefficient;

        let mut chars = units_str.chars();
        let carrot = if let Some(x) = chars.nth(index + unit_str.len()) {
            x
        } else {
            for i in 0..powers.len() {
                powers[i] += composite_unit.0[i] * fraction_coefficient;
            }
            scale *= composite_unit.1;
            continue;
        };
        if carrot != '^' {
            for i in 0..powers.len() {
                powers[i] += composite_unit.0[i] * fraction_coefficient;
            }
            scale *= composite_unit.1;
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
        match digits.parse::<f64>() {
            Err(_) => {
                for i in 0..powers.len() {
                    powers[i] += composite_unit.0[i] * fraction_coefficient;
                }
                scale *= composite_unit.1;
            }
            Ok(power) => {
                for i in 0..powers.len() {
                    powers[i] += composite_unit.0[i] * sign * power * fraction_coefficient;
                }
                scale *= composite_unit.1;
            }
        }
    }

    let data = generate_powers_from_unit_str(units_str);
    data.0
        .iter()
        .enumerate()
        .for_each(|(i, power)| powers[i] += *power);

    (
        powers.try_into().unwrap_or_else(|_| {
            critical!("reached unreachable");
        }),
        composite_units_used,
        scale * data.1,
    )
}
fn generate_powers_from_unit_str(units_str: String) -> ([f64; NUM_OF_UNITS], f64) {
    let mut powers = vec![0.0; NUM_OF_UNITS];
    let slash_index = units_str.find("/").unwrap_or(units_str.len());
    let mut scale = 1.0;
    for (i, unit) in Units::collection().iter().enumerate() {
        let unit_str = unit.to_string();
        let index = if let Some(i) = units_str.find(&unit_str) {
            i
        } else {
            continue;
        };
        // allows stuff like "m/s" and "m/s^2"
        let fraction_coefficient = if slash_index > index { 1.0 } else { -1.0 };

        let prefix_index = index as isize - 1;
        let prefix_coefficient: f64 = if index == 0 {
            1.0
        } else {
            Units::prefix(units_str.chars().nth(prefix_index as usize))
        };
        scale *= prefix_coefficient;

        let mut chars = units_str.chars();
        let carrot = if let Some(x) = chars.nth(index + unit_str.len()) {
            x
        } else {
            powers[i] = fraction_coefficient;
            continue;
        };
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
        match digits.parse::<f64>() {
            Err(_) => powers[i] = fraction_coefficient,
            Ok(power) => powers[i] = sign * power * fraction_coefficient,
        }
    }

    (
        powers.try_into().unwrap_or_else(|_| {
            critical!("reached unreachable");
        }),
        scale,
    )
}
fn generate_unit_str_from_powers_composite(
    data: ([f64; NUM_OF_UNITS], BTreeSet<String>, f64),
) -> (String, f64, String) {
    let powers = data.0;
    let mut numerator_str = String::new();
    let mut denominator_str = String::new();

    let mut powers_by_composite = vec![];
    for (unit_str, composite_unit) in Units::composite()
        .into_iter()
        .filter(|(unit_str, _)| data.1.contains(unit_str))
    {
        let (diff_numerator, diff_denominator) = composite_unit.0.iter().enumerate().fold(
            (0.0, 0.0),
            |acc, (i, composite_unit_power)| {
                (
                    acc.0 + (powers[i] - composite_unit_power).abs(),
                    acc.1 + (powers[i] + composite_unit_power).abs(),
                )
            },
        );
        let binding;
        let sign;

        if diff_numerator < diff_denominator {
            binding = 0;
            sign = -1.0;
        } else {
            binding = 1;
            sign = 1.0;
        }
        powers_by_composite.push((
            binding,
            (0..powers.len())
                .map(|i| powers[i] + sign * composite_unit.0[i])
                .collect::<Vec<f64>>(),
            unit_str,
            composite_unit.1,
        ));
    }

    // find the best usage
    let mut best_index = -1;
    let mut best_value = powers.iter().fold(0.0, |acc, x| acc + x.abs());
    for (i, item) in powers_by_composite
        .iter()
        .map(|powers_by_composite| {
            powers_by_composite
                .1
                .iter()
                .fold(0.0, |acc, x| acc + x.abs())
        })
        .enumerate()
    {
        if item <= best_value {
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
                    "Invalid power length, expected length to be {}, found {}",
                    NUM_OF_UNITS,
                    v.len()
                );
            });
        let mut new_data = data.1.clone();
        let mut new_scale: f64 = data.2;
        if best_unit.0 == 0 {
            // unit goes in numerator
            numerator_str += &best_unit.2;
            new_scale /= best_unit.3;
        } else {
            denominator_str += &best_unit.2;
            new_scale *= best_unit.3;
        }
        new_data.insert(best_unit.2.clone());
        let (normal_unit_str_data, scale, prefix) =
            generate_unit_str_from_powers_composite((new_powers, new_data, new_scale));
        let normal_unit_str: Vec<&str> = normal_unit_str_data.split('/').collect();
        if normal_unit_str.len() > 1 {
            return (
                format!(
                    "{}{}{}/{}{}",
                    prefix, numerator_str, normal_unit_str[0], denominator_str, normal_unit_str[1]
                ),
                scale,
                prefix,
            );
        }
        if denominator_str.len() == 0 {
            return (
                format!("{}{}{}", prefix, numerator_str, normal_unit_str[0]),
                scale,
                prefix,
            );
        }
        return (
            format!(
                "{}{}{}/{}",
                prefix, numerator_str, normal_unit_str[0], denominator_str
            ),
            scale,
            prefix,
        );
    }

    let new_scale;
    let mut new_prefix = String::new();
    let (valid_prefix, prefix) = Units::to_prefix(data.2);
    if valid_prefix {
        new_scale = 1.0;
        new_prefix += &prefix.to_string();
    } else {
        new_scale = data.2;
    }

    let normal_unit_str_data = generate_unit_str_from_powers(powers);
    let normal_unit_str: Vec<&str> = normal_unit_str_data.split('/').collect();
    if normal_unit_str.len() > 1 {
        return (
            format!(
                "{}{}/{}{}",
                numerator_str, normal_unit_str[0], denominator_str, normal_unit_str[1]
            ),
            new_scale,
            new_prefix,
        );
    }
    if denominator_str.len() == 0 {
        return (
            format!("{}{}", numerator_str, normal_unit_str[0]),
            new_scale,
            new_prefix,
        );
    }
    (
        format!(
            "{}{}/{}",
            numerator_str, normal_unit_str[0], denominator_str
        ),
        new_scale,
        new_prefix,
    )
}
fn generate_unit_str_from_powers(powers: [f64; NUM_OF_UNITS]) -> String {
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
    pub powers: [f64; NUM_OF_UNITS],
    composites_used: BTreeSet<String>,
    pub scale: f64,
}
impl Unit {
    pub fn new(value: f64, powers: [f64; NUM_OF_UNITS], scale: f64) -> Self {
        let mut unit = Self {
            value,
            powers,
            composites_used: BTreeSet::new(),
            scale,
        };
        unit.check_for_composite();
        unit
    }
    pub fn new_int(value: i64, powers: [u8; NUM_OF_UNITS], scale: f64) -> Self {
        Self::new_from_vec(
            value as f64,
            powers.to_vec().into_iter().map(|x| x as f64).collect(),
            scale,
        )
    }
    pub fn new_from_vec(value: f64, powers: Vec<f64>, scale: f64) -> Self {
        let mut unit = Self {
            value,
            powers: powers.try_into().unwrap_or_else(|v: Vec<f64>| {
                critical!(
                    "Invalid power length, expected length to be {}, found {}",
                    NUM_OF_UNITS,
                    v.len()
                );
            }),
            composites_used: BTreeSet::new(),
            scale,
        };
        unit.check_for_composite();
        unit
    }
    pub fn new_composite(value: f64, data: ([f64; NUM_OF_UNITS], BTreeSet<String>, f64)) -> Self {
        let mut unit = Self {
            value,
            powers: data.0,
            composites_used: data.1,
            scale: data.2,
        };
        unit.check_for_composite();
        unit
    }
    pub fn new_composite_from_vec(
        value: f64,
        data: (Vec<f64>, BTreeSet<String>),
        scale: f64,
    ) -> Self {
        let mut unit = Self {
            value,
            powers: data.0.try_into().unwrap_or_else(|v: Vec<f64>| {
                critical!(
                    "Invalid power length, expected length to be {}, found {}",
                    NUM_OF_UNITS,
                    v.len()
                );
            }),
            composites_used: data.1,
            scale,
        };
        unit.check_for_composite();
        unit
    }
    pub fn check_for_composite(&mut self) {
        for (unit_str, composite_unit) in Units::composite().iter() {
            if composite_unit
                .0
                .iter()
                .enumerate()
                .all(|(i, power)| self.powers[i] == *power)
            {
                self.composites_used.insert(unit_str.clone());
            }
        }
    }
}
impl PartialEq for Unit {
    fn eq(&self, other: &Self) -> bool {
        (self.value * self.scale == other.value * other.scale) && self.powers == other.powers
    }
    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}
impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = generate_unit_str_from_powers_composite((
            self.powers,
            self.composites_used.clone(),
            self.scale,
        ));
        f.write_fmt(format_args!("{:.3} {}", self.value * data.1, data.0))
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
        Unit::new_composite(
            self.value + rhs.value * rhs.scale / self.scale,
            (self.powers, composites, self.scale),
        )
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
        Unit::new_composite(
            self.value - rhs.value * rhs.scale / self.scale,
            (self.powers, composites, self.scale),
        )
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
        Unit::new_composite_from_vec(
            self.value * rhs.value,
            (power_sum, composites),
            self.scale * rhs.scale,
        )
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
        Unit::new_composite_from_vec(
            self.value / rhs.value,
            (power_diff, composites),
            self.scale / rhs.scale,
        )
    }
}

impl ops::Mul<f64> for Unit {
    type Output = Unit;
    fn mul(self, rhs: f64) -> Self::Output {
        Unit::new_composite(
            self.value * rhs,
            (self.powers, self.composites_used, self.scale),
        )
    }
}
impl ops::Div<f64> for Unit {
    type Output = Unit;
    fn div(self, rhs: f64) -> Self::Output {
        Unit::new_composite(
            self.value / rhs,
            (self.powers, self.composites_used, self.scale),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{
        modules::units::{generate_powers_from_unit_str_composite, Unit},
        unit,
    };
    #[test]
    fn force() {
        let acceleration = unit!(5.0, "m/s^2");
        let mass = unit!(20.0, "kg");
        let force = mass * acceleration;
        assert_eq!(force, unit!(100.0, "N"));
    }
    #[test]
    fn force_from_electric_field() {
        let electric_field = unit!(25.0, "N/C");
        let charge = unit!(2.0, "C");
        let force = electric_field * charge;
        assert_eq!(force, unit!(50.0, "N"));
    }
    #[test]
    fn power_out_of_nowhere() {
        let mass = unit!(10.0, "kg");
        let velocity = unit!(50.0, "m/s");
        let energy = mass * velocity.clone() * velocity / 2.0;
        assert_eq!(energy, unit!(12500.0, "J"));

        let power = energy / unit!(600.0, "s");
        assert_eq!(power, unit!(20.833333333333332, "W"));
    }
    #[test]
    fn adding_different_scales() {
        let decilitre = unit!(5.0, "dL");
        let litre = unit!(1.0, "L");
        let result = litre + decilitre;
        assert_eq!(result, unit!(1.005, "L"));
    }
    #[test]
    fn multiplying_different_scales() {
        let electric_field = unit!(25.0, "N/C");
        let charge = unit!(2.0, "uC");
        let force = electric_field * charge;
        assert_eq!(force, unit!(50.0, "uN"))
    }
    #[test]
    fn dividing_different_scales() {
        let force = unit!(100.0, "uN");
        let charge = unit!(2.0, "uC");
        let field = force / charge;
        assert_eq!(field, unit!(50.0, "N/C"));
    }
}
