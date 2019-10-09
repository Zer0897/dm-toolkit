use dm_tools::count::Count;
use dm_tools::time::UnitTime;
use dm_tools::world::World;

fn main() {
    let mut world = World::new();
    world.time.add(1992, UnitTime::Year);
    world.time.add(2, UnitTime::Month);
    world.time.add(1, UnitTime::Day);
    world.time.add(2, UnitTime::Hour);
    world.time.add(2, UnitTime::Week);
    println!("{:?}", world.time.value());
}
