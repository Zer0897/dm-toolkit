use num_traits::{FromPrimitive, ToPrimitive};
use std::collections::HashMap;
use std::hash::Hash;

pub trait Unit
where
    Self: FromPrimitive
        + ToPrimitive
        + Eq
        + PartialEq
        + Sized
        + std::fmt::Debug
        + Copy
        + Hash
        + 'static,
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
            if let Some(c) = count.get_mut(&unit) {
                *c += 1;
            } else {
                count.insert(unit, 1);
            }
        }

        count
    }

    fn distribute(value: usize) -> HashMap<Self, usize> {
        Self::distribute_from(Self::variants(), value)
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
