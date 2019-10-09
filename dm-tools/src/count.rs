use num_traits::{FromPrimitive, ToPrimitive};

/// A tool for counting with the given `units`.
/// # Example
/// ```
/// use num_derive::{FromPrimitive, ToPrimitive};
/// use dm_tools::count::*;
///
/// struct FooCounter {
///     value: i64,
/// }
///
/// #[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq)]
/// enum FooUnit {
///     One = 1,
///     Two = 2,
///     Three = 3
/// }
///
/// impl Count for FooCounter {
///     type Unit = FooUnit;
///
///     fn value(&self) -> i64 {
///         self.value
///     }
///     fn mut_value(&mut self) -> &mut i64 {
///         &mut self.value
///     }
/// }
///
/// let mut counter = FooCounter { value: 0 };
/// counter.add(3, FooUnit::One);
/// counter.add(1, FooUnit::Two);
///
/// assert_eq!(
///     counter.distribute(&[FooUnit::One, FooUnit::Two]),
///     vec![(2, FooUnit::Two), (1, FooUnit::One)]
/// );
///
/// ```
pub trait Count {
    type Unit: FromPrimitive + ToPrimitive + std::fmt::Debug + Copy;

    /// Raw value (no units).
    fn value(&self) -> i64;

    /// Mutable access to raw value (no units).
    fn mut_value(&mut self) -> &mut i64;

    /// Get `unit`s value.
    fn value_of(unit: Self::Unit) -> i64 {
        unit.to_i64()
            .expect(&format!("Unable to value_of {:?}", unit))
    }

    /// Convert current value into number of `unit`s.
    /// # Example
    /// ```
    /// use num_derive::{FromPrimitive, ToPrimitive};
    /// use dm_tools::count::*;
    ///
    /// struct FooCounter {
    ///     value: i64,
    /// }
    ///
    /// #[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq)]
    /// enum FooUnit {
    ///     One = 1,
    ///     Two = 2,
    ///     Three = 3
    /// }
    ///
    /// impl Count for FooCounter {
    ///     type Unit = FooUnit;
    ///
    ///     fn value(&self) -> i64 {
    ///         self.value
    ///     }
    ///     fn mut_value(&mut self) -> &mut i64 {
    ///         &mut self.value
    ///     }
    /// }
    ///
    /// let counter = FooCounter { value: 3 };
    /// assert_eq!(counter.convert(FooUnit::One), 3);
    /// // If the current value is not evenly divisible, the result will
    /// // be rounded down.
    /// assert_eq!(counter.convert(FooUnit::Two), 1);
    /// ```
    fn convert(&self, unit: Self::Unit) -> i64 {
        self.value() / Self::value_of(unit)
    }
    /// Set the current value equal to `num` of `units`.
    fn set(&mut self, num: i64, unit: Self::Unit) {
        *self.mut_value() = Self::value_of(unit) * num;
    }
    /// Add the current value to `num` of `units`.
    fn add(&mut self, num: i64, unit: Self::Unit) {
        *self.mut_value() += Self::value_of(unit) * num;
    }
    /// Subtract the current value from `num` of `units`.
    fn sub(&mut self, num: i64, unit: Self::Unit) {
        self.add(-num, unit);
    }

    /// Distribute value into given units. Largest are used first. The returning vector
    /// is always ordered.
    /// # Example
    /// ```
    /// use num_derive::{FromPrimitive, ToPrimitive};
    /// use dm_tools::count::*;
    ///
    /// struct FooCounter {
    ///     value: i64,
    /// }
    ///
    /// #[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq)]
    /// enum FooUnit {
    ///     One = 1,
    ///     Two = 2,
    ///     Three = 3,
    ///     Four = 4
    /// }
    ///
    /// impl Count for FooCounter {
    ///     type Unit = FooUnit;
    ///
    ///     fn value(&self) -> i64 {
    ///         self.value
    ///     }
    ///     fn mut_value(&mut self) -> &mut i64 {
    ///         &mut self.value
    ///     }
    /// }
    ///
    /// let counter = FooCounter { value: 3 };
    /// // Units that don't fit will be left out.
    /// assert_eq!(counter.distribute(
    ///     &[FooUnit::One, FooUnit::Two, FooUnit::Four]),
    ///     vec![(1, FooUnit::Two), (1, FooUnit::One)])
    /// ```
    fn distribute(&self, units: &[Self::Unit]) -> Vec<(i64, Self::Unit)> {
        let mut choices: Vec<(i64, Self::Unit)> =
            units.iter().map(|u| (Self::value_of(*u), *u)).collect();
        choices.sort_by_key(|(v, _)| std::cmp::Reverse(*v));

        let mut out = Vec::new();
        let mut total = self.value();
        for (value, unit) in choices.into_iter() {
            let amount = total / value;
            if amount > 0 {
                out.push((amount, unit));
                total -= amount * value;
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::count::tests::FooUnit::*;
    use num_derive::{FromPrimitive, ToPrimitive};

    struct FooCounter {
        value: i64,
    }

    #[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq)]
    enum FooUnit {
        One = 1,
        Two = 2,
        Three = 3,
    }

    impl Count for FooCounter {
        type Unit = FooUnit;

        fn value(&self) -> i64 {
            self.value
        }
        fn mut_value(&mut self) -> &mut i64 {
            &mut self.value
        }
    }

    #[test]
    fn test_distribute() {
        let mut counter = FooCounter { value: 0 };
        counter.add(3, One);
        counter.add(1, Two);

        assert_eq!(counter.distribute(&[One, Two]), vec![(2, Two), (1, One)]);
    }

    #[test]
    fn test_distribute_from_one_unit() {
        let mut counter = FooCounter { value: 0 };
        counter.add(61, Two);

        assert_eq!(counter.distribute(&[One, Two]), vec![(61, Two)]);
    }

    #[test]
    fn test_add_twice() {
        let mut counter = FooCounter { value: 0 };

        counter.add(1, One);
        counter.add(1, One);
        assert_eq!(counter.value(), 2);
    }

    #[test]
    fn test_add_two() {
        let mut counter = FooCounter { value: 0 };

        counter.add(2, One);
        assert_eq!(counter.value(), 2);
    }

    #[test]
    fn test_set_value() {
        let mut counter = FooCounter { value: 0 };

        counter.set(2, One);
        assert_eq!(counter.value(), 2);

        counter.set(1, One);
        assert_eq!(counter.value(), 1);
    }

    #[test]
    fn test_convert_value() {
        let mut counter = FooCounter { value: 0 };

        counter.set(100, One);
        assert_eq!(counter.convert(Two), 50);

        counter.set(100, Two);
        assert_eq!(counter.convert(One), 200);
    }

    #[test]
    fn test_subtract() {
        let mut counter = FooCounter { value: 0 };

        counter.set(2, One);
        counter.sub(1, One);
        assert_eq!(counter.value(), 1);
    }

    #[test]
    fn test_sub_is_same_as_neg_add() {
        let mut counter = FooCounter { value: 0 };

        counter.set(2, One);
        counter.sub(1, One);
        counter.add(-1, One);
        assert_eq!(counter.value(), 0);
    }

    #[test]
    fn test_add_is_same_as_neg_sub() {
        let mut counter = FooCounter { value: 0 };

        counter.add(1, One);
        counter.sub(-1, One);
        assert_eq!(counter.value(), 2);
    }

    #[test]
    fn test_convert_unit_matches_conversion() {
        let mut counter = FooCounter { value: 0 };

        counter.add(1, Two);
        assert_eq!(counter.value(), FooCounter::value_of(Two));
    }
}
