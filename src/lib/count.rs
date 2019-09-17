use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;


pub type Value = i64;


/// A tool for counting with the given `units`. Takes a hashmap of the unit identifier and its
/// value.
/// # Example
/// ```
/// let mut units = HashMap::new();
/// m.insert("one".to_owned(), 1);
/// m.insert("two".to_owned(), 2);
///
/// let mut counter = Counter::new(&units);
///
/// counter.add(1, "two".to_owned());
/// assert_eq!(counter.value(), 2);
/// ```
#[derive(Clone)]
pub struct Counter<'a, U> { value: Value, units: &'a HashMap<U, Value> }


impl<'a, U> Counter<'a, U> where
    U: Eq + Hash
{
    pub fn new(units: &'a HashMap<U, Value>) -> Self {
        Self { value: 0, units: units }
    }
    /// The current value of the counter.
    pub fn value(&self) -> Value { self.value }
    /// The base value of given `unit`.
    pub fn value_of(&self, unit: U) -> Option<Value> {
        self.units.get(&unit).cloned()
    }
    /// The number of `unit`s that fit into the current value.
    /// # Example
    /// ```
    /// let mut units = HashMap::new();
    /// m.insert("one".to_owned(), 1);
    /// m.insert("two".to_owned(), 2);
    ///
    /// counter.set(100, "one".to_owned());
    /// assert_eq!(counter.to("two".to_owned()), Some(50));

    /// counter.set(100, "two".to_owned());
    /// assert_eq!(counter.to("one".to_owned()), Some(200));
    /// ```
    pub fn to(&self, unit: U) -> Option<Value> {
        self.value_of(unit).map(|v| self.value / v)
    }
    /// Set the current value equal to `num` of `units`.
    pub fn set(&mut self, num: Value, unit: U) {
        self.value = self.value_of(unit).map(|v| num * v).unwrap();
    }
    /// Add the current value to `num` of `units`.
    pub fn add(&mut self, num: Value, unit: U) {
        self.value += self.value_of(unit).map(|v| num * v).unwrap();
    }
    /// Subtract the current value from `num` of `units`.
    pub fn sub(&mut self, num: Value, unit: U) {
        self.add(-num, unit);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    fn foo_units() -> HashMap<String, Value> {
        let mut m = HashMap::new();
        m.insert("one".to_owned(), 1);
        m.insert("two".to_owned(), 2);
        m
    }

    #[test]
    fn test_add_twice() {
        let units = foo_units();
        let mut counter = Counter::new(&units);

        counter.add(1, "one".to_owned());
        counter.add(1, "one".to_owned());
        assert_eq!(counter.value(), 2);
    }

    #[test]
    fn test_add_two() {
        let units = foo_units();
        let mut counter = Counter::new(&units);

        counter.add(2, "one".to_owned());
        assert_eq!(counter.value(), 2);
    }

    #[test]
    fn test_set_value() {
        let units = foo_units();
        let mut counter = Counter::new(&units);

        counter.set(2, "one".to_owned());
        assert_eq!(counter.value(), 2);

        counter.set(1, "one".to_owned());
        assert_eq!(counter.value(), 1);
    }

    #[test]
    fn test_convert_value() {
        let units = foo_units();
        let mut counter = Counter::new(&units);

        counter.set(100, "one".to_owned());
        assert_eq!(counter.to("two".to_owned()), Some(50));

        counter.set(100, "two".to_owned());
        assert_eq!(counter.to("one".to_owned()), Some(200));
    }

    #[test]
    fn test_subtract() {
        let units = foo_units();
        let mut counter = Counter::new(&units);

        counter.set(2, "one".to_owned());
        counter.sub(1, "one".to_owned());
        assert_eq!(counter.value(), 1);
    }

    #[test]
    fn test_sub_is_same_as_neg_add() {
        let units = foo_units();
        let mut counter = Counter::new(&units);

        counter.set(2, "one".to_owned());
        counter.sub(1, "one".to_owned());
        counter.add(-1, "one".to_owned());
        assert_eq!(counter.value(), 0);
    }

    #[test]
    fn test_add_is_same_as_neg_sub() {
        let units = foo_units();
        let mut counter = Counter::new(&units);

        counter.add(1, "one".to_owned());
        counter.sub(-1, "one".to_owned());
        assert_eq!(counter.value(), 2);
    }

    #[test]
    fn test_value_of_unit_matches_conversion() {
        let units = foo_units();
        let mut counter = Counter::new(&units);

        counter.add(1, "two".to_owned());
        assert_eq!(counter.value(), counter.value_of("two".to_owned()).unwrap());
    }
}
