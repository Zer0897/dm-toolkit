use lazy_static::lazy_static;
use std::collections::HashMap;
use super::count::{Counter, Value};

use crate::UnitTime::*;


#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
pub enum UnitTime {
    SECOND,
    MINUTE,
    HOUR,
    DAY,
    WEEK,
    MONTH,
    YEAR
}

lazy_static! {
    static ref UNITS: HashMap<UnitTime, Value> = {
        let mut m = HashMap::new();
        m.insert(SECOND, 1);
        m.insert(MINUTE, 60);
        m.insert(HOUR, 3600);
        m.insert(DAY, 86400);
        m.insert(WEEK, 604800);
        m.insert(MONTH, 2629800);
        m.insert(YEAR, 31557600);
        m
    };
}

type TimeCounter = Counter<'static, UnitTime>;


pub struct Time { pub current: TimeCounter }

impl Time {
    pub fn new() -> Self {
        Time { current: Counter::new(&UNITS) }
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
