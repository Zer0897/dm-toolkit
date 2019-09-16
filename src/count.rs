use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;


pub type Value = i64;


#[derive(Clone)]
pub struct Counter<'a, U> { value: Value, units: &'a HashMap<U, Value> }


impl<'a, U> Counter<'a, U> where
    U: Eq + Hash
{
    pub fn new(units: &'a HashMap<U, Value>) -> Self { Counter { value: 0, units: units } }

    pub fn value_of(&self, unit: U) -> Option<Value> {
        self.units.get(&unit).cloned()
    }
    pub fn to(&self, unit: U) -> Option<Value> {
        self.value_of(unit).map(|v| self.value / v)
    }
    pub fn set(&mut self, num: Value, unit: U) {
        self.value = self.value_of(unit).map(|v| num * v).unwrap();
    }
    pub fn add(&mut self, num: Value, unit: U) {
        self.value += self.value_of(unit).map(|v| num * v).unwrap();
    }
    pub fn sub(&mut self, num: Value, unit: U) {
        self.add(-num, unit);
    }

    pub fn value(&self) -> Value { self.value }
}
