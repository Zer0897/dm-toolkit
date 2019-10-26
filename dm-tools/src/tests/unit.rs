use crate::tests::unit::FooUnit::*;
use crate::unit::*;
use dm_tools_derive::Unit;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(
    FromPrimitive, ToPrimitive, Debug, Hash, Copy, Clone, PartialEq, Eq, Unit, Ord, PartialOrd,
)]
enum FooUnit {
    One = 1,
    Two = 2,
    Three = 3,
}

#[test]
fn distribute_from() {
    let mut value = 0;
    value += Two.value();
    value += One.value();

    let res = FooUnit::distribute_from(&[One, Two], value as usize);
    assert_eq!(res.get(&Two), Some(1 as i64).as_ref());
    assert_eq!(res.get(&One), Some(1 as i64).as_ref());
}

#[test]
fn distribute() {
    let mut value = 0;
    value += Two.value();
    value += One.value();

    let res = FooUnit::distribute(value as usize);
    assert_eq!(res.get(&Three), Some(1 as i64).as_ref());
}

#[test]
fn distribute_uses_minimum_steps() {
    #[derive(
        FromPrimitive, ToPrimitive, Hash, Debug, Copy, Clone, PartialEq, Eq, Unit, Ord, PartialOrd,
    )]
    enum Coins {
        One = 1,
        Three = 3,
        Four = 4,
    }

    let res = Coins::distribute(6);
    assert_eq!(res.get(&Coins::Three), Some(2 as i64).as_ref());
}

#[test]
fn distribute_from_one_unit() {
    let mut value = 0;
    value += Two.value() * 61;

    let res = FooUnit::distribute_from(&[One, Two], value as usize);
    assert_eq!(res.get(&Two), Some(61 as i64).as_ref());
}

#[test]
fn convert_value() {
    assert_eq!(One.value() * 100 / Two.value(), 50);
}

#[test]
fn counter_add() {
    let mut counter = UnitCounter::<FooUnit>::new();
    counter.add(1).unwrap();
    assert_eq!(counter.get_count(&FooUnit::One).unwrap(), 1);
}

#[test]
fn counter_sub() {
    let mut counter = UnitCounter::<FooUnit>::new();
    counter.sub(1).unwrap();
    assert_eq!(counter.get_count(&FooUnit::One).unwrap(), -1);
}

#[test]
fn counter_sub_equals_add_neg() {
    let mut counter = UnitCounter::<FooUnit>::new();
    counter.sub(1).unwrap();
    counter.add(-1).unwrap();
    assert_eq!(counter.get_count(&FooUnit::One).unwrap(), -2);
}

#[test]
fn counter_add_many() {
    let mut counter = UnitCounter::<FooUnit>::new();
    counter.add(7).unwrap();
    assert_eq!(counter.get_count(&FooUnit::One).unwrap(), 1);
    assert_eq!(counter.get_count(&FooUnit::Three).unwrap(), 2);
}

#[test]
fn counter_sub_many() {
    let mut counter = UnitCounter::<FooUnit>::new();
    counter.add(7).unwrap();
    assert_eq!(counter.get_count(&FooUnit::One).unwrap(), 1);
    assert_eq!(counter.get_count(&FooUnit::Three).unwrap(), 2);
}

#[test]
fn counter_set_count() {
    let mut counter = UnitCounter::<FooUnit>::new();
    counter.set_units(1, &FooUnit::One).unwrap();
    assert_eq!(counter.get_count(&FooUnit::One).unwrap(), 1);
}

#[test]
fn counter_distribute_give() {
    let mut counter = UnitCounter::<FooUnit>::new();
    counter.set_units(120, &FooUnit::One).unwrap();
    assert_eq!(counter.get_count(&FooUnit::One).unwrap(), 120);
    counter.redistribute().unwrap();
    assert_eq!(counter.get_count(&FooUnit::Three).unwrap(), 40);
}

#[test]
fn counter_distribute_steal() {
    let mut counter = UnitCounter::<FooUnit>::new();
    counter.set_units(-120, &FooUnit::One).unwrap();
    counter.set_units(40, &FooUnit::Three).unwrap();
    counter.redistribute().unwrap();
    assert_eq!(counter.get_count(&FooUnit::One).unwrap(), 0);
    assert_eq!(counter.get_count(&FooUnit::Two).unwrap(), 0);
    assert_eq!(counter.get_count(&FooUnit::Three).unwrap(), 0);
}

#[test]
fn counter_distribute_overflow() {
    let mut counter = UnitCounter::<FooUnit>::new();
    counter.set_units(2, &FooUnit::One).unwrap();
    counter.redistribute().unwrap();
    assert_eq!(counter.get_count(&FooUnit::One).unwrap(), 0);
    assert_eq!(counter.get_count(&FooUnit::Two).unwrap(), 1);
}

#[test]
fn counter_distribute_underflow() {
    let mut counter = UnitCounter::<FooUnit>::new();
    counter.set_units(1, &FooUnit::Two).unwrap();
    counter.sub_units(1, &FooUnit::One).unwrap();
    counter.redistribute().unwrap();
    assert_eq!(counter.get_count(&FooUnit::One).unwrap(), 1);
    assert_eq!(counter.get_count(&FooUnit::Two).unwrap(), 0);
}

#[test]
fn counter_distribute_underflow_with_existing() {
    let mut counter = UnitCounter::<FooUnit>::new();
    counter.set_units(1, &FooUnit::One).unwrap();
    counter.set_units(2, &FooUnit::Two).unwrap();
    counter.sub_units(3, &FooUnit::One).unwrap();
    counter.redistribute().unwrap();
    assert_eq!(counter.get_count(&FooUnit::One).unwrap(), 0);
    assert_eq!(counter.get_count(&FooUnit::Two).unwrap(), 1);
}

#[test]
fn counter_distribute_overflow_with_existing() {
    let mut counter = UnitCounter::<FooUnit>::new();
    counter.set_units(1, &FooUnit::One).unwrap();
    counter.set_units(2, &FooUnit::Two).unwrap();
    counter.add_units(3, &FooUnit::One).unwrap();
    counter.redistribute().unwrap();
    assert_eq!(counter.get_count(&FooUnit::One).unwrap(), 0);
    assert_eq!(counter.get_count(&FooUnit::Two).unwrap(), 1);
    assert_eq!(counter.get_count(&FooUnit::Three).unwrap(), 2);
}
