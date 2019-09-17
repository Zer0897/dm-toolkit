use lazy_static::lazy_static;
use std::collections::HashMap;

use crate::count::{Counter, Value};
use crate::time::UnitTime::*;


#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum UnitTime {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year
}

lazy_static! {
    static ref UNITS: HashMap<UnitTime, Value> = {
        let mut m = HashMap::new();
        m.insert(Second, 1);
        m.insert(Minute, 60);
        m.insert(Hour, 3600);
        m.insert(Day, 86400);
        m.insert(Week, 604800);
        m.insert(Month, 2629800);
        m.insert(Year, 31557600);
        m
    };
}


/// A tool for managing time and its units.
pub struct Time { pub value: Value }

impl Counter for Time {
    type Unit = UnitTime;

    fn value(&self) -> Value {
        self.value
    }
    fn mut_value(&mut self) -> &mut Value {
        &mut self.value
    }
    /// The base value of given `unit`.
    fn value_of(unit: Self::Unit) -> Option<Value> {
        UNITS.get(&unit).cloned()
    }
}

impl Time {
    pub fn new() -> Self {
        Self { value: 0 }
    }
    /// Break up value time into applicable units.
    /// # Example
    /// ```
    /// use lib::count::Counter;
    /// use lib::time::UnitTime::*;
    /// use lib::time::*;
    ///
    /// let mut time = Time::new();
    /// time.add(61, Second);
    ///
    /// assert_eq!(time.distribute(), vec![(1, Minute), (1, Second)]);
    /// ```
    pub fn distribute(&self) -> Vec<(Value, UnitTime)> {
        let mut choices: Vec<(&UnitTime, &Value)> = UNITS.iter().collect();
        choices.sort_by_key(|(_, v)| std::cmp::Reverse(*v));

        let mut out = Vec::new();
        let mut total = self.value();
        for (unit, value) in choices.into_iter() {
            let amount = total / value;
            if amount > 0 {
                out.push((amount, *unit));
                total -= amount * value;
            }
        }
        out
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distribute() {
        let mut time = Time::new();
        time.add(3, Day);
        time.add(1, Second);
        time.add(2, Hour);

        assert_eq!(time.distribute(), vec![(3, Day), (2, Hour), (1, Second)]);
    }

    #[test]
    fn test_distribute_from_one_unit() {
        let mut time = Time::new();
        time.add(61, Second);

        assert_eq!(time.distribute(), vec![(1, Minute), (1, Second)]);
    }
}
