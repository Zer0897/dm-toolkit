use num_traits::{FromPrimitive, ToPrimitive};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Unit
where
    Self: FromPrimitive + ToPrimitive + Eq + PartialEq + Sized + Debug + Copy + Hash + 'static,
{
    fn variants() -> &'static [Self];

    fn value(&self) -> usize {
        self.to_usize().expect("Error converting.")
    }

    fn distribute_from(units: &[Self], value: usize) -> HashMap<Self, usize> {
        // Allocate space for all possible values that `value` could be distributed into.
        let mut choices: Vec<Option<(usize, Self)>> = Vec::with_capacity(value + 1);
        choices.resize(value + 1, None);

        // Calculate number of steps required for each `unit` to divide into `value` and
        // tabulate the lowest one for each of the possible values.
        for i in 1..=value {
            choices[i] = units
                .iter()
                .map(|u| (u.value(), u))
                // Only values that fit into `i`.
                .filter(|(v, _)| i >= *v)
                // Number of steps required to divide into `i`.
                .map(|(v, u)| choices[i - v].map_or_else(|| (1, *u), |(v, _)| (v + 1, *u)))
                // Minimum number of steps.
                .min_by_key(|x| x.0);
        }

        // Distribute `value` into `units` in the least number of steps possible,
        // counting each unit as it's used.
        let mut rem = value;
        let mut count = HashMap::new();
        while let Some((_, unit)) = choices[rem] {
            rem -= unit.value();
            let counter = count.entry(unit).or_insert(0);
            *counter += 1;
        }
        count
    }

    fn distribute(value: usize) -> HashMap<Self, usize> {
        Self::distribute_from(Self::variants(), value)
    }
}

pub struct UnitCounter<T>
where
    T: Unit,
{
    count: HashMap<T, usize>,
    units: Vec<T>,
}

impl<T> UnitCounter<T>
where
    T: Unit,
{
    pub fn new() -> Self {
        Self::new_with_units(T::variants())
    }

    pub fn new_with_units(units: &[T]) -> Self {
        let units: Vec<T> = units.iter().cloned().collect();
        let count: HashMap<T, usize> = units.iter().cloned().map(|u| (u, 0)).collect();
        Self { count, units }
    }

    pub fn add(&mut self, value: usize) {
        let count = T::distribute_from(&self.units, value);
        for (k, v) in count.into_iter() {
            self.add_units(v, k);
        }
    }

    pub fn sub(&mut self, value: usize) {
        let count = T::distribute_from(&self.units, value);
        for (k, v) in count.into_iter() {
            self.sub_units(v, k);
        }
    }

    pub fn add_units(&mut self, count: usize, unit: T) {
        self.count
            .get_mut(&unit)
            .map(|v| *v += count)
            .expect("Invalid unit");
    }

    pub fn sub_units(&mut self, count: usize, unit: T) {
        let curr = self.get_count(unit);
        if count <= curr {
            self.count
                .get_mut(&unit)
                .map(|v| *v -= count)
                .expect("Invalid unit");
        } else {
            let other = self
                .units
                .iter()
                .filter(|other| other.value() > unit.value())
                .min_by_key(|other| other.value())
                .cloned();

            if let Some(other) = other {
                let other_count = match count * unit.value() / other.value() {
                    0 => 1,
                    val => val,
                };
                self.sub_units(other_count, other);

                let count = curr + (other.value() * other_count) / unit.value() - count;
                self.set_count(count, unit);
            } else {
                panic!("Not enough!")
            }
        }
    }

    pub fn get_count(&self, unit: T) -> usize {
        *self.count.get(&unit).expect("Invalid unit")
    }

    pub fn set_count(&mut self, count: usize, unit: T) {
        self.count
            .get_mut(&unit)
            .map(|v| *v = count)
            .expect("Invalid unit");
    }

    pub fn redistribute(&mut self) {
        loop {
            let result = self
                .count
                .iter()
                .filter_map(|(unit, count)| {
                    self.units
                        .iter()
                        .map(|other| (other.value() / unit.value(), other))
                        .filter(|(div, _)| div <= count && *div > 1)
                        .max_by_key(|(div, _)| *div)
                        .map(|(div, other)| (*unit, *other, div, count / div))
                })
                .next();

            if let Some((unit, other, div, quo)) = result {
                self.add_units(quo, other);
                self.sub_units(div * quo, unit);
            } else {
                break;
            }
        }
    }

    pub fn set_from_string(&mut self, value: &str, unit: T) -> Result<(), std::num::ParseIntError> {
        if value.starts_with('-') || value.starts_with('+') {
            let count: usize = value[1..].parse()?;
            match value.chars().nth(0).unwrap() {
                '-' => self.sub_units(count, unit),
                '+' => self.add_units(count, unit),
                _ => panic!("Somebody fucked up"),
            };
        } else {
            let count: usize = value.parse()?;
            self.set_count(count, unit);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::tests::FooUnit::*;
    use dm_tools_derive::Unit;
    use num_derive::{FromPrimitive, ToPrimitive};

    #[derive(FromPrimitive, ToPrimitive, Debug, Hash, Copy, Clone, PartialEq, Eq, Unit)]
    enum FooUnit {
        One = 1,
        Two = 2,
        Three = 3,
    }

    #[test]
    fn test_distribute_from() {
        let mut value = 0;
        value += Two.value();
        value += One.value();

        let res = FooUnit::distribute_from(&[One, Two], value as usize);
        assert_eq!(res.get(&Two), Some(1 as usize).as_ref());
        assert_eq!(res.get(&One), Some(1 as usize).as_ref());
    }

    #[test]
    fn test_distribute() {
        let mut value = 0;
        value += Two.value();
        value += One.value();

        let res = FooUnit::distribute(value);
        assert_eq!(res.get(&Three), Some(1 as usize).as_ref());
    }

    #[test]
    fn test_distribute_uses_minimum_steps() {
        #[derive(FromPrimitive, ToPrimitive, Hash, Debug, Copy, Clone, PartialEq, Eq, Unit)]
        enum Coins {
            One = 1,
            Three = 3,
            Four = 4,
        }

        let res = Coins::distribute(6);
        assert_eq!(res.get(&Coins::Three), Some(2 as usize).as_ref());
    }

    #[test]
    fn test_distribute_from_one_unit() {
        let mut value = 0;
        value += Two.value() * 61;

        let res = FooUnit::distribute_from(&[One, Two], value as usize);
        assert_eq!(res.get(&Two), Some(61 as usize).as_ref());
    }

    #[test]
    fn test_convert_value() {
        assert_eq!(One.value() * 100 / Two.value(), 50);
    }
}
