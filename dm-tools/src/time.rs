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
    Year,
}

/// A tool for managing time and its units.
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub struct Time {
    pub value: Value,
}

impl Counter for Time {
    type Unit = UnitTime;

    fn value(&self) -> Value {
        self.value
    }
    fn mut_value(&mut self) -> &mut Value {
        &mut self.value
    }
    /// The base value of given `unit`.
    fn value_of(unit: Self::Unit) -> Value {
        match unit {
            Second => 1,
            Minute => 60,
            Hour => 3600,
            Day => 86400,
            Week => 604800,
            Month => 2629800,
            Year => 31557600,
        }
    }
}

impl Time {
    pub fn new() -> Self {
        Self { value: 0 }
    }
    pub fn from(num: Value, unit: UnitTime) -> Self {
        Self {
            value: Self::value_of(unit) * num,
        }
    }
    // pub fn distribute(&self) -> Vec<(Value, UnitTime)> {
    //     let mut choices: Vec<(&UnitTime, &Value)> = UNITS.iter().collect();
    //     choices.sort_by_key(|(_, v)| std::cmp::Reverse(*v));

    //     let mut out = Vec::new();
    //     let mut total = self.value();
    //     for (unit, value) in choices.into_iter() {
    //         let amount = total / value;
    //         if amount > 0 {
    //             out.push((amount, *unit));
    //             total -= amount * value;
    //         }
    //     }
    //     out
    // }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Event {
    pub start: Time,
    pub end: Time,
    pub id: u64,
}

pub struct Schedule {
    pub time: Time,
    events: Vec<Event>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            time: Time::new(),
            events: Vec::new(),
        }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push(event)
    }

    pub fn is_active(&self, event: &Event) -> bool {
        event.start <= self.time && event.end >= self.time
    }

    pub fn get_all(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn get_active(&self) -> Vec<&Event> {
        self.get_all()
            .iter()
            .filter(|e| self.is_active(e))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_compare_gt() {
        assert_eq!(Time::from(1, Second) > Time::new(), true);
    }

    #[test]
    fn test_time_compare_lt() {
        assert_eq!(Time::new() < Time::from(1, Second), true);
    }

    #[test]
    fn test_time_compare_eq() {
        assert_eq!(Time::from(1, Second) == Time::from(1, Second), true);
    }

    #[test]
    fn test_schedule_not_active_before_time() {
        let mut schedule = Schedule::new();
        let start = Time::from(1, Second);
        let end = Time::from(2, Second);
        schedule.push(Event {
            start: start,
            end: end,
            id: 1,
        });
        assert_eq!(schedule.get_active(), Vec::<&Event>::new());
    }

    #[test]
    fn test_schedule_active_in_time() {
        let mut schedule = Schedule::new();
        let start = Time::from(1, Second);
        let end = Time::from(2, Second);
        let event = Event {
            start: start,
            end: end,
            id: 1,
        };
        schedule.push(event.clone());
        schedule.time.add(1, Second);
        assert_eq!(schedule.get_active(), vec![&event]);
    }

    #[test]
    fn test_schedule_not_active_after_time() {
        let mut schedule = Schedule::new();
        let start = Time::from(1, Second);
        let end = Time::from(2, Second);
        let event = Event {
            start: start,
            end: end,
            id: 1,
        };
        schedule.push(event.clone());
        schedule.time.add(3, Second);
        assert_eq!(schedule.get_active(), Vec::<&Event>::new());
    }
}
