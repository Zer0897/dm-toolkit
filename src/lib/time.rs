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

type TimeCounter = Counter<'static, UnitTime>;


pub struct Time { pub current: TimeCounter }

impl Time {
    pub fn new() -> Self {
        Self { current: Counter::new(&UNITS) }
    }

    pub fn distribute(&self) -> Vec<(Value, UnitTime)> {
        let mut choices: Vec<(&UnitTime, &Value)> = UNITS.iter().collect();
        choices.sort_by_key(|(_, v)| std::cmp::Reverse(*v));

        let mut out = Vec::new();
        let mut total = self.current.value();
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
