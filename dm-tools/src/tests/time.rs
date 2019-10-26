use crate::time::UnitTime::*;
use crate::time::*;
use crate::unit::Unit;

#[test]
fn time_compare_gt() {
    assert_eq!(Time::from(1, Second) > Time::new(), true);
}

#[test]
fn time_compare_lt() {
    assert_eq!(Time::new() < Time::from(1, Second), true);
}

#[test]
fn time_compare_eq() {
    assert_eq!(Time::from(1, Second) == Time::from(1, Second), true);
}

#[test]
fn schedule_not_active_before_time() {
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
fn schedule_active_in_time() {
    let mut schedule = Scheduler::new();
    let start = Time::from(1, Second);
    let end = Time::from(2, Second);
    let event = Event {
        start: start,
        end: end,
        id: 1,
    };
    schedule.push(event.clone());
    schedule.time.value += Second.value();
    assert_eq!(schedule.active_events().collect::<Vec<_>>(), vec![&event]);
}

#[test]
fn schedule_not_active_after_time() {
    let mut schedule = Scheduler::new();
    let start = Time::from(1, Second);
    let end = Time::from(2, Second);
    let event = Event {
        start: start,
        end: end,
        id: 1,
    };
    schedule.push(event.clone());
    schedule.time.value += Second.value() * 3;
    assert_eq!(
        schedule.active_events().collect::<Vec<_>>(),
        Vec::<&Event>::new()
    );
}

#[test]
fn events_by_id() {
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
fn active_events_by_id() {
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
    schedule.time.value += Second.value();
    assert_eq!(
        schedule.active_events_by_id(2).collect::<Vec<_>>(),
        vec![&event2]
    );
}
