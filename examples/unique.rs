use std::time::{Duration, Instant};

use retaker::{
    aoc::AOCStorage,
    scheduler::{Scheduler, System},
    world::{EntityStorage, World},
};

pub struct Timer {
    last_uptade: Instant,
    delay: Duration,
    elapsed: Duration,
}

fn create_timer(world: &mut World) {
    world.insert_unique_entity::<Timer>(Timer {
        last_uptade: Instant::now(),
        delay: Duration::from_secs(1),
        elapsed: Duration::ZERO,
    });
}

fn uptade_timer(world: &mut World) {
    let timer = world.mut_unique_entity::<Timer>().unwrap();
    if timer.elapsed >= Duration::from_secs(5) {
        world.exit();
        return;
    }
    if timer.last_uptade.elapsed() >= timer.delay {
        timer.last_uptade = Instant::now();
        timer.elapsed += timer.delay;
        println!("hiiii");
    }
}

pub fn exiting(_world: &mut World) {
    println!("exiting!");
}

fn main() {
    let world = World::new(EntityStorage::AOC(AOCStorage::new()));
    let mut scheduler = Scheduler::new(world);

    scheduler.add_systems([System::Start(create_timer), System::Uptade(uptade_timer)]);

    scheduler.run();
}
