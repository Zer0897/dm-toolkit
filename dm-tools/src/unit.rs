use itertools::Itertools;
use num_traits::{FromPrimitive, ToPrimitive};
use std::collections::HashMap;

pub trait Unit
where
    Self: FromPrimitive + ToPrimitive + Sized + std::fmt::Debug + Copy + 'static,
{
    fn variants() -> &'static [Self];

    fn value(&self) -> usize {
        self.to_usize().expect("Error converting.")
    }

    fn distribute_from(units: &[Self], value: usize) -> Vec<(usize, Self)> {
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
            if let Some(c) = count.get_mut(&unit.value()) {
                *c += 1;
            } else {
                count.insert(unit.value(), 1);
            }
        }

        // Transform and sort output.
        count
            .into_iter()
            .map(|(k, v)| (v, Self::from_usize(k).unwrap()))
            .sorted_by_key(|(_, u)| std::cmp::Reverse(u.value()))
            .collect()
    }

    fn distribute(value: usize) -> Vec<(usize, Self)> {
        Self::distribute_from(Self::variants(), value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit::tests::FooUnit::*;
    use dm_tools_derive::Unit;
    use num_derive::{FromPrimitive, ToPrimitive};

    #[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq, Unit)]
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

        assert_eq!(
            FooUnit::distribute_from(&[One, Two], value as usize),
            vec![(1, Two), (1, One)]
        );
    }

    #[test]
    fn test_distribute() {
        let mut value = 0;
        value += Two.value();
        value += One.value();

        assert_eq!(FooUnit::distribute(value), vec![(1, Three)]);
    }

    #[test]
    fn test_distribute_uses_minimum_steps() {
        #[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq, Unit)]
        enum Coins {
            One = 1,
            Three = 3,
            Four = 4,
        }

        // Greedy distribution would be [(1, Coins::Four), (2, Coins::One)]
        assert_eq!(Coins::distribute(6), vec![(2, Coins::Three)]);
    }

    #[test]
    fn test_distribute_from_one_unit() {
        let mut value = 0;
        value += Two.value() * 61;

        assert_eq!(
            FooUnit::distribute_from(&[One, Two], value as usize),
            vec![(61, Two)]
        );
    }

    #[test]
    fn test_convert_value() {
        assert_eq!(One.value() * 100 / Two.value(), 50);
    }
}
