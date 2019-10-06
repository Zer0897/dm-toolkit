use std::cmp::Eq;
use std::hash::Hash;

pub type Value = i64;

/// A tool for counting with the given `units`.
pub trait Counter {
    type Unit: Eq + Hash;

    /// The current value of the counter.
    fn value(&self) -> Value;

    /// Mutable access to current value.
    fn mut_value(&mut self) -> &mut Value;

    /// The base value of given `unit`.
    fn value_of(unit: Self::Unit) -> Option<Value>;

    /// The number of `unit`s that fit into the current value.
    fn as_unit(&self, unit: Self::Unit) -> Option<Value> {
        Self::value_of(unit).map(|v| self.value() / v)
    }
    /// Set the current value equal to `num` of `units`.
    fn set(&mut self, num: Value, unit: Self::Unit) {
        *self.mut_value() = Self::value_of(unit).map(|v| num * v).unwrap();
    }
    /// Add the current value to `num` of `units`.
    fn add(&mut self, num: Value, unit: Self::Unit) {
        *self.mut_value() += Self::value_of(unit).map(|v| num * v).unwrap();
    }
    /// Subtract the current value from `num` of `units`.
    fn sub(&mut self, num: Value, unit: Self::Unit) {
        self.add(-num, unit);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

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
        fn value_of(unit: &'static str) -> Option<Value> {
            let mut m = HashMap::new();
            m.insert("one", 1);
            m.insert("two", 2);
            m.get(unit).cloned()
        }
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
        assert_eq!(counter.as_unit("two"), Some(50));

        counter.set(100, "two");
        assert_eq!(counter.as_unit("one"), Some(200));
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
        assert_eq!(counter.value(), FooCounter::value_of("two").unwrap());
    }
}
