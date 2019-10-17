use num_traits::{FromPrimitive, ToPrimitive};

pub trait Unit
where
    Self: FromPrimitive + ToPrimitive + Sized + Copy + 'static,
{
    fn variants() -> &'static [Self];

    fn value(&self) -> i64 {
        self.to_i64().expect("Error converting.")
    }

    fn distribute_from(units: &[Self], value: i64) -> Vec<(i64, Self)> {
        let mut choices: Vec<(i64, &Self)> = units.iter().map(|u| (u.value(), u)).collect();
        choices.sort_by_key(|(v, _)| std::cmp::Reverse(*v));

        let mut out = Vec::new();
        let mut total = value;
        for (value, unit) in choices.into_iter() {
            let amount = total / value;
            if amount > 0 {
                out.push((amount, *unit));
                total -= amount * value;
            }
        }
        out
    }

    fn distribute(value: i64) -> Vec<(i64, Self)> {
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
            FooUnit::distribute_from(&[One, Two], value),
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
    fn test_distribute_from_one_unit() {
        let mut value = 0;
        value += Two.value() * 61;

        assert_eq!(
            FooUnit::distribute_from(&[One, Two], value),
            vec![(61, Two)]
        );
    }

    #[test]
    fn test_convert_value() {
        assert_eq!(One.value() * 100 / Two.value(), 50);
    }
}
