// trait Status {
//     fn status_effects(&mut self) -> Vec<StatusEffect>;

//     fn remove_status(&mut self, name: String) {
//         for i, s in self.status_effects().iter().enumerate() {
//             ...
//         }
//     }
//     fn add_status(&mut self, name: StatusEffect) {}
//     fn update_statuses(&mut self) {}
// }

// struct StatusEffect {
//     name: String,
//     duration: u32
// }


// struct Creature {
//     name: String,
//     health: i32,
//     ac: u32,
//     status_effects: Vec<StatusEffect>
// }

// impl Status for Creature {
//     fn status_effects(&mut self) -> Vec<StatusEffect> {
//         return self.status_effects
//     }
// }

// impl Creature {
// }


// struct CombatState {
//     participants: Vec<Creature>,
//     round: u32,
//     status_effects: Vec<StatusEffect>
// }


mod time;
mod world;
mod count;

use world::World;
use time::UnitTime;



fn main() {
    let mut world = World::new();
    world.time.current.add(1992, UnitTime::YEAR);
    world.time.current.add(2, UnitTime::MONTH);
    world.time.current.add(1, UnitTime::DAY);
    world.time.current.add(2, UnitTime::HOUR);
    world.time.current.add(2, UnitTime::WEEK);
    println!("{:?}", world.time.distribute());
    println!("{:?}", world.time.current.value());
}
