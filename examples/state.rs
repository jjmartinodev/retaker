use std::time::{Duration, Instant};

use retaker::{
    aoc::AOCStorage,
    scheduler::{Scheduler, System},
    world::{EntityStorage, World},
};

pub struct Timer {
    last_uptade: Instant,
    delay: Duration,
}

fn create_timer(world: &mut World) {
    world.insert_unique_entity::<Timer>(Timer { last_uptade: Instant::now(), delay: Duration::from_secs(1) });
}

fn uptade_timer(world: &mut World) {
    let timer = world.mut_unique_entity::<Timer>().unwrap();
    if timer.last_uptade.elapsed() >= timer.delay {
        timer.last_uptade = Instant::now();
        println!("hiii");
    }
}

fn main() {
    let mut world = World::new(EntityStorage::AOC(AOCStorage::new()));
    let mut scheduler = Scheduler::new();

    scheduler.add_system(System::Start(create_timer));
    scheduler.add_system(System::Uptade(uptade_timer));

    scheduler.start(&mut world);
    loop {
        scheduler.uptade(&mut world);
    }
}
