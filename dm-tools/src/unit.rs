use num::Integer;
use num_traits::{FromPrimitive, ToPrimitive};
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

pub trait Unit:
    FromPrimitive
    + ToPrimitive
    + Eq
    + PartialEq
    + Sized
    + Debug
    + Copy
    + Hash
    + Ord
    + PartialOrd
    + 'static
{
    fn variants() -> &'static [Self];

    fn value(&self) -> i64 {
        self.to_i64().expect("Error converting.")
    }

    fn distribute_from(units: &[Self], value: usize) -> HashMap<Self, i64> {
        // Allocate space for all possible values that `value` could be distributed into.
        let mut choices: Vec<Option<(usize, Self)>> = Vec::with_capacity(value + 1);
        choices.resize(value + 1, None);

        // Calculate number of steps required for each `unit` to divide into `value` and
        // tabulate the lowest one for each of the possible values.
        for i in 1..=value {
            choices[i] = units
                .iter()
                .map(|u| (u.value() as usize, u))
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
            rem -= unit.value() as usize;
            let counter = count.entry(unit).or_insert(0);
            *counter += 1;
        }
        count
    }

    fn distribute(value: usize) -> HashMap<Self, i64> {
        Self::distribute_from(Self::variants(), value)
    }
}

#[derive(Debug)]
pub enum CountError {
    UnitNotFound,
    NotEnough,
    InvalidValue,
}

pub struct UnitCounter<T>
where
    T: Unit,
{
    count: HashMap<T, i64>,
    units: Vec<T>,
}

impl<T> UnitCounter<T>
where
    T: Unit,
{
    pub fn new() -> Self {
        Self::new_with_units(T::variants())
    }

    /// Create a new counter using only the provided units.
    pub fn new_with_units(units: &[T]) -> Self {
        let mut units: Vec<T> = units.iter().cloned().collect();
        let count: HashMap<T, i64> = units.iter().cloned().map(|u| (u, 0)).collect();
        units.sort();
        Self { count, units }
    }

    pub fn add(&mut self, value: i64) -> Result<(), CountError> {
        let count = T::distribute_from(&self.units, value.abs() as usize);
        for (k, v) in count.into_iter() {
            self.add_units(v * value.signum(), &k)?;
        }
        Ok(())
    }

    pub fn sub(&mut self, value: i64) -> Result<(), CountError> {
        self.add(-value)
    }

    pub fn add_units(&mut self, count: i64, unit: &T) -> Result<(), CountError> {
        self.get_mut_count(unit).map(|v| *v += count)
    }

    pub fn sub_units(&mut self, count: i64, unit: &T) -> Result<(), CountError> {
        self.add_units(-count, unit)
    }

    pub fn get_count(&self, unit: &T) -> Result<i64, CountError> {
        self.count
            .get(&unit)
            .map_or_else(|| Err(CountError::UnitNotFound), |v| Ok(*v))
    }

    fn get_mut_count(&mut self, unit: &T) -> Result<&mut i64, CountError> {
        self.count
            .get_mut(unit)
            .map_or_else(|| Err(CountError::UnitNotFound), |v| Ok(v))
    }

    pub fn set_units(&mut self, count: i64, unit: &T) -> Result<(), CountError> {
        self.get_mut_count(unit).map(|v| *v = count)
    }

    pub fn redistribute(&mut self) -> Result<(), CountError> {
        // Assumed `self.units` is sorted
        let units = self.units.clone();
        for (i, unit) in units.iter().enumerate() {
            let count = self.get_count(&unit)?;
            let result = units.get(i + 1).map(|next| {
                let total = unit.value() * count;
                // Calculate how many of `next` units we need to either
                // steal (negative) or add (positive).
                let (quo, rem) = total.div_rem(&next.value());
                // Stealing with remainder will need one extra to cover it.
                let next_count = if rem.is_negative() { quo - 1 } else { quo };
                (next_count, next)
            });
            match result {
                Some((next_count, next)) if next_count != 0 => {
                    self.add_units(next_count, &next)?;
                    self.sub_units(next_count * next.value() / unit.value(), &unit)?;
                }
                // Last unit, can't steal anymore.
                None if count.is_negative() => {
                    self.reset(&units)?;
                    return Err(CountError::NotEnough);
                }
                _ => {}
            };
        }
        Ok(())
    }

    pub fn reset(&mut self, units: &[T]) -> Result<(), CountError> {
        for unit in units.iter() {
            self.set_units(0, unit)?;
        }
        Ok(())
    }

    // TODO Refactor
    pub fn set_from_text(&mut self, value: &str, unit: &T) -> Result<(), CountError> {
        if value.starts_with('-') || value.starts_with('+') {
            if let Ok(count) = value[1..].parse::<i64>() {
                match value.chars().nth(0).unwrap() {
                    '-' => self.sub_units(count, unit)?,
                    '+' => self.add_units(count, unit)?,
                    _ => unreachable!(),
                };
            } else {
                return Err(CountError::InvalidValue);
            }
        } else {
            if let Ok(count) = value.parse::<i64>() {
                self.set_units(count, unit)?;
            } else {
                return Err(CountError::InvalidValue);
            }
        }
        Ok(())
    }
}
