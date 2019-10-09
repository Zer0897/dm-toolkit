use num_derive::{FromPrimitive, ToPrimitive};

use crate::count::Count;


#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq)]
pub enum UnitTime {
    Second = 1,
    Minute = 60,
    Hour = 3600,
    Day = 86400,
    Week = 604800,
    Month = 2629800,
    Year = 31557600,
}

/// Tool for managing time.
/// # Example
/// ```
/// use dm_tools::time::{Time, UnitTime};
/// use dm_tools::count::Count;
///
/// let mut t = Time::new();
/// t.add(1, UnitTime::Year);
/// t.add(1, UnitTime::Hour);
/// t.add(5, UnitTime::Second);
///
/// assert_eq!(t.convert(UnitTime::Second), 31561205);
/// assert_eq!(t.distribute(
///     &[UnitTime::Second, UnitTime::Week, UnitTime::Minute, UnitTime::Hour]),
///     vec![(52, UnitTime::Week), (31, UnitTime::Hour), (5, UnitTime::Second)])
/// ```
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub struct Time {
    pub value: i64,
}

impl Count for Time {
    type Unit = UnitTime;

    fn value(&self) -> i64 {
        self.value
    }
    fn mut_value(&mut self) -> &mut i64 {
        &mut self.value
    }
}

impl Time {
    pub fn new() -> Self {
        Self { value: 0 }
    }
    pub fn from(num: i64, unit: UnitTime) -> Self {
        Self {
            value: Self::value_of(unit) * num,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Event {
    pub start: Time,
    pub end: Time,
    pub id: u64,
}

pub struct Scheduler {
    pub time: Time,
    events: Vec<Event>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            time: Time::new(),
            events: Vec::new(),
        }
    }

    pub fn push(&mut self, event: Event) {
        self.events.push(event)
    }

    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }

    pub fn active_events(&self) -> impl Iterator<Item = &Event> {
        self.events().iter().filter(move |e| self.is_active(e))
    }

    pub fn events_by_id(&self, event_id: u64) -> impl Iterator<Item = &Event> {
        self.events().iter().filter(move |e| e.id == event_id)
    }

    pub fn active_events_by_id(&self, event_id: u64) -> impl Iterator<Item = &Event> {
        self.events_by_id(event_id)
            .filter(move |e| self.is_active(e))
    }

    pub fn is_active(&self, event: &Event) -> bool {
        event.start <= self.time && event.end >= self.time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::UnitTime::*;

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
        let mut schedule = Scheduler::new();
        let start = Time::from(1, Second);
        let end = Time::from(2, Second);
        schedule.push(Event {
            start: start,
            end: end,
            id: 1,
        });
        assert_eq!(
            schedule.active_events().collect::<Vec<_>>(),
            Vec::<&Event>::new()
        );
    }

    #[test]
    fn test_schedule_active_in_time() {
        let mut schedule = Scheduler::new();
        let start = Time::from(1, Second);
        let end = Time::from(2, Second);
        let event = Event {
            start: start,
            end: end,
            id: 1,
        };
        schedule.push(event.clone());
        schedule.time.add(1, Second);
        assert_eq!(schedule.active_events().collect::<Vec<_>>(), vec![&event]);
    }

    #[test]
    fn test_schedule_not_active_after_time() {
        let mut schedule = Scheduler::new();
        let start = Time::from(1, Second);
        let end = Time::from(2, Second);
        let event = Event {
            start: start,
            end: end,
            id: 1,
        };
        schedule.push(event.clone());
        schedule.time.add(3, Second);
        assert_eq!(
            schedule.active_events().collect::<Vec<_>>(),
            Vec::<&Event>::new()
        );
    }

    #[test]
    fn test_events_by_id() {
        let mut schedule = Scheduler::new();
        let event = Event {
            start: Time::from(1, Second),
            end: Time::from(2, Second),
            id: 1,
        };
        let event2 = Event {
            start: Time::from(1, Second),
            end: Time::from(2, Second),
            id: 2,
        };
        schedule.push(event.clone());
        schedule.push(event2.clone());
        assert_eq!(schedule.events_by_id(2).collect::<Vec<_>>(), vec![&event2]);
    }

    #[test]
    fn test_active_events_by_id() {
        let mut schedule = Scheduler::new();
        let event = Event {
            start: Time::from(3, Second),
            end: Time::from(4, Second),
            id: 2,
        };
        let event2 = Event {
            start: Time::from(1, Second),
            end: Time::from(2, Second),
            id: 2,
        };
        schedule.push(event.clone());
        schedule.push(event2.clone());
        schedule.time.add(1, Second);
        assert_eq!(
            schedule.active_events_by_id(2).collect::<Vec<_>>(),
            vec![&event2]
        );
    }
}
