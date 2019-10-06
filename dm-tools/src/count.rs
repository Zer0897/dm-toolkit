use std::cmp::Eq;
use std::hash::Hash;

pub type Value = i64;

/// A tool for counting with the given `units`.
/// # Example
/// ```
/// use dm_tools::count::*;
/// struct FooCounter {
///     value: Value,
/// }

/// impl Counter for FooCounter {
///     type Unit = &'static str;

///     fn value(&self) -> Value {
///         self.value
///     }
///     fn mut_value(&mut self) -> &mut Value {
///         &mut self.value
///     }
///     /// The base value of given `unit`.
///     fn value_of(unit: &'static str) -> Value {
///         match unit {
///             "one" => 1,
///             "two" => 2,
///             "three" => 3,
///             _ => 1,
///         }
///     }
/// }

/// let mut counter = FooCounter { value: 0 };
/// counter.add(3, "one");
/// counter.add(1, "two");

/// assert_eq!(
///     counter.distribute(&["one", "two"]),
///     vec![(2, "two"), (1, "one")]
/// );
///
/// ```
pub trait Counter {
    type Unit: Eq + Hash + Copy;

    /// The current value of the counter.
    fn value(&self) -> Value;

    /// Mutable access to current value.
    fn mut_value(&mut self) -> &mut Value;

    /// The base value of given `unit`.
    fn value_of(unit: Self::Unit) -> Value;

    /// The number of `unit`s that fit into the current value.
    fn as_unit(&self, unit: Self::Unit) -> Value {
        self.value() / Self::value_of(unit)
    }
    /// Set the current value equal to `num` of `units`.
    fn set(&mut self, num: Value, unit: Self::Unit) {
        *self.mut_value() = Self::value_of(unit) * num;
    }
    /// Add the current value to `num` of `units`.
    fn add(&mut self, num: Value, unit: Self::Unit) {
        *self.mut_value() += Self::value_of(unit) * num;
    }
    /// Subtract the current value from `num` of `units`.
    fn sub(&mut self, num: Value, unit: Self::Unit) {
        self.add(-num, unit);
    }

    /// Break up value time into applicable units.
    fn distribute(&self, units: &[Self::Unit]) -> Vec<(Value, Self::Unit)> {
        let mut choices: Vec<(Value, Self::Unit)> =
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

    struct FooCounter {
        value: Value,
    }

    impl Counter for FooCounter {
        type Unit = &'static str;

        fn value(&self) -> Value {
            self.value
        }
        fn mut_value(&mut self) -> &mut Value {
            &mut self.value
        }
        /// The base value of given `unit`.
        fn value_of(unit: &'static str) -> Value {
            match unit {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                _ => 1,
            }
        }
    }

    #[test]
    fn test_distribute() {
        let mut counter = FooCounter { value: 0 };
        counter.add(3, "one");
        counter.add(1, "two");

        assert_eq!(
            counter.distribute(&["one", "two"]),
            vec![(2, "two"), (1, "one")]
        );
    }

    #[test]
    fn test_distribute_from_one_unit() {
        let mut counter = FooCounter { value: 0 };
        counter.add(61, "two");

        assert_eq!(counter.distribute(&["one", "two"]), vec![(61, "two")]);
    }

    #[test]
    fn test_add_twice() {
        let mut counter = FooCounter { value: 0 };

        counter.add(1, "one");
        counter.add(1, "one");
        assert_eq!(counter.value(), 2);
    }

    #[test]
    fn test_add_two() {
        let mut counter = FooCounter { value: 0 };

        counter.add(2, "one");
        assert_eq!(counter.value(), 2);
    }

    #[test]
    fn test_set_value() {
        let mut counter = FooCounter { value: 0 };

        counter.set(2, "one");
        assert_eq!(counter.value(), 2);

        counter.set(1, "one");
        assert_eq!(counter.value(), 1);
    }

    #[test]
    fn test_convert_value() {
        let mut counter = FooCounter { value: 0 };

        counter.set(100, "one");
        assert_eq!(counter.as_unit("two"), 50);

        counter.set(100, "two");
        assert_eq!(counter.as_unit("one"), 200);
    }

    #[test]
    fn test_subtract() {
        let mut counter = FooCounter { value: 0 };

        counter.set(2, "one");
        counter.sub(1, "one");
        assert_eq!(counter.value(), 1);
    }

    #[test]
    fn test_sub_is_same_as_neg_add() {
        let mut counter = FooCounter { value: 0 };

        counter.set(2, "one");
        counter.sub(1, "one");
        counter.add(-1, "one");
        assert_eq!(counter.value(), 0);
    }

    #[test]
    fn test_add_is_same_as_neg_sub() {
        let mut counter = FooCounter { value: 0 };

        counter.add(1, "one");
        counter.sub(-1, "one");
        assert_eq!(counter.value(), 2);
    }

    #[test]
    fn test_value_of_unit_matches_conversion() {
        let mut counter = FooCounter { value: 0 };

        counter.add(1, "two");
        assert_eq!(counter.value(), FooCounter::value_of("two"));
    }
}
