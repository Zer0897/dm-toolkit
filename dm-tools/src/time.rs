use dm_tools_derive::Unit;
use num_derive::{FromPrimitive, ToPrimitive};
use strum_macros::AsStaticStr;

use crate::unit::Unit;

#[derive(
    Debug,
    Copy,
    Clone,
    Hash,
    Eq,
    PartialEq,
    Unit,
    FromPrimitive,
    ToPrimitive,
    Ord,
    PartialOrd,
    AsStaticStr,
)]
pub enum UnitTime {
    Second = 1,
    Minute = 60,
    Hour = 3600,
    Day = 86400,
    Week = 604800,
    Month = 2629800,
    Year = 31557600,
}

/// A tool for managing time and its units.
#[derive(Eq, PartialEq, Ord, PartialOrd, Debug, Clone)]
pub struct Time {
    pub value: i64,
}

impl Time {
    pub fn new() -> Self {
        Self { value: 0 }
    }
    pub fn from(num: i64, unit: UnitTime) -> Self {
        Self {
            value: unit.value() * num,
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
